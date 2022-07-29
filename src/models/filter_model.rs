pub mod filter_model {
	use chrono::NaiveDate;
	use fltk::dialog::{ alert, message, alert_default };
	use postgres::{ Error, Row };

	use crate::models::client::*;

	pub unsafe fn trns_query(root: (String, String), start_id: (String, String), stop_id: (String, String), trnstype: String, date: (String, String)) -> Vec<Row> {
		let mut query_string: String = format!("select * from transport where ").to_string();
		let mut flag = false;
		let (root_value, root_sign) = root;
		let (start_value, start_sign) = start_id;
		let (stop_value, stop_sign) = stop_id;
		let (date_value, date_sign) = date;

		if !root_value.is_empty() {
			query_string += &format!("root_number {root_sign} {root_value} ");
			flag = true;
		}
		if !start_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("start_id {start_sign} {start_value} ");
			flag = true;
		}
		if !stop_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("stop_id {stop_sign} {stop_value} ");
			flag = true;
		}
		if !trnstype.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("transport_type = {trnstype} ");
			flag = true;
		}
		if !date_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("entry_date {date_sign} {date_value} ");
			flag = true;
		}
		if !flag {
			query_string = "select * from transport".to_string();
		}

		roles::U.get_valid().query(&query_string, &[]).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			Vec::new()
		})
	}
}