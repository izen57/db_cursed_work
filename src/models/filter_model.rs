pub mod filter_model {
	use chrono::NaiveDate;
	use fltk::dialog::{ alert, message, alert_default };
	use postgres::{ Error, Row };

	use crate::models::client_model::*;

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

	pub unsafe fn fare_query(root: (String, String), start_id: (String, String), stop_id: (String, String), price: (String, String), daytime: String) -> Vec<Row> {
		let mut query_string: String = format!("select * from fare where ").to_string();
		let mut flag = false;
		let (root_value, root_sign) = root;
		let (start_value, start_sign) = start_id;
		let (stop_value, stop_sign) = stop_id;
		let (price_value, price_sign) = price;

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
		if !daytime.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("day_time = {daytime} ");
			flag = true;
		}
		if !price_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("price {price_sign} {price_value} ");
			flag = true;
		}
		if !flag {
			query_string = "select * from fare".to_string();
		}

		roles::U.get_valid().query(&query_string, &[]).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			Vec::new()
		})
	}

	pub unsafe fn trst_query(id: (String, String), name: String, address: String, request: bool, install_year: (String, String), electricity: bool, rails: bool) -> Vec<Row> {
		let mut query_string: String = format!("select * from transport_stop where ").to_string();
		let mut flag = false;
		let (id_value, id_sign) = id;
		let (year_value, year_sign) = install_year;

		if !id_value.is_empty() {
			query_string += &format!("id {id_sign} {id_value} ");
			flag = true;
		}
		if !name.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("name = {name} ");
			flag = true;
		}
		if !address.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("address = {address} ");
			flag = true;
		}
		if flag {
			query_string += "and ";
		}
		query_string += &format!("request_stop = {request} ");
		flag = true;
		if !year_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("install_year {year_sign} {year_value} ");
			flag = true;
		}
		if flag {
			query_string += "and ";
		}
		query_string += &format!("electricity = {electricity} ");
		flag = true;
		if flag {
			query_string += "and ";
		}
		query_string += &format!("rails = {rails} ");
		flag = true;
		if !flag {
			query_string = "select * from transport_stop".to_string();
		}

		roles::U.get_valid().query(&query_string, &[]).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			Vec::new()
		})
	}

	pub unsafe fn tmt_query(root: (String, String), time: (String, String), stop_id: (String, String), price: (String, String), weekends: bool) -> Vec<Row> {
		let mut query_string: String = format!("select * from timetable where ").to_string();
		let mut flag = false;
		let (root_value, root_sign) = root;
		let (time_value, time_sign) = time;
		let (stop_value, stop_sign) = stop_id;
		let (price_value, price_sign) = price;

		if !root_value.is_empty() {
			query_string += &format!("root {root_sign} {root_value} ");
			flag = true;
		}
		if !time_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("timing {time_sign} {time_value} ");
			flag = true;
		}
		if !stop_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("transport_stop_id {stop_sign} {stop_value} ");
			flag = true;
		}
		if flag {
			query_string += "and ";
		}
		query_string += &format!("weekends = {weekends} ");
		flag = true;
		if !price_value.is_empty() {
			if flag {
				query_string += "and ";
			}
			query_string += &format!("max_price {price_sign} {price_value} ");
			flag = true;
		}
		if !flag {
			query_string = "select * from timetable".to_string();
		}

		roles::U.get_valid().query(&query_string, &[]).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			Vec::new()
		})
	}
}