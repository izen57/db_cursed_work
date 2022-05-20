pub mod fare_view {
	use std::error::Error;

use fltk::{
		button::Button,
		dialog::alert,
		frame::Frame,
		input::Input,
		prelude::*,
		window::Window
	};
	use crate::{controllers::fare_controller::*, models::fare_model::*};

	fn convert_error() {
		alert(10, 10, &format!("Вводимые данные некорректны!"));
	}

	pub fn fare_window(w: &mut impl WidgetExt) {
		let mut fare_window = Window::default()
			.with_size(1500,600)
			//.center_screen()
			.with_label("Полный список тарифов");

		unsafe {
			let _ = fare_controller::table();
		}
		let ins_nmb_lbl = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let insert_id = Input::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let ins_price_lbl = Frame::default()
			.with_pos(980, 10)
			.with_size(60, 30)
			.with_label("Цена за билет:");
		let insert_price = Input::default()
			.with_pos(1070, 10)
			.with_size(60, 30);
		let mut price_for_change = Button::default()
			.with_pos(1140, 10)
			.with_size(180, 30)
			.with_label("Сменить цену билета");
		price_for_change.set_callback(move |_| {
			// let mut f: fare_model::Fare = fare_model::Fare::new(22.3, 43, 221, 7, "всё".to_string());
			let num_conv: i32 = insert_id.value().parse().unwrap_or_else(|_| {
				convert_error();
				-1
			});
			let price_conv: f64 = insert_price.value().parse().unwrap_or_else(|_| {
				convert_error();
				-1.0
			});
			
			unsafe {
				fare_model::Fare::change_price(num_conv, price_conv);
			}
		});
		//insert_btn.set_callback(&fare_view::fare_window);

		//println!("{:?}", fare_window.parent());
		fare_window.end();
		fare_window.show();
	}
}