pub mod timetable_model {
	use chrono::{ Date, NaiveTime, prelude::*, Utc };
	use postgres::{ Error, Row };
	use fltk::dialog::{ alert_default, message };

	use crate::models::client_model::*;

	pub struct Timetable {
		timing: String,
		root: i32,
		max_price: f64,
		transport_stop_id: i32,
		weekends: bool
	}
	
	impl Timetable {
		const fn new(timing: String, root: i32, max_price: f64, transport_stop_id: i32, weekends: bool) -> Timetable {
			Timetable{timing, root, max_price, transport_stop_id, weekends}
		}
	}

	pub static mut T: Timetable = Timetable::new(String::new(), 0, 0.0, 0, false);

	unsafe fn get_checking_timing(timing: &String) -> Vec<Row> {
		roles::U.get_valid().query("select * from timetable where timing = $1", &[&timing]).unwrap()
	}

	pub unsafe fn change_timing(timing: String, new_timing: String) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let mut transaction = roles::U.get_valid().transaction().unwrap();
		let mut result = transaction.execute("update timetable set timing = $1 where timing = $2", &[&new_timing, &timing])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
		result = transaction.execute("update trst_ti set timing = $1 where timing = $2", &[&new_timing, &timing])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		transaction.commit().unwrap();
	}

	pub unsafe fn change_wknd(timing: String, new_wknd: String) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let result = roles::U.get_valid().execute("update timetable set weekends = $1 where timing = $2", &[&new_wknd, &timing])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_root(timing: String, new_root: i32) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let result = roles::U.get_valid().execute("update timetable set root = $1 where timing = $2", &[&timing, &new_root])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_stop(timing: String, new_stop: String) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let result = roles::U.get_valid().execute("update timetable set transport_stop_id = $1 where timing = $2", &[&new_stop, &timing])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn change_price(timing: String, new_price: f64) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Маршрут с номером {} не зарегистрован.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let mut result = roles::U.get_valid().execute("update timetable set max_price = $1 where timing = $2", &[&new_price, &timing])
			.unwrap_or_else(|error| {
				alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
				0
			});
		println!("{}", result);
	}

	pub unsafe fn remove_row(timing: String) {
		let mut transaction = roles::U.get_valid().transaction().unwrap();
		println!("{}", transaction.execute("update trst_ti set timimng = null where timing = $1", &[&timing]).unwrap());
		println!("{}", transaction.execute("delete from timetable where timing = $1", &[&timing]).unwrap());
		transaction.commit().unwrap();
		message(10, 10, "Запись удалена!");
	}

	pub unsafe fn create_row(timing: String, root: i32, max_price: f64, transport_stop_id: i32, weekends: bool) {
		let checking = get_checking_timing(&timing);
		let result: &Row;
		if checking.is_empty() {
			alert_default(&format!("Время прибытия {} не зарегистровано.", timing));
			return;
		} else {
			result = checking.get_unchecked(0);
		}

		let mut transaction = roles::U.get_valid().transaction().unwrap();
		transaction.execute(
			"insert into timetable values ($1, $2, $3, $4, $5)",
			&[&timing, &root, &max_price, &transport_stop_id, &weekends]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});

		transaction.execute(
			"insert into trst_ti values ($1, $2) on conflict do nothing",
			&[&transport_stop_id, &timing]
		).unwrap_or_else(|error| {
			alert_default(&format!("Не удалось обновить строку из-за ошибки: {}", error));
			0
		});
		message(10, 10, "Запись добавлена!");
	}
}