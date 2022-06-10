pub mod fare_model {
	use fltk::prelude::WidgetExt;
	use postgres::{Error, Row};
	use fltk::dialog::{alert, message};

	use crate::models::client::*;
	use crate::controllers::fare_controller::*;

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
			Fare{price, root_number, start_id, stop_id, day_time}
		}

		pub fn get_price(&self) -> f64 {
			self.price
		}
	}

	unsafe fn get_checking_root(root_number: i32) -> Vec<Row> {
		roles::U.get_valid().query("select * from fare where root_number = $1", &[&root_number]).unwrap()
	}

	pub unsafe fn change_daytime(root_number: i32, new_daytime: String) {
		let checking = get_checking_root(root_number);
		let result: &Row;
		if checking.is_empty() {
			alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
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

		let result = roles::U.get_valid().execute("update fare set day_time = $1 where root_number = $2", &[&new_daytime, &root_number])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_start(root_number: i32, new_start: i32) {
		let checking = get_checking_root(root_number);
		let result: &Row;
		if checking.is_empty() {
			alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
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

		let result = roles::U.get_valid().execute("update fare set start_id = $1 where root_number = $2", &[&new_start, &root_number])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			});
			println!("{}", result);
	}

		pub unsafe fn change_stop(root_number: i32, new_stop: i32) {
		let checking = get_checking_root(root_number);
		let result: &Row;
		if checking.is_empty() {
			alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
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

		let result = roles::U.get_valid().execute("update fare set stop_id = $1 where root_number = $2", &[&new_stop, &root_number])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_price(root_number: i32, new_price: f64) {
		let checking = get_checking_root(root_number);
		let result: &Row;
		if checking.is_empty() {
			alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
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

		let mut result = roles::U.get_valid().execute("update fare set price = $1 where root_number = $2", &[&new_price, &root_number])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			});
		println!("{}", result);
		result = roles::U.get_valid().execute("update tr_fa set price = $1 where root_number = $2", &[&new_price, &root_number]).unwrap_or_else(|error| {
			alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
			0
		});
	}

	pub unsafe fn remove_row(price: f64) {
		println!("{}", roles::U.get_valid().execute("delete from tr_fa where price = $1", &[&price]).unwrap());
		println!("{}", roles::U.get_valid().execute("delete from fare where price = $1", &[&price]).unwrap());
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(price: f32, root_number: i32, start_id: i32, stop_id: i32, day_time: String) {
		let checking = get_checking_root(root_number);
		let result: &Row;
		if checking.is_empty() {
			alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		roles::U.get_valid().execute(
			"insert into fare values ($1, $2, $3, $4, $5)",
			&[&root_number, &price, &start_id, &stop_id, &day_time])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			}
		);

		roles::U.get_valid().execute(
			"insert into tr_fa values ($1, $2)",
			&[&root_number, &price])
			.unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			}
		);
		message(10, 10, "Запись добавлена!");
	}
}