pub mod fare_model {
	use postgres::Row;
	use fltk::dialog::{ alert_default, message };

	use crate::models::client_model::*;

	pub struct Fare {
		price: f64,
		root_number: i32,
		start_id: i32,
		stop_id: i32,
		day_time: String
	}

	pub static mut F: Fare = Fare::new(0.0, 0, 0, 0, String::new());

	impl Fare {
		pub const fn new(price: f64, root_number: i32, start_id: i32, stop_id: i32, day_time: String) -> Fare {
			Fare{ price, root_number, start_id, stop_id, day_time }
		}

		pub fn get_price(&self) -> f64 {
			self.price
		}
	}

	unsafe fn get_checking_root(root_number: i32, price: f64) -> Vec<Row> {
		roles::U.get_valid().query("select * from fare where root_number = $1 and price = $2", &[&root_number, &price]).unwrap()
	}

	pub unsafe fn change_daytime(root_number: String, new_daytime: String) {
		let nmbprice: Vec<&str> = root_number.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let price: f64 = nmbprice[1].parse().unwrap();

		let checking = get_checking_root(root, price);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} и ценой {} не зарегистрован.", root_number, price));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		F = Fare::new(
			result.get("price"),
			result.get("root_number"),
			result.get("start_id"),
			result.get("stop_id"),
			result.get("day_time")
		);

		let result = roles::U.get_valid().execute("update fare set day_time = $1 where root_number = $2 and price = $3", &[&new_daytime, &root, &price]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});
		println!("{}", result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_start(root_number: String, new_start: i32) {
		let nmbprice: Vec<&str> = root_number.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let price: f64 = nmbprice[1].parse().unwrap();

		let checking = get_checking_root(root, price);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		F = Fare::new(
			result.get("price"),
			result.get("root_number"),
			result.get("start_id"),
			result.get("stop_id"),
			result.get("day_time")
		);

		let result = roles::U.get_valid().execute("update fare set start_id = $1 where root_number = $2 and price = $3", &[&new_start, &root, &price]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});
		println!("{}", result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_stop(root_number: String, new_stop: i32) {
		let nmbprice: Vec<&str> = root_number.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let price: f64 = nmbprice[1].parse().unwrap();

		let checking = get_checking_root(root, price);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		F = Fare::new(
			result.get("price"),
			result.get("root_number"),
			result.get("start_id"),
			result.get("stop_id"),
			result.get("day_time")
		);

		let result = roles::U.get_valid().execute("update fare set stop_id = $1 where root_number = $2 and price = $3", &[&new_stop, &root, &price]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});
		println!("{}", result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_price(root_number: String, new_price: f64) {
		let nmbprice: Vec<&str> = root_number.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let price: f64 = nmbprice[1].parse().unwrap();

		let checking = get_checking_root(root, price);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		F = Fare::new(
			result.get("price"),
			result.get("root_number"),
			result.get("start_id"),
			result.get("stop_id"),
			result.get("day_time")
		);

		let result = roles::U.get_valid().execute("update fare set price = $1 where root_number = $2 and price = $3", &[&new_price, &root, &price]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});
		println!("{}", result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn remove_row(price: f64, root_number: i32) {
		println!("{}", roles::U.get_valid().execute("delete from fare where price = $1 and root_number = $2", &[&price, &root_number]).unwrap());
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(price: f32, root_number: i32, start_id: i32, stop_id: i32, day_time: String) {
		// let nmbprice: Vec<&str> = root_number.split_terminator(",").collect();
		// let root: i32 = nmbprice[0].parse().unwrap();
		// let price: f64 = nmbprice[1].parse().unwrap();

		// let checking = get_checking_root(root, price);
		// let result: &Row;
		// if checking.is_empty() {
		// 	alert_default(&format!("Маршрут с номером {} не зарегистрован.", root_number));
		// 	return;
		// } else {
		// 	result = checking.get_unchecked(0);
		// }

		roles::U.get_valid().execute(
			"insert into fare values ($1, $2, $3, $4, $5)",
			&[&root_number, &price, &start_id, &stop_id, &day_time]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось вставить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});

		message(10, 10, "Запись добавлена!");
	}
}