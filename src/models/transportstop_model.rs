pub mod transportstop_model {
	use chrono::NaiveDate;
	use postgres::Row;
	use fltk::dialog::{ alert_default, message };

	use crate::models::client_model::*;

	// pub struct TransportStop {
	// 	id: i32,
	// 	name: String,
	// 	address: String,
	// 	request_stop: bool,
	// 	install_year: String,
	// 	electricity: bool,
	// 	rails: bool
	// }
	
	// impl TransportStop {
	// 	const fn new(id: i32, name: String, address: String, request_stop: bool, install_year: String, electricity: bool, rails: bool) -> TransportStop {
	// 		TransportStop{ id, name, address, request_stop, install_year, electricity, rails }
	// 	}
	// }

	// pub static mut TRS: TransportStop = TransportStop::new(
	// 	0,
	// 	String::new(),
	// 	String::new(),
	// 	false,
	// 	String::new(),
	// 	false,
	// 	false
	// );

	unsafe fn get_checking_root(id: i32) -> Vec<Row> {
		roles::U.get_valid().query("select * from transport_stop where id = $1", &[&id]).unwrap()
	}

	pub unsafe fn change_year(id: i32, new_year: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
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

		let _result = roles::U.get_valid().execute("update transport_stop set install_year = $1 where id = $2", &[&resdate, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", _result);
	}

	pub unsafe fn change_name(id: i32, new_name: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	_result.get("root_number"),
		// 	_result.get("start_id"),
		// 	_result.get("stop_id"),
		// 	_result.get("transport_type"),
		// 	_result.get("entry_date")
		// );

		let _result = roles::U.get_valid().execute("update transport_stop set name = $1 where id = $2", &[&new_name, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
			println!("{}", _result);
	}

	pub unsafe fn change_address(id: i32, new_address: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	_result.get("root_number"),
		// 	_result.get("start_id"),
		// 	_result.get("stop_id"),
		// 	_result.get("transport_type"),
		// 	_result.get("entry_date")
		// );

		let _result = roles::U.get_valid().execute("update transport_stop set address = $1 where id = $2", &[&new_address, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", _result);
	}

	pub unsafe fn change_elec(id: i32, new_el: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	_result.get("root_number"),
		// 	_result.get("start_id"),
		// 	_result.get("stop_id"),
		// 	_result.get("transport_type"),
		// 	_result.get("entry_date")
		// );

		let _result = roles::U.get_valid().execute("update transport_stop set electricity = $1 where id = $2", &[&new_el, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", _result);
	}

	pub unsafe fn change_rails(id: i32, new_rails: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	_result.get("root_number"),
		// 	_result.get("start_id"),
		// 	_result.get("stop_id"),
		// 	_result.get("transport_type"),
		// 	_result.get("entry_date")
		// );

		let _result = roles::U.get_valid().execute("update transport_stop set rails = $1 where id = $2", &[&new_rails, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", _result);
	}

	pub unsafe fn change_request(id: i32, new_request: String) {
		let checking = get_checking_root(id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		// TR = Transport::new(
		// 	_result.get("root_number"),
		// 	_result.get("start_id"),
		// 	_result.get("stop_id"),
		// 	_result.get("transport_type"),
		// 	_result.get("entry_date")
		// );

		let _result = roles::U.get_valid().execute("update transport_stop set request_stop = $1 where id = $2", &[&new_request, &id])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", _result);
	}

	pub unsafe fn remove_row(id: i32/*, and_stop: bool*/) {
		let mut transaction = roles::U.get_valid().transaction().unwrap();
		println!("{}", transaction.execute("update tr_trst set transport_stop_id = null where transport_stop_id = $1", &[&id]).unwrap());
		println!("{}", transaction.execute("delete from transport_stop where id = $1", &[&id]).unwrap());
		println!("{}", transaction.execute("delete from trst_ti where trst_id = $1", &[&id]).unwrap());
		// if and_stop {
		// 	println!("{}", roles::U.get_valid().execute("update transport set transport_stop_id = null where root_number = $1", &[&id]).unwrap());
		// }
		transaction.commit().unwrap();
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(id: i32, name: String, address: String, request_stop: bool, install_year: String, electricity: bool, rails: bool) {
		let checking = get_checking_root(id);
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", id));
			return;
		}

		let mut transaction = roles::U.get_valid().transaction().unwrap();
		transaction.execute(
			"insert into transport_stop values ($1, $2, $3, $4, $5, $6, $7)",
			&[&id, &name, &address, &request_stop, &install_year, &electricity, &rails]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		transaction.execute(
			"insert into trst_ti (trst_id) values ($1)",
			&[&id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку с параметрами из-за ошибки: {}", error));
			0
		});
		transaction.commit().unwrap();
		message(10, 10, "Запись добавлена!");
	}

	pub unsafe fn add_roots(root_number: i32, roots: String) {
		let v: Vec<&str> = roots.split(",").collect();
		if v.is_empty() {
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