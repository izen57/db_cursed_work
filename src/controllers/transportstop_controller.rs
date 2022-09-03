pub mod transportstop_controller {
	use fltk::dialog::alert_default;
	use fltk_table::*;
	use chrono::NaiveDate;

	use crate::models::{ transportstop_model::*, client_model::* };

	pub unsafe fn prepare_row_del(root_number: String/*, and_stop: bool*/) {
		transportstop_model::remove_row(root_number.parse().unwrap()/*, and_stop*/);
	}

	pub unsafe fn prepare_row_crt(id: String, root_numbers: String, name: String, address: String, request_stop: String, install_year: String, electricity: String, rails: String) {
		if id.is_empty() {
			alert_default("Введите идентификатор остановки.");
			return;
		} else if !id.is_empty() && !root_numbers.is_empty() {
			transportstop_model::add_roots(id.parse().unwrap(), root_numbers);
		} else if !id.is_empty() && (
			name.is_empty() || address.is_empty() || request_stop.is_empty() ||
			install_year.is_empty() || electricity.is_empty() || rails.is_empty()
		) {
			alert_default("Заполните нужные поля.");
			return;
		}

		let res = NaiveDate::parse_from_str(&install_year, "%Y");
		let _resdate: NaiveDate;
		match res {
			Ok(success) => _resdate = success,
			Err(_) => {
				alert_default("Не удалось преобразовать год установки.");
				return
			}
		};
		transportstop_model::create_row(
			id.parse().unwrap(),
			name,
			address,
			request_stop.parse().unwrap(),
			install_year.parse().unwrap(),
			electricity.parse().unwrap(),
			rails.parse().unwrap()
		);
	}

	pub unsafe fn table() {
		let request = roles::U.get_valid().query("select * from transport_stop", &[]).unwrap_or_default();
		let row_count = request.len();

		let mut all_table = SmartTable::default()
			.with_pos(5, 5)
			.with_size(750, 590)
			.with_opts(TableOpts{
				rows: row_count as i32,
				cols: 7,
				editable: false,
				..Default::default()
			});
		all_table.set_col_header_value(0, "Ид-р остановки");
		all_table.set_col_header_value(1, "Название");
		all_table.set_col_header_value(2, "Адрес");
		all_table.set_col_header_value(3, "По требованию");
		all_table.set_col_header_value(4, "Год установки");
		all_table.set_col_header_value(5, "Контактный провод");
		all_table.set_col_header_value(6, "Рельсы");

		for (row_index, row) in request.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "text" {
					let value: &str = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value);
				} else if col_type == "bool" {
					let value: bool = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, if value { "есть" } else { "нет" });
				} else if col_type == "date" {
					let value: NaiveDate = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}
	}
}