pub mod filter_view {
	use fltk::{
		button::Button,
		enums::{ Align, Color, FrameType },
		frame::Frame,
		group::Group,
		input::{ Input, IntInput, FloatInput },
		menu::Choice,
		prelude::*,
		window::Window
	};

	pub fn filter_window(w: &mut impl WidgetExt) {
		let mut filter_window = Window::default()
			.with_size(800, 600)
			.with_label("Полный список тарифов");

		let mut trns_frame = Frame::default()
			.with_pos(10, 10)
			.with_size(231, 301);
		trns_frame.set_frame(FrameType::UpBox);

		let nmb_lbl = Frame::default()
			.with_pos(35, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input = IntInput::default()
			.with_pos(20, 35)
			.with_size(60, 20);
		let mut nmb_choice = Choice::default()
			.with_pos(90, 35)
			.with_size(60, 20);
		nmb_choice.add_choice("= | < | > | ⩽ | ⩾");

		let start_lbl = Frame::default()
			.with_pos(57, 50)
			.with_size(80, 30)
			.with_label("ID начальной остановки:");
		let start_input = IntInput::default()
			.with_pos(20, 75)
			.with_size(60, 20);
		let mut start_choice = Choice::default()
			.with_pos(90, 75)
			.with_size(60, 20);
		start_choice.add_choice("= | < | > | ⩽ | ⩾");

		let stop_lbl = Frame::default()
			.with_pos(53, 90)
			.with_size(80, 30)
			.with_label("ID конечной остановки:");
		let stop_input = IntInput::default()
			.with_pos(20, 115)
			.with_size(60, 20);
		let mut stop_choice = Choice::default()
			.with_pos(90, 115)
			.with_size(60, 20);
		stop_choice.add_choice("= | < | > | ⩽ | ⩾");

		let type_lbl = Frame::default()
			.with_pos(30, 130)
			.with_size(80, 30)
			.with_label("Тип транспорта:");
		let type_input = Input::default()
			.with_pos(20, 155)
			.with_size(80, 20);

		let date_lbl = Frame::default()
			.with_pos(60, 163)
			.with_size(80, 60)
			.with_label("Дата введения маршрута\n(YYYY-MM-DD):");
		let date_input = Input::default()
			.with_pos(20, 213)
			.with_size(60, 20);
		let mut date_choice = Choice::default()
			.with_pos(90, 213)
			.with_size(60, 20);
		date_choice.add_choice("= | < | > | ⩽ | ⩾");

		let send_btn = Button::default()
			.with_pos(100, 275)
			.with_size(70, 30)
			.with_label("Отправить");

		filter_window.end();
		filter_window.show();
	}
}