pub mod filter_model {
	use chrono::NaiveDate;
	use fltk::dialog::{ alert, message, alert_default };
	use postgres::{ Error, Row };

	use crate::models::client::*;

	pub unsafe fn trns_query(root: (String, String), start_id: (String, String), stop_id: (String, String), trnstype: String, date: (String, String)) -> Vec<Row> {
		let query_string = "select * from transport";

		roles::U.get_valid().query(&format!(
			"select * from transport where
			root_number $0 $1 and
			start_id $2 $3 and
			stop_id $4 $5 and
			transport_type = $6 and
			entry_date $7 $8"
		), &[
			&root.1, &root.0, &start_id.1,
			&start_id.0, &stop_id.1, &stop_id.0,
			&trnstype, &date.1, &date.0
		]).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			Vec::new()
		})
	}
}