pub mod timetable_controller {
	use fltk::dialog::alert_default;
	use fltk_table::*;
	use chrono::NaiveTime;

	use crate::models::{ timetable_model::*, client_model::* };

	pub unsafe fn prepare_row_del(root: String, timing: String, stop_id: String) {
		timetable_model::remove_row(
			timing,
			root.parse().unwrap(),
			stop_id.parse().unwrap()
		);
	}

	pub unsafe fn prepare_row_crt(timing: String, root: String, max_price: String, transport_stop_id: String, weekends: String) {
		if timing.is_empty() {
			alert_default("Введите время прибытия в формате \"%H:%M:%S\".");
			return;
		}

		let res = NaiveTime::parse_from_str(&timing, "%H:%M:%S");
		let resdate: NaiveTime;
		match res {
			Ok(success) => resdate = success,
			Err(_) => {
				alert_default("Не удалось преобразовать дату ввода маршрута.");
				return
			}
		};
		timetable_model::create_row(
			resdate.to_string(),
			root.parse().unwrap(),
			max_price.parse().unwrap(),
			transport_stop_id.parse().unwrap(),
			weekends.parse().unwrap()
		);
	}

	pub unsafe fn table() {
		let request = roles::U.get_valid().query("select * from timetable", &[]).unwrap_or_default();
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
		all_table.set_col_header_value(0, "Время прибытия");
		all_table.set_col_header_value(1, "Ид-р остановки");
		all_table.set_col_header_value(2, "Номер маршрута");
		all_table.set_col_header_value(3, "Работа по выходным");
		all_table.set_col_header_value(4, "Максимальная цена за проезд");

		for (row_index, row) in request.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "bool" {
					let value: bool = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, if value { "есть" } else { "нет" });
				} else if col_type == "time" {
					let value: NaiveTime = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "float8" {
					let value: f64 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}
	}
}