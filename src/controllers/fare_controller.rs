pub mod fare_controller {
	use fltk::dialog::alert_default;
	use fltk_table::*;
	use crate::models::{ fare_model::*, client_model::* };

	fn convert_error() {
		alert_default("Вводимые данные некорректны!");
	}

	pub unsafe fn check_i32(str_value: String) -> i32 {
		let num_conv: i32 = str_value.parse().unwrap_or_else(|_| {
			convert_error();
			-1
		});
		num_conv
	}

	pub unsafe fn prepare_row_del(price: String, root: String) {
		fare_model::remove_row(price.parse().unwrap(), root.parse().unwrap());
	}

	pub unsafe fn prepare_row_crt(price: String, root_number: String, start_id: String, stop_id: String, day_time: String) {
		if price.is_empty() {
			alert_default("Заполните все поля.");
			return;
		}
		fare_model::create_row(
			price.parse().unwrap(),
			root_number.parse().unwrap(),
			start_id.parse().unwrap(),
			stop_id.parse().unwrap(),
			day_time
		);
	}

	pub unsafe fn table() {
		let request = roles::U.get_valid().query("select * from fare", &[]).unwrap_or_default();
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
		all_table.set_col_header_value(1, "Цена билета");
		all_table.set_col_header_value(2, "Ид-р начальной остановки");
		all_table.set_col_header_value(3, "Ид-р конечной остановки");
		all_table.set_col_header_value(4, "Время");

		for (row_index, row) in request.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "text" {
					let value: &str = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value);
				} else if col_type == "float8" {
					let value: f64 = row.get(col_index);
					all_table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}
	}
}