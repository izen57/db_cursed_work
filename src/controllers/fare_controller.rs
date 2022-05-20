pub mod fare_controller {
	use fltk_table::*;
	use postgres::{Error, Row};
	use crate::models::client::*;

	pub unsafe fn table() -> Result<(), Error> {
		let request = roles::U.get_valid().query("select * from fare;", &[])?;
		let row_count = request.len();

		// for row in request {
		// 	let start_id: i32 = row.get("start_id");
		// 	println!("{}", start_id);
		// }

		let mut all_table = SmartTable::default()
			.with_pos(10, 10)
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
		Ok(())
	}
}