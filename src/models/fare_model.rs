pub mod fare_model {
	use postgres::{Error, Row};
	use fltk::dialog::alert;
	use crate::models::client::*;

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
		// 	Self.price = price;
		// }

		// fn set_root_number(&mut self, root_number: i32) {
		// 	self.root_number = root_number;
		// }

		// fn set_start_id(&mut self, start_id: i32) {
		// 	self.start_id = start_id;
		// }

		// fn set_stop_id(&mut self, stop_id: i32) {
		// 	self.stop_id = stop_id;
		// }

		// fn set_day_time(&mut self, day_time: String) {
		// 	self.day_time = day_time;
		// }

		pub unsafe fn change_price(&mut self, root_number: i32, new_price: f64) {
			let checking = roles::U.get_valid().query_one("select * from fare where root_number = $1;", &[&root_number]).unwrap()/*
				.unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку из-за ошибки: {}", error));
					Row{statement: 0, body: 0, ranges: 0}
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

			let result = roles::U.get_valid().execute("update fare set price = $1 where root_number = $2;", &[&new_price, &root_number])
				.unwrap_or_else(|error| {
					alert(10, 10, &format!("Не удалось обновить строку с параметрами ({}, {}, {}, {}, {}) из-за ошибки: {}", self.price, self.root_number, self.start_id, self.stop_id, self.day_time, error));
					0
				});
			println!("{}", result);
		}
	}
}