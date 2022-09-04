pub mod transport_controller {
	use fltk::dialog::alert_default;
	use fltk_table::*;
	use chrono::NaiveDate;

	use crate::models::{ transport_model::*, client_model::* };

	pub unsafe fn prepare_row_del(root_number: String) {
		transport_model::remove_row(root_number.parse().unwrap());
	}

	pub unsafe fn get_transport_types() -> String {
		let types: String = roles::U.get_valid()
			.query_one("select enum_range(NULL::trtype)::text", &[])
			.unwrap_unchecked()
			.get(0);
		types.replace("}", "")
			.replace("{", "")
			.replace(",", "|")
	}

	pub unsafe fn prepare_row_crt(root_number: String, start_id: String, stop_id: String, transport_type: String, entry_date: String, stoplist: String) {
		if root_number.is_empty() {
			alert_default("Введите номер маршрута.");
			return;
		} else if !root_number.is_empty() && !stoplist.is_empty() {
			transport_model::add_stops(root_number.parse().unwrap(), stoplist);
		} else if !root_number.is_empty() && (start_id.is_empty() || stop_id.is_empty() || transport_type.is_empty() || entry_date.is_empty()) {
			alert_default("Заполните нужные поля.");
			return;
		}

		let res = NaiveDate::parse_from_str(&entry_date, "%Y-%m-%d");
		let resdate: NaiveDate;
		match res {
			Ok(success) => resdate = success,
			Err(_) => {
				alert_default("Не удалось преобразовать дату ввода маршрута.");
				return
			}
		};
		transport_model::create_row(
			root_number.parse().unwrap(),
			start_id.parse().unwrap(),
			stop_id.parse().unwrap(),
			transport_type,
			resdate
		);
	}

	pub unsafe fn table() {
		let request = roles::U.get_valid().query("select * from transport", &[]).unwrap_or_default();
		let row_count = request.len();

		let mut all_table = SmartTable::default()
			.with_pos(5, 5)
			.with_size(750, 590)
			.with_opts(TableOpts{
				rows: row_count as i32,
				cols: 5,
				editable: false,
				..Default::default()
			});
		all_table.set_col_header_value(0, "Номер маршрута");
		all_table.set_col_header_value(1, "Ид-р начальной остановки");
		all_table.set_col_header_value(2, "Ид-р конечной остановки");
		all_table.set_col_header_value(3, "Тип");
		all_table.set_col_header_value(4, "Время ввода");

		for (row_index, row) in request.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "trtype" {
					let value: transport_model::TrType = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "date" {
					let value: NaiveDate = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}
	}
}