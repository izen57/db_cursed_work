pub mod filter_controller {
	use fltk::dialog::{ alert, message, alert_default };
	use chrono::{NaiveDate, NaiveTime};
	use postgres::{ Error, Row };

	use crate::models::{ filter_model::*, client::* };

	pub fn prepare_trns_query(root: (String, String), start_id: (String, String), stop_id: (String, String), trnstype: String, date: (String, String)) -> Vec<Row> {
		let mut resdate: String = String::default();
		if !date.0.is_empty() {
			let answer = NaiveDate::parse_from_str(&date.0, "%Y-%m-%d");
			match answer {
				Ok(success) => resdate = success.to_string(),
				Err(_) => {
					alert_default("Не удалось преобразовать дату введения маршрута.");
					return Vec::new();
				}
			};
		}

		unsafe {
			filter_model::trns_query(
				(root.0, root.1),
				(start_id.0, start_id.1),
				(stop_id.0, stop_id.1),
				trnstype,
				(resdate, date.1)
			)
		}
	}

	pub fn prepare_fare_query(root: (String, String), start_id: (String, String), stop_id: (String, String), price: (String, String), daytime: String) -> Vec<Row> {
		unsafe {
			filter_model::fare_query(
				(root.0, root.1),
				(start_id.0, start_id.1),
				(stop_id.0, stop_id.1),
				(price.0, price.1),
				daytime
			)
		}
	}

	pub fn prepare_trst_query(id: (String, String), name: String, address: String, request: bool, install_year: (String, String), electricity: bool, rails: bool) -> Vec<Row> {
		unsafe {
			filter_model::trst_query(
				(id.0, id.1),
				name,
				address,
				request,
				(install_year.0, install_year.1),
				electricity,
				rails
			)
		}
	}

	pub fn prepare_tmt_query(root: (String, String), time: (String, String), stop_id: (String, String), price: (String, String), weekends: bool) -> Vec<Row> {
		let mut restime: String = String::default();
		if !time.0.is_empty() {
			let answer = NaiveTime::parse_from_str(&time.0, "%H:%M");
			match answer {
				Ok(success) => restime = success.to_string(),
				Err(_) => {
					alert_default("Не удалось преобразовать время прибытия маршрута.");
					return Vec::new();
				}
			};
		}

		unsafe {
			filter_model::tmt_query(
				(root.0, root.1),
				(restime, time.1),
				(stop_id.0, stop_id.1),
				(price.0, price.1),
				weekends
			)
		}
	}
}