pub mod fare_controller {
	use fltk_table::{SmartTable, TableOpts};
	use postgres::{Error, Row};
	use crate::models::client::*;

	pub fn table() -> Result<(), Error> {
		unsafe {
			let request = roles::U.get_valid().query("select * from fare", &[])?;
			let row_count = request.len();

			for row in request {
				let start_id: i32 = row.get("start_id");
				println!("{}", start_id);
			}

			let mut table = SmartTable::default()
				.with_size(790, 590)
				.center_of_parent()
				.with_opts(TableOpts{
					rows: row_count as i32,
					editable: true,
					..Default::default()
				});
		}
		Ok(())
	}
}