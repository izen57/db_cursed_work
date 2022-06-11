pub mod transportstop_model {
	use chrono::{ Date, NaiveDate, prelude::*, Utc };
	use postgres::{ Error, Row };
	use fltk::dialog::{ alert_default, message };

	use crate::models::client::*;

	pub struct TransportStop {
		id: i32,
		name: String,
		address: String,
		request_stop: bool,
		install_year: String,
		electricity: bool,
		rails: bool
	}
	
	impl TransportStop {
		const fn new(id: i32, name: String, address: String, request_stop: bool, install_year: String, electricity: bool, rails: bool) -> TransportStop {
			TransportStop{ id, name, address, request_stop, install_year, electricity, rails }
		}
	}

	pub static mut TRS: TransportStop = TransportStop::new(
		0,
		String::new(),
		String::new(),
		false,
		String::new(),
		false,
		false
	);

	unsafe fn get_checking_root(id: i32) -> Vec<Row> {
		roles::U.get_valid().query("select * from transport_stop where id = $1", &[&id]).unwrap()
	}

	pub unsafe fn change_year(id: i32, new_year: String) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let res = NaiveDate::parse_from_str(&new_year, "%Y-%m-%d");
		let resdate: NaiveDate;
		match res {
			Ok(success) => resdate = success,
			Err(_) => {
				alert_default("Не удалось преобразовать дату ввода маршрута.");
				return
			}
		};

		let result = roles::U.get_valid().execute("update transport_stop set install_year = $1 where id = $2", &[&resdate, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_name(id: i32, new_name: String) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport_stop set name = $1 where id = $2", &[&new_name, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
			println!("{}", result);
	}

	pub unsafe fn change_address(id: i32, new_address: String) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport_stop set address = $1 where id = $2", &[&new_address, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_elec(id: i32, new_el: String) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport_stop set electricity = $1 where id = $2", &[&new_el, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_rails(id: i32, new_rails: String) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport_stop set rails = $1 where id = $2", &[&new_rails, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_request(id: i32, new_request: bool) {
		let checking = get_checking_root(id);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	result.get("root_number"),
		// 	result.get("start_id"),
		// 	result.get("stop_id"),
		// 	result.get("transport_type"),
		// 	result.get("entry_date")
		// );

		let result = roles::U.get_valid().execute("update transport_stop set request_stop = $1 where id = $2", &[&new_request, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn remove_row(id: i32, and_stop: bool) {
		println!("{}", roles::U.get_valid().execute("delete from tr_trst where root_number = $1", &[&id]).unwrap());
		println!("{}", roles::U.get_valid().execute("delete from transport where root_number = $1", &[&id]).unwrap());
		if and_stop {
			println!("{}", roles::U.get_valid().execute("update transport set transport_stop_id = null where root_number = $1", &[&id]).unwrap());
		}
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(id: i32, name: String, address: String, request_stop: bool, install_year: String, electricity: bool, rails: bool) {
		let checking = get_checking_root(id);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		}

		roles::U.get_valid().execute(
			"insert into transport values ($1, $2, $3, $4, $5, $6, $7)",
			&[&id, &name, &address, &request_stop, &install_year, &electricity, &rails]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		// roles::U.get_valid().execute(
		// 	"insert into trst_ti values ($1, $2)",
		// 	&[&root_number, &timing]
		// ).unwrap_or_else(|error| {
		// 	alert_default(&format!("Не удалось обновить строку с параметрами из-за ошибки: {}", error));
		// 	0
		// });

		message(10, 10, "Запись добавлена!");
	}

	pub unsafe fn add_roots(root_number: i32, roots: String) {
		let v: Vec<&str> = roots.split(",").collect();
		if v.is_empty() || v.len() != 2 {
			alert_default(&format!("Не удалось разделить входящую строку"));
			return;
		}

		for elem in v {
			roles::U.get_valid().execute(
				"insert into tr_trst values ($1, $2) on conflict do nothing",
				&[&root_number, &elem]
			).unwrap_or_else(|error| {
				alert_default(&format!("Не удалось разделить входящую строку"));
				0
			});
		}
	}
}