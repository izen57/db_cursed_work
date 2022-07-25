pub mod filter_controller {
	use fltk::dialog::{ alert, message, alert_default };
	use chrono::NaiveDate;
	use postgres::{ Error, Row };

	use crate::models::{ filter_model::*, client::* };

	pub fn prepare_trns_query(root: (String, String), start_id: (String, String), stop_id: (String, String), trnstype: String, date: (String, String)) -> Vec<Row> {
		if root.0.is_empty() && start_id.0.is_empty() && stop_id.0.is_empty()
		&& stop_id.0.is_empty() && trnstype.is_empty() && date.0.is_empty() {
			alert_default("Заполните хотя бы одно поле.");
			return Vec::new();
		}

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
}