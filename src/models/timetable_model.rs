pub mod timetable_model {
	use postgres::Row;
	use fltk::dialog::{ alert_default, message };

	use crate::models::client_model::*;
	use crate::models::transport_model::transport_model;

	// pub struct Timetable {
	// 	timing: String,
	// 	root: i32,
	// 	max_price: f64,
	// 	transport_stop_id: i32,
	// 	weekends: bool
	// }
	
	// impl Timetable {
	// 	const fn new(timing: String, root: i32, max_price: f64, transport_stop_id: i32, weekends: bool) -> Timetable {
	// 		Timetable{timing, root, max_price, transport_stop_id, weekends}
	// 	}
	// }

	// pub static mut T: Timetable = Timetable::new(String::new(), 0, 0.0, 0, false);

	unsafe fn get_checking_timing(root: i32, timing: &String, stop_id: i32) -> Vec<Row> {
		roles::U.get_valid().query("select * from timetable where root = $1 and timing = $2 and transport_stop_id = $3", &[&root, &timing, &stop_id]).unwrap()
	}

	pub unsafe fn change_timing(timing: String, new_timing: String) {
		let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let time: String = nmbprice[1].parse().unwrap();
		let stop_id: i32 = nmbprice[2].parse().unwrap();

		let checking = get_checking_timing(root, &time, stop_id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		let _result = roles::U.get_valid().execute("update timetable set timing = $1 where root = $2 and timing = $3 and transport_stop_id = $4", &[&new_timing, &timing]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		println!("{}", _result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_wknd(timing: String, new_wknd: String) {
		let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let time: String = nmbprice[1].parse().unwrap();
		let stop_id: i32 = nmbprice[2].parse().unwrap();

		let checking = get_checking_timing(root, &time, stop_id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		let _result = roles::U.get_valid().execute("update timetable set weekends = $1 where root = $2 and timing = $3 and transport_stop_id = $4", &[&new_wknd, &root, &timing, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		println!("{}", _result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_root(timing: String, new_root: i32) {
		let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let time: String = nmbprice[1].parse().unwrap();
		let stop_id: i32 = nmbprice[2].parse().unwrap();

		let checking = get_checking_timing(root, &time, stop_id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		let _result = roles::U.get_valid().execute("update timetable set root = $1 where root = $2 and timing = $3 and transport_stop_id = $4", &[&new_root, &root, &time, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		println!("{}", _result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_stop(timing: String, new_stop: String) {
		let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let time: String = nmbprice[1].parse().unwrap();
		let stop_id: i32 = nmbprice[2].parse().unwrap();

		let checking = get_checking_timing(root, &time, stop_id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		let _result = roles::U.get_valid().execute("update timetable set transport_stop_id = $1 where root = $2 and timing = $3 and transport_stop_id = $4", &[&new_stop, &root, &time, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		println!("{}", _result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn change_price(timing: String, new_price: f64) {
		let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		let root: i32 = nmbprice[0].parse().unwrap();
		let time: String = nmbprice[1].parse().unwrap();
		let stop_id: i32 = nmbprice[2].parse().unwrap();

		let checking = get_checking_timing(root, &time, stop_id);
		let _result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", timing));
			return;
		} else {
			_result = checking.get_unchecked(0);
		}

		let mut _result = roles::U.get_valid().execute("update timetable set max_price = $1 where root = $2 and timing = $3 and transport_stop_id = $4", &[&new_price, &root, &time, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		println!("{}", _result);
		message(10, 10, "Запись обновлена!");
	}

	pub unsafe fn remove_row(time: String, root: i32, stop_id: i32) {
		println!("{}", roles::U.get_valid().execute("delete from timetable where root = $1 and timing = $2 and transport_stop_id = $3", &[&root, &time, &stop_id]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		}));
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(timing: String, root: i32, max_price: f64, transport_stop_id: i32, weekends: bool) {
		// let nmbprice: Vec<&str> = timing.split_terminator(",").collect();
		// let root: i32 = nmbprice[0].parse().unwrap();
		// let time: String = nmbprice[1].parse().unwrap();
		// let stop_id: i32 = nmbprice[2].parse().unwrap();

		// let checking = transport_model::get_checking_root(root);
		// let _result: &Row;
		// if checking.is_empty() {
		// 	alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
		// 	return;
		// } else {
		// 	_result = checking.get_unchecked(0);
		// }

		roles::U.get_valid().execute(
			"insert into timetable values ($1, $2, $3, $4, $5)",
			&[&timing, &root, &max_price, &transport_stop_id, &weekends]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось вставить строку из-за ошибки: {}", error));
			0
		});

		message(10, 10, "Запись добавлена!");
	}
}