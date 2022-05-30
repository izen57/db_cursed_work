pub mod fare_view {
	use fltk::{
		button::Button,
		frame::Frame,
		input::{Input, IntInput},
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
		let nmb_lbl = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input = IntInput::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let daytime_lbl = Frame::default()
			.with_pos(990, 10)
			.with_size(60, 30)
			.with_label("Новое время суток:");
		let daytime_input = Input::default()
			.with_pos(1090, 10)
			.with_size(60, 30);
		let mut daytime_btn = Button::default()
			.with_pos(1160, 10)
			.with_size(180, 30)
			.with_label("Сменить время суток");
		let start_lbl = Frame::default()
			.with_pos(1000, 45)
			.with_size(90, 30)
			.with_label("Новый ид-р нач. остановки:");
		let start_input = IntInput::default()
			.with_pos(1145, 45)
			.with_size(60, 30);
		let mut start_btn = Button::default()
			.with_pos(1215, 45)
			.with_size(180, 30)
			.with_label("Сменить нач. остановку");

		let root = nmb_input.value().parse().unwrap();
		unsafe {
			daytime_btn.set_callback(move |_|
				fare_model::Fare::change_daytime(root, daytime_input.value())
			);

			start_btn.set_callback(move |_|
				fare_model::Fare::change_start(root, start_input.value().parse().unwrap())
			);
		}

		fare_window.end();
		fare_window.redraw();
		fare_window.show();
	}
}