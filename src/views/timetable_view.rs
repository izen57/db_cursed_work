pub mod timetable_view {
	use fltk::{
		button::{ Button, CheckButton },
		frame::Frame,
		input::{ Input, IntInput, FloatInput },
		prelude::*,
		window::Window
	};
	use crate::{ controllers::timetable_controller::*, models::timetable_model::* };

	pub fn timetable_window(_w: &mut impl WidgetExt) {
		let mut timetable_window = Window::default()
			.with_size(1500, 600)
			.with_label("Расписание");

		let _nmb_lbl1 = Frame::default()
			.with_pos(790, 10)
			.with_size(80, 60)
			.with_label("Номер,время,\nостановка:");
		let nmb_input1 = Input::default()
			.with_pos(890, 10)
			.with_size(60, 30);
		let _trst_lbl = Frame::default()
			.with_pos(990, 10)
			.with_size(60, 30)
			.with_label("Новый ид-р остановки:");
		let trst_input = IntInput::default()
			.with_pos(1145, 10)
			.with_size(60, 30);
		let mut trst_btn = Button::default()
			.with_pos(1215, 10)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl2 = Frame::default()
			.with_pos(790, 45)
			.with_size(80, 60)
			.with_label("Номер,время,\nостановка:");
		let nmb_input2 = Input::default()
			.with_pos(890, 45)
			.with_size(60, 30);
		let _root_lbl = Frame::default()
			.with_pos(1000, 45)
			.with_size(90, 30)
			.with_label("Новый номер маршрута:");
		let root_input = IntInput::default()
			.with_pos(1145, 45)
			.with_size(60, 30);
		let mut root_btn = Button::default()
			.with_pos(1215, 45)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl3 = Frame::default()
			.with_pos(790, 80)
			.with_size(80, 60)
			.with_label("Номер,время,\nостановка:");
		let nmb_input3 = Input::default()
			.with_pos(890, 80)
			.with_size(60, 30);
		let _wknd_lbl = Frame::default()
			.with_pos(1000, 80)
			.with_size(90, 30)
			.with_label("Работа по выходным:");
		let wknd_input = CheckButton::default()
			.with_pos(1145, 80)
			.with_size(60, 30);
		let mut wknd_btn = Button::default()
			.with_pos(1215, 80)
			.with_size(180, 30)
			.with_label("Поменять");

		let _nmb_lbl4 = Frame::default()
			.with_pos(790, 115)
			.with_size(80, 60)
			.with_label("Номер,время,\nостановка:");
		let nmb_input4 = Input::default()
			.with_pos(890, 115)
			.with_size(60, 30);
		let _price_lbl = Frame::default()
			.with_pos(1000, 115)
			.with_size(90, 30)
			.with_label("Новая макс. цена:");
		let price_input = FloatInput::default()
			.with_pos(1145, 115)
			.with_size(60, 30);
		let mut price_btn = Button::default()
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
			timetable_controller::table();

			trst_btn.set_callback(move |_|
				timetable_model::change_stop(nmb_input1.value(),
				trst_input.value().parse().unwrap())
			);

			root_btn.set_callback(move |_|
				timetable_model::change_root(nmb_input2.value(), root_input.value().parse().unwrap())
			);

			wknd_btn.set_callback(move |_|
				timetable_model::change_stop(nmb_input3.value(), wknd_input.value().to_string())
			);

			price_btn.set_callback(move |_|
				timetable_model::change_price(nmb_input4.value(), price_input.value().parse().unwrap())
			);

			add_btn.set_callback(&add_window);

			rmv_btn.set_callback(&del_window);
		}
		timetable_window.end();
		timetable_window.show();
	}

	fn add_window(_w: &mut impl WidgetExt) {
		let mut add_window = Window::default()
			.with_size(300, 400)
			.with_label("Новое расписание");

		let _nmb_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Время прибытия:");
		let nmb_input = Input::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		let _trst_lbl = Frame::default()
			.with_pos(45, 45)
			.with_size(90, 30)
			.with_label("Ид-р остановки:");
		let trst_input = IntInput::default()
			.with_pos(180, 45)
			.with_size(60, 30);

		let _root_lbl = Frame::default()
			.with_pos(45, 80)
			.with_size(90, 30)
			.with_label("Номер маршрута:");
		let root_input = IntInput::default()
			.with_pos(180, 80)
			.with_size(60, 30);

		let _wknd_lbl = Frame::default()
			.with_pos(45, 115)
			.with_size(90, 30)
			.with_label("Работа по выходным (0 или 1):");
		let wknd_input = IntInput::default()
			.with_pos(180, 115)
			.with_size(60, 30);

		let _price_lbl = Frame::default()
			.with_pos(45, 150)
			.with_size(90, 30)
			.with_label("Максимальная цена:");
		let price_input = FloatInput::default()
			.with_pos(180, 150)
			.with_size(60, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				timetable_controller::prepare_row_crt(
					nmb_input.value(),
					root_input.value(),
					price_input.value(),
					trst_input.value(),
					wknd_input.value()
				)
			);
		}

		add_window.end();
		add_window.show();
	}

	fn del_window(_w: &mut impl WidgetExt) {
		let mut del_window = Window::default()
			.with_size(300, 400)
			.with_label("Удалить расписание");

		let _entry_lbl = Frame::default()
			.with_pos(45, 10)
			.with_size(90, 30)
			.with_label("Время прибытия:");
		let entry_input = Input::default()
			.with_pos(180, 10)
			.with_size(60, 30);

		let _stop_lbl = Frame::default()
			.with_pos(45, 45)
			.with_size(90, 30)
			.with_label("ИД остановки:");
		let stop_input = IntInput::default()
			.with_pos(180, 45)
			.with_size(60, 30);

		let _root_lbl = Frame::default()
			.with_pos(45, 80)
			.with_size(90, 30)
			.with_label("Номер маршрута:");
		let root_input = IntInput::default()
			.with_pos(180, 80)
			.with_size(60, 30);

		let mut enter_btn = Button::default()
			.with_pos(270, 370)
			.with_size(30, 30)
			.with_label("@filesaveas");

		unsafe {
			enter_btn.set_callback(move |_|
				timetable_controller::prepare_row_del(
					root_input.value(),
					entry_input.value(),
					stop_input.value()
				)
			);
		}
		del_window.end();
		del_window.show();
	}
}