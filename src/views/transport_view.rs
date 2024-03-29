pub mod transport_view {
	use fltk::{
		button::Button,
		frame::Frame,
		input::{ Input, IntInput },
		prelude::*,
		window::Window, menu::Choice
	};
	use crate::{ controllers::transport_controller::{*, transport_controller::get_transport_types}, models::transport_model::* };

	pub fn transport_window(_w: &mut impl WidgetExt) {
		let mut transport_window = Window::default()
			.with_size(1500, 600)
			.with_label("Полный список транспорта");

		let _nmb_lbl1 = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input1 = IntInput::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let _type_lbl = Frame::default()
			.with_pos(990, 10)
			.with_size(60, 30)
			.with_label("Новый тип транспорта:");
		let type_input = Input::default()
			.with_pos(1145, 10)
			.with_size(60, 30);
		let mut type_btn = Button::default()
			.with_pos(1215, 10)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl2 = Frame::default()
			.with_pos(790, 45)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input2 = IntInput::default()
			.with_pos(890, 45)
			.with_size(60, 30);
		let _start_lbl = Frame::default()
			.with_pos(1000, 45)
			.with_size(90, 30)
			.with_label("Новый ид-р нач. остановки:");
		let start_input = IntInput::default()
			.with_pos(1145, 45)
			.with_size(60, 30);
		let mut start_btn = Button::default()
			.with_pos(1215, 45)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl3 = Frame::default()
			.with_pos(790, 80)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input3 = IntInput::default()
			.with_pos(890, 80)
			.with_size(60, 30);
		let _stop_lbl = Frame::default()
			.with_pos(1000, 80)
			.with_size(90, 30)
			.with_label("Новый ид-р кон. остановки:");
		let stop_input = IntInput::default()
			.with_pos(1145, 80)
			.with_size(60, 30);
		let mut stop_btn = Button::default()
			.with_pos(1215, 80)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl4 = Frame::default()
			.with_pos(790, 115)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_input4 = IntInput::default()
			.with_pos(890, 115)
			.with_size(60, 30);
		let _entry_lbl = Frame::default()
			.with_pos(1000, 115)
			.with_size(90, 30)
			.with_label("Новая дата введения:");
		let entry_input = Input::default()
			.with_pos(1145, 115)
			.with_size(60, 30);
		let mut entry_btn = Button::default()
			.with_pos(1215, 115)
			.with_size(180, 30)
			.with_label("Поменять");

		let mut add_btn = Button::default()
			.with_pos(760, 575)
			.with_size(20, 20)
			.with_label("@+");
		let mut rmv_btn = Button::default()
			.with_pos(760, 555)
			.with_size(20, 20)
			.with_label("@1+");

		unsafe {
			transport_controller::table();

			type_btn.set_callback(move |_|
				transport_model::change_type(nmb_input1.value().parse().unwrap(), type_input.value())
			);

			start_btn.set_callback(move |_|
				transport_model::change_start(nmb_input2.value().parse().unwrap(), start_input.value().parse().unwrap())
			);

			stop_btn.set_callback(move |_|
				transport_model::change_stop(nmb_input3.value().parse().unwrap(), stop_input.value().parse().unwrap())
			);

			entry_btn.set_callback(move |_|
				transport_model::change_date(nmb_input4.value().parse().unwrap(), entry_input.value())
			);

			add_btn.set_callback(&add_window);

			rmv_btn.set_callback(&del_window);
		}
		transport_window.end();
		transport_window.show();
	}

	fn add_window(_w: &mut impl WidgetExt) {
		let mut add_window = Window::default()
			.with_size(300, 400)
			.with_label("Новый транспорт");

		let _nmb_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Номер маршрута:");
		let nmb_input = IntInput::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		let _entry_lbl = Frame::default()
			.with_pos(45, 45)
			.with_size(90, 30)
			.with_label("Дата введения:");
		let entry_input = Input::default()
			.with_pos(180, 45)
			.with_size(60, 30);

		let _start_lbl = Frame::default()
			.with_pos(45, 80)
			.with_size(90, 30)
			.with_label("Ид-р нач. остановки:");
		let start_input = IntInput::default()
			.with_pos(180, 80)
			.with_size(60, 30);

		let _stop_lbl = Frame::default()
			.with_pos(45, 115)
			.with_size(90, 30)
			.with_label("Ид-р кон. остановки:");
		let stop_input = IntInput::default()
			.with_pos(180, 115)
			.with_size(60, 30);
		
		let _type_lbl = Frame::default()
			.with_pos(45, 150)
			.with_size(90, 30)
			.with_label("Тип:");
		let mut type_input = Choice::default()
			.with_pos(180, 150)
			.with_size(60, 30);
		unsafe { type_input.add_choice(&get_transport_types()) };
		type_input.set_value(0);

		let _stops_lbl = Frame::default()
			.with_pos(90, 185)
			.with_size(120, 30)
			.with_label("Введите через запятую все остановки, на\nкоторых останавливается данный маршрут:");
		let stops_input = Input::default()
			.with_pos(90, 225)
			.with_size(120, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				transport_controller::prepare_row_crt(
					nmb_input.value(),
					start_input.value(),
					stop_input.value(),
					type_input.choice().unwrap(),
					entry_input.value(),
					stops_input.value()
				)
			);
		}

		add_window.end();
		add_window.show();
	}

	fn del_window(_w: &mut impl WidgetExt) {
		let mut del_window = Window::default()
			.with_size(300, 400)
			.with_label("Удалить транспорт");

		let _entry_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Номер маршрута:");
		let entry_input = IntInput::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				transport_controller::prepare_row_del(entry_input.value())
			);
		}
		del_window.end();
		del_window.show();
	}
}