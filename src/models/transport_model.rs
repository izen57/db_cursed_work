pub mod transport_model {
	use chrono::{ Date, NaiveDate, prelude::*, Utc };
	use postgres::{ Error, Row };
	use fltk::dialog::{ alert_default, message };

	use crate::models::client::*;

	pub struct Transport {
		root_number: i32,
		start_id: i32,
		stop_id: i32,
		transport_type: String,
		entry_date: String
	}

	impl Transport {
		const fn new(root_number: i32, start_id: i32, stop_id: i32, transport_type: String, entry_date: String) -> Transport {
			Transport{ root_number, start_id, stop_id, transport_type, entry_date }
		}
	}

	pub static mut TR: Transport = Transport::new(0, 0, 0, String::new(), String::new());

	unsafe fn get_checking_root(root_number: i32) -> Vec<Row> {
		roles::U.get_valid().query("select * from transport where root_number = $1", &[&root_number]).unwrap()
	}

	pub unsafe fn change_date(root_number: i32, new_date: String) {
		let checking = get_checking_root(root_number);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		}

		let res = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d");
		let resdate: NaiveDate;
		match res {
			Ok(success) => resdate = success,
			Err(_) => {
				alert_default("Не удалось преобразовать дату ввода маршрута.");
				return
			}
		};

		// let str_entry_date: String = result.get::<'static, &str, String>("entry_date").to_string();

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get::<'static, &str, String>("entry_date").to_string()
		// );

		let result = roles::U.get_valid().execute("update transport set entry_date = $1 where root_number = $2", &[&resdate, &root_number])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_start(root_number: i32, new_start: i32) {
		let checking = get_checking_root(root_number);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport set start_id = $1 where root_number = $2", &[&new_start, &root_number])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
			println!("{}", result);
	}

	pub unsafe fn change_stop(root_number: i32, new_stop: i32) {
		let checking = get_checking_root(root_number);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport set stop_id = $1 where root_number = $2", &[&new_stop, &root_number])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_type(root_number: i32, new_type: String) {
		let checking = get_checking_root(root_number);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport set transport_type = $1 where root_number = $2", &[&new_type, &root_number])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn remove_row(root_number: i32) {
		println!("{}", roles::U.get_valid().execute("delete from tr_trst where root_number = $1", &[&root_number]).unwrap());
		println!("{}", roles::U.get_valid().execute("delete from transport where root_number = $1", &[&root_number]).unwrap());
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(root_number: i32, price: f32, start_id: i32, stop_id: i32, transport_type: String, day_time: NaiveDate) {
		let checking = get_checking_root(root_number);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		}

		roles::U.get_valid().execute(
			"insert into transport values ($1, $2, $3, $4, $5)",
			&[&root_number, &start_id, &stop_id, &transport_type, &day_time]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		roles::U.get_valid().execute(
			"insert into tr_trst values ($1, $2)",
			&[&root_number, &start_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		roles::U.get_valid().execute(
			"insert into tr_trst values ($1, $2)",
			&[&root_number, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		roles::U.get_valid().execute(
			"insert into tr_fa values ($1, $2)",
			&[&root_number, &price]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});

		message(10, 10, "Запись добавлена!");
	}

	pub unsafe fn add_stops(root_number: i32, stoplist: String) {
		let v: Vec<&str> = stoplist.split(",").collect();
		if v.is_empty() || v.len() != 2 {
			alert_default(&format!("Не удалось разделить входящую строку"));
			return;
		}

		for elem in v {
			roles::U.get_valid().execute(
				"insert into tr_trst values ($1, $2) on conflict do nothing",
				&[&root_number, &elem]
			).unwrap_or_else(|_| {
				alert_default(&format!("Не удалось разделить входящую строку"));
				0
			});
		}
	}
}