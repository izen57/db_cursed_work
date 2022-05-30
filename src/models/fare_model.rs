pub mod fare_model {
	use fltk::prelude::WidgetExt;
	use postgres::{Error, Row};
	use fltk::dialog::alert;
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

		// fn set_price(price: f64) {
		// 	F.price = price;
		// }

		// fn set_root_number(&mut F, root_number: i32) {
		// 	F.root_number = root_number;
		// }

		// fn set_start_id(&mut F, start_id: i32) {
		// 	F.start_id = start_id;
		// }

		// fn set_stop_id(&mut F, stop_id: i32) {
		// 	F.stop_id = stop_id;
		// }

		// fn set_day_time(&mut F, day_time: String) {
		// 	F.day_time = day_time;
		// }

		unsafe fn get_checking_root(root_number: i32) -> Row {
			roles::U.get_valid().query_one("select * from fare where root_number = $1;", &[&root_number]).unwrap()
		}

		pub unsafe fn change_daytime(root_number: i32, new_daytime: String) {
			let checking = Fare::get_checking_root(root_number);
			if checking.is_empty() {
				alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
				return;
			}
			F = Fare::new(
				checking.get("price"),
				checking.get("root_number"),
				checking.get("start_id"),
				checking.get("stop_id"),
				checking.get("day_time")
			);

			let result = roles::U.get_valid().execute("update fare set day_time = $1 where root_number = $2;", &[&new_daytime, &root_number])
				.unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
					0
				});
			println!("{}", result);
		}

		pub unsafe fn change_start(root_number: i32, new_start: i32) {
			let checking = Fare::get_checking_root(root_number);
			if checking.is_empty() {
				alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
				return;
			}
			F = Fare::new(
				checking.get("price"),
				checking.get("root_number"),
				checking.get("start_id"),
				checking.get("stop_id"),
				checking.get("day_time")
			);

			let result = roles::U.get_valid().execute("update fare set start_id = $1 where root_number = $2;", &[&new_start, &root_number])
				.unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
					0
				});
			println!("{}", result);
		}

		pub unsafe fn change_price(root_number: i32, new_price: f64) {
			let checking = roles::U.get_valid().query_one("select * from fare where root_number = $1;", &[&root_number]).unwrap()/* .unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку из-за ошибки: {}", error));
					Row{statement: Statement, body: 0, ranges: 0}
			})*/;
			if checking.is_empty() {
				alert(10, 10, &format!("Маршрут с номером {} не зарегистрован.", root_number));
				return
			}
			F = Fare::new(
				checking.get("price"),
				checking.get("root_number"),
				checking.get("start_id"),
				checking.get("stop_id"),
				checking.get("day_time")
			);

			let mut result = roles::U.get_valid().execute("update tr_fa set price = $1 where root_number = $2;", &[&new_price, &root_number])
				.unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
					0
				});
			println!("{}", result);
			result = roles::U.get_valid().execute("update fare set price = $1 where root_number = $2;", &[&new_price, &root_number]).unwrap_or_else(|error| {
				alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", F.price, F.root_number, F.start_id, F.stop_id, F.day_time, error));
				0
			});

			fare_controller::table();
		}
	}
}