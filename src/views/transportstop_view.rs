pub mod transportstop_view {
	use fltk::{
		button::{ Button, CheckButton },
		enums::Align,
		frame::Frame,
		input::{ Input, IntInput, FloatInput },
		prelude::*,
		window::Window
	};
	use crate::{ controllers::transportstop_controller::*, models::transportstop_model::* };

	pub fn transportstop_window(w: &mut impl WidgetExt) {
		let mut transport_window = Window::default()
			.with_size(1500, 600)
			.with_label("Полный список остановок транспорта");

		let nmb_lbl1 = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input1 = IntInput::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let name_lbl = Frame::default()
			.with_pos(990, 10)
			.with_size(60, 30)
			.with_label("Новое название:");
		let name_input = Input::default()
			.with_pos(1145, 10)
			.with_size(60, 30);
		let mut name_btn = Button::default()
			.with_pos(1215, 10)
			.with_size(180, 30)
			.with_label("Поменять");

		let nmb_lbl2 = Frame::default()
			.with_pos(790, 45)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input2 = IntInput::default()
			.with_pos(890, 45)
			.with_size(60, 30);
		let address_lbl = Frame::default()
			.with_pos(1000, 45)
			.with_size(90, 30)
			.with_label("Новый адрес:");
		let address_input = Input::default()
			.with_pos(1145, 45)
			.with_size(60, 30);
		let mut address_btn = Button::default()
			.with_pos(1215, 45)
			.with_size(180, 30)
			.with_label("Поменять");

		let nmb_lbl3 = Frame::default()
			.with_pos(790, 80)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input3 = IntInput::default()
			.with_pos(890, 80)
			.with_size(60, 30);
		let stop_lbl = Frame::default()
			.with_pos(1000, 80)
			.with_size(90, 30)
			.with_label("По требованию:");
		let stop_input = IntInput::default()
			.with_pos(1145, 80)
			.with_size(60, 30);
		let mut stop_btn = Button::default()
			.with_pos(1215, 80)
			.with_size(180, 30)
			.with_label("Поменять");

		let nmb_lbl4 = Frame::default()
			.with_pos(790, 115)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input4 = IntInput::default()
			.with_pos(890, 115)
			.with_size(60, 30);
		let year_lbl = Frame::default()
			.with_pos(1000, 115)
			.with_size(90, 30)
			.with_label("Новый год установки:");
		let year_input = Input::default()
			.with_pos(1145, 115)
			.with_size(60, 30);
		let mut year_btn = Button::default()
			.with_pos(1215, 115)
			.with_size(180, 30)
			.with_label("Поменять");

		let nmb_lbl5 = Frame::default()
			.with_pos(790, 150)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input5 = IntInput::default()
			.with_pos(890, 150)
			.with_size(60, 30);
		let elec_lbl = Frame::default()
			.with_pos(1000, 150)
			.with_size(90, 30)
			.with_label("Контактный провод (0 или 1):");
		let elec_input = IntInput::default()
			.with_pos(1145, 150)
			.with_size(60, 30);
		let mut elec_btn = Button::default()
			.with_pos(1215, 150)
			.with_size(180, 30)
			.with_label("Поменять");

		let nmb_lbl6= Frame::default()
			.with_pos(790, 185)
			.with_size(80, 30)
			.with_label("Идентификатор:");
		let nmb_input6 = IntInput::default()
			.with_pos(890, 185)
			.with_size(60, 30);
		let rails_lbl = Frame::default()
			.with_pos(1000, 185)
			.with_size(90, 30)
			.with_label("Рельсы (0 или 1):");
		let rails_input = IntInput::default()
			.with_pos(1145, 185)
			.with_size(60, 30);
		let mut rails_btn = Button::default()
			.with_pos(1215, 185)
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
			let mut table = transportstop_controller::table();

			name_btn.set_callback(move |_|
				transportstop_model::change_name(nmb_input1.value().parse().unwrap(), name_input.value())
			);

			address_btn.set_callback(move |_|
				transportstop_model::change_address(nmb_input2.value().parse().unwrap(), address_input.value().parse().unwrap())
			);

			stop_btn.set_callback(move |_|
				transportstop_model::change_request(nmb_input3.value().parse().unwrap(), stop_input.value().parse().unwrap())
			);

			year_btn.set_callback(move |_|
				transportstop_model::change_year(nmb_input4.value().parse().unwrap(), year_input.value())
			);

			elec_btn.set_callback(move |_|
				transportstop_model::change_elec(nmb_input5.value().parse().unwrap(), elec_input.value())
			);

			rails_btn.set_callback(move |_|
				transportstop_model::change_rails(nmb_input6.value().parse().unwrap(), rails_input.value())
			);

			add_btn.set_callback(&add_window);

			rmv_btn.set_callback(&del_window);
		}
		transport_window.end();
		transport_window.show();
	}

	fn add_window(w: &mut impl WidgetExt) {
		let mut add_window = Window::default()
			.with_size(300, 400)
			.with_label("Новая остановка");

		let nmb_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Идентификатор:");
		let nmb_input = IntInput::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		// let timing_lbl = Frame::default()
		// 	.with_pos(45, 45)
		// 	.with_size(90, 30)
		// 	.with_label("Время прибытия:");
		// let timing_input = Input::default()
		// 	.with_pos(180, 45)
		// 	.with_size(60, 30);

		let name_lbl = Frame::default()
			.with_pos(45, 80)
			.with_size(90, 30)
			.with_label("Название:");
		let name_input = Input::default()
			.with_pos(180, 80)
			.with_size(60, 30);

		let address_lbl = Frame::default()
			.with_pos(45, 115)
			.with_size(90, 30)
			.with_label("Адрес:");
		let address_input = Input::default()
			.with_pos(180, 115)
			.with_size(60, 30);

		let stop_lbl = Frame::default()
			.with_pos(45, 150)
			.with_size(90, 30)
			.with_label("По требованию:");
		let stop_input = IntInput::default()
			.with_pos(180, 150)
			.with_size(60, 30);

		let year_lbl = Frame::default()
			.with_pos(45, 185)
			.with_size(90, 30)
			.with_label("Год установки:");
		let year_input = Input::default()
			.with_pos(180, 185)
			.with_size(60, 30);
		
		let elec_lbl = Frame::default()
			.with_pos(45, 220)
			.with_size(90, 30)
			.with_label("Контактный провод (0 или 1):");
		let elec_input = IntInput::default()
			.with_pos(180, 220)
			.with_size(60, 30);

		let rails_lbl = Frame::default()
			.with_pos(45, 255)
			.with_size(120, 30)
			.with_label("Рельсы (0 или 1):");
		let rails_input = IntInput::default()
			.with_pos(180, 255)
			.with_size(60, 30);

		let trsp_lbl = Frame::default()
			.with_pos(90, 290)
			.with_size(120, 30)
			.with_label("Введите через запятую все маршруты, \nкоторые останавливаются на данной остановке:");
		let trsp_input = Input::default()
			.with_pos(90, 325)
			.with_size(120, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				transportstop_controller::prepare_row_crt(
					nmb_input.value(),
					trsp_input.value(),
					name_input.value(),
					address_input.value(),
					stop_input.value(),
					year_input.value(),
					elec_input.value(),
					rails_input.value()
				)
			);
		}

		add_window.end();
		add_window.show();
	}

	fn del_window(w: &mut impl WidgetExt) {
		let mut del_window = Window::default()
			.with_size(300, 400)
			.with_label("Удалить остановку");

		let entry_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Идентификатор:");
		let entry_input = IntInput::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		// let and_stops_lbl = Frame::default()
		// 	.with_pos(45, 45)
		// 	.with_size(90, 30)
		// 	.with_label("Удалить также соотв. маршруты:");
		// let and_stops_check = CheckButton::default()
		// 	.with_pos(180, 45)
		// 	.with_size(60, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				transportstop_controller::prepare_row_del(
					entry_input.value()/*/,
					and_stops_check.is_checked()*/
				)
			);
		}
		del_window.end();
		del_window.show();
	}
}