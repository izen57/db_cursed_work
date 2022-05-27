pub mod fare_view {
	use std::error::Error;

	use fltk::{
		button::Button,
		frame::Frame,
		input::Input,
		prelude::*,
		window::Window
	};
	use crate::{controllers::fare_controller::*, models::fare_model::*};

	pub fn fare_window(w: &mut impl WidgetExt) {
		let mut fare_window = Window::default()
			.with_size(1500,600)
			//.center_screen()
			.with_label("Полный список тарифов");

		unsafe {
			let table = fare_controller::table();
		}
		let ins_nmb_lbl = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let insert_nmb = Input::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let ins_daytime_lbl = Frame::default()
			.with_pos(980, 10)
			.with_size(60, 30)
			.with_label("Новое время суток:");
		let insert_daytime = Input::default()
			.with_pos(1070, 10)
			.with_size(60, 30);
		let mut daytime_for_change = Button::default()
			.with_pos(1140, 10)
			.with_size(180, 30)
			.with_label("Сменить время суток");
		daytime_for_change.set_callback(move |table|
			unsafe {
				let root = fare_controller::check_root(insert_nmb.value());
				fare_model::Fare::change_daytime(root, insert_daytime.value())
			}
		);

		w.redraw();
		fare_window.end();
		fare_window.show();
	}
}