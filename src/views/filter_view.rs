pub mod filter_view {
	use chrono::{NaiveDate, NaiveTime};
	use fltk::{
		button::{ Button, CheckButton },
		dialog::alert_default,
		enums::FrameType,
		frame::Frame,
		input::{ Input, FloatInput },
		menu::Choice,
		prelude::*,
		window::Window
	};
	use fltk_table::{SmartTable, TableOpts};

	use crate::controllers::filter_controller::*;
	use crate::controllers::transport_controller::transport_controller::get_transport_types;
	use crate::models::transport_model::transport_model::TrType;

	pub fn filter_window(_w: &mut impl WidgetExt) {
		let mut filter_window = Window::default()
			.with_size(800, 800)
			.with_label("Фильтр");


		let mut trns_frame = Frame::default()
			.with_pos(10, 10)
			.with_size(231, 301);
		trns_frame.set_frame(FrameType::UpBox);

		let _nmb_trns_lbl = Frame::default()
			.with_pos(35, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_trns_input = Input::default()
			.with_pos(20, 35)
			.with_size(60, 20);
		let mut nmb_trns_choice = Choice::default()
			.with_pos(90, 35)
			.with_size(60, 20);
		nmb_trns_choice.add_choice("=|<|>|<=|>=");
		nmb_trns_choice.set_value(0);

		let _start_trns_lbl = Frame::default()
			.with_pos(57, 50)
			.with_size(80, 30)
			.with_label("ID начальной остановки:");
		let start_trns_input = Input::default()
			.with_pos(20, 75)
			.with_size(60, 20);
		let mut start_trns_choice = Choice::default()
			.with_pos(90, 75)
			.with_size(60, 20);
		start_trns_choice.add_choice("=|<|>|<=|>=");
		start_trns_choice.set_value(0);

		let _stop_trns_lbl = Frame::default()
			.with_pos(53, 90)
			.with_size(80, 30)
			.with_label("ID конечной остановки:");
		let stop_trns_input = Input::default()
			.with_pos(20, 115)
			.with_size(60, 20);
		let mut stop_trns_choice = Choice::default()
			.with_pos(90, 115)
			.with_size(60, 20);
		stop_trns_choice.add_choice("=|<|>|<=|>=");
		stop_trns_choice.set_value(0);

		let _type_lbl = Frame::default()
			.with_pos(30, 130)
			.with_size(80, 30)
			.with_label("Тип транспорта:");
		let mut type_choice = Choice::default()
			.with_pos(20, 155)
			.with_size(80, 20);
		unsafe { type_choice.add_choice(&get_transport_types()) };

		let _date_lbl = Frame::default()
			.with_pos(60, 163)
			.with_size(80, 60)
			.with_label("Дата введения маршрута\n(YYYY-MM-DD):");
		let date_input = Input::default()
			.with_pos(20, 213)
			.with_size(60, 20);
		let mut date_choice = Choice::default()
			.with_pos(90, 213)
			.with_size(60, 20);
		date_choice.add_choice("=|<|>|<=|>=");
		date_choice.set_value(0);

		let mut send_trns_btn = Button::default()
			.with_pos(100, 275)
			.with_size(80, 30)
			.with_label("Найти");
		send_trns_btn.set_callback(move |_| answer_trns_window(
			(nmb_trns_input.value(), nmb_trns_choice.choice().unwrap()),
			(start_trns_input.value(), start_trns_choice.choice().unwrap()),
			(stop_trns_input.value(), stop_trns_choice.choice().unwrap()),
			type_choice.choice(),
			(date_input.value(), date_choice.choice().unwrap())
		));


		let mut fare_frame = Frame::default()
			.with_pos(250, 10)
			.with_size(231, 301);
		fare_frame.set_frame(FrameType::UpBox);

		let _nmb_fare_lbl = Frame::default()
			.with_pos(277, 10)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_fare_input = Input::default()
			.with_pos(260, 35)
			.with_size(60, 20);
		let mut nmb_fare_choice = Choice::default()
			.with_pos(330, 35)
			.with_size(60, 20);
		nmb_fare_choice.add_choice("=|<|>|<=|>=");
		nmb_fare_choice.set_value(0);

		let _start_fare_lbl = Frame::default()
			.with_pos(300, 50)
			.with_size(80, 30)
			.with_label("ID начальной остановки:");
		let start_fare_input = Input::default()
			.with_pos(260, 75)
			.with_size(60, 20);
		let mut start_fare_choice = Choice::default()
			.with_pos(330, 75)
			.with_size(60, 20);
		start_fare_choice.add_choice("=|<|>|<=|>=");
		start_fare_choice.set_value(0);

		let _stop_fare_lbl = Frame::default()
			.with_pos(290, 90)
			.with_size(80, 30)
			.with_label("ID конечной остановки:");
		let stop_fare_input = Input::default()
			.with_pos(260, 115)
			.with_size(60, 20);
		let mut stop_fare_choice = Choice::default()
			.with_pos(330, 115)
			.with_size(60, 20);
		stop_fare_choice.add_choice("=|<|>|<=|>=");
		stop_fare_choice.set_value(0);

		let _price_lbl = Frame::default()
			.with_pos(265, 130)
			.with_size(80, 30)
			.with_label("Цена билета:");
		let price_input = Input::default()
			.with_pos(260, 155)
			.with_size(60, 20);
		let mut price_choice = Choice::default()
			.with_pos(330, 155)
			.with_size(60, 20);
		price_choice.add_choice("=|<|>|<=|>=");
		price_choice.set_value(0);

		let _daytime_lbl = Frame::default()
			.with_pos(265, 170)
			.with_size(80, 30)
			.with_label("Время суток:");
		let daytime_input = Input::default()
			.with_pos(260, 195)
			.with_size(60, 20);

		let mut send_fare_btn = Button::default()
			.with_pos(340, 275)
			.with_size(80, 30)
			.with_label("Найти");
		send_fare_btn.set_callback(move |_| answer_fare_window(
			(nmb_fare_input.value(), nmb_fare_choice.choice().unwrap()),
			(start_fare_input.value(), start_fare_choice.choice().unwrap()),
			(stop_fare_input.value(), stop_fare_choice.choice().unwrap()),
			(price_input.value(), price_choice.choice().unwrap()),
			daytime_input.value()
		));


		let mut trst_frame = Frame::default()
			.with_pos(10, 320)
			.with_size(231, 335);
		trst_frame.set_frame(FrameType::UpBox);

		let _id_lbl = Frame::default()
			.with_pos(25, 320)
			.with_size(80, 30)
			.with_label("ID остановки:");
		let id_input = Input::default()
			.with_pos(20, 345)
			.with_size(60, 20);
		let mut id_choice = Choice::default()
			.with_pos(90, 345)
			.with_size(60, 20);
		id_choice.add_choice("=|<|>|<=|>=");
		id_choice.set_value(0);

		let _name_lbl = Frame::default()
			.with_pos(17, 360)
			.with_size(80, 30)
			.with_label("Название:");
		let name_input = Input::default()
			.with_pos(20, 385)
			.with_size(60, 20);

		let _adr_lbl = Frame::default()
			.with_pos(5, 400)
			.with_size(80, 30)
			.with_label("Адрес:");
		let adr_input = Input::default()
			.with_pos(20, 425)
			.with_size(60, 20);

		let _req_lbl = Frame::default()
			.with_pos(30, 445)
			.with_size(80, 30)
			.with_label("По требованию:");
		let req_input = CheckButton::default()
			.with_pos(20, 465)
			.with_size(80, 20);

		let _date_lbl = Frame::default()
			.with_pos(62, 470)
			.with_size(80, 60)
			.with_label("Дата установки остановки\n(YYYY-MM-DD):");
		let date_input = Input::default()
			.with_pos(20, 520)
			.with_size(60, 20);
		let mut date_choice = Choice::default()
			.with_pos(90, 520)
			.with_size(60, 20);
		date_choice.add_choice("=|<|>|<=|>=");
		date_choice.set_value(0);

		let _elec_lbl = Frame::default()
			.with_pos(10, 540)
			.with_size(80, 30)
			.with_label("Рельсы:");
		let elec_input = CheckButton::default()
			.with_pos(20, 560)
			.with_size(80, 20);

		let _rails_lbl = Frame::default()
			.with_pos(45, 575)
			.with_size(80, 30)
			.with_label("Контактный провод:");
		let rails_input = CheckButton::default()
			.with_pos(20, 595)
			.with_size(80, 20);

		let mut send_trst_btn = Button::default()
			.with_pos(100, 620)
			.with_size(80, 30)
			.with_label("Найти");
		send_trst_btn.set_callback(move |_| answer_trst_window(
			(id_input.value(), id_choice.choice().unwrap()),
			name_input.value(),
			adr_input.value(),
			req_input.value(),
			(date_input.value(), date_choice.choice().unwrap()),
			elec_input.value(),
			rails_input.value()
		));


		let mut tmt_frame = Frame::default()
			.with_pos(250, 320)
			.with_size(231, 301);
		tmt_frame.set_frame(FrameType::UpBox);

		let _nmb_tmt_lbl = Frame::default()
			.with_pos(277, 320)
			.with_size(80, 30)
			.with_label("Номер маршрута:");
		let nmb_tmt_input = Input::default()
			.with_pos(260, 345)
			.with_size(60, 20);
		let mut nmb_tmt_choice = Choice::default()
			.with_pos(330, 345)
			.with_size(60, 20);
		nmb_tmt_choice.add_choice("=|<|>|<=|>=");
		nmb_tmt_choice.set_value(0);

		let _time_lbl = Frame::default()
			.with_pos(300, 360)
			.with_size(80, 30)
			.with_label("Время прибытия (ЧЧ:ММ):");
		let time_input = Input::default()
			.with_pos(260, 385)
			.with_size(60, 20);
		let mut time_choice = Choice::default()
			.with_pos(330, 385)
			.with_size(60, 20);
		time_choice.add_choice("=|<|>|<=|>=");
		time_choice.set_value(0);

		let _stopid_lbl = Frame::default()
			.with_pos(265, 400)
			.with_size(80, 30)
			.with_label("ID остановки:");
		let stopid_input = Input::default()
			.with_pos(260, 425)
			.with_size(60, 20);
		let mut stopid_choice = Choice::default()
			.with_pos(330, 425)
			.with_size(60, 20);
		stopid_choice.add_choice("=|<|>|<=|>=");
		stopid_choice.set_value(0);

		let _price_lbl = Frame::default()
			.with_pos(315, 440)
			.with_size(80, 30)
			.with_label("Максимальная цена за проезд:");
		let price_input = FloatInput::default()
			.with_pos(260, 465)
			.with_size(60, 20);
		let mut price_choice = Choice::default()
			.with_pos(330, 465)
			.with_size(60, 20);
		price_choice.add_choice("=|<|>|<=|>=");
		price_choice.set_value(0);

		let _wknd_lbl = Frame::default()
			.with_pos(285, 480)
			.with_size(80, 30)
			.with_label("Ходит по выходным:");
		let wknd_input = CheckButton::default()
			.with_pos(260, 500)
			.with_size(60, 20);

		let mut send_tmt_btn = Button::default()
			.with_pos(340, 525)
			.with_size(80, 30)
			.with_label("Найти");
		send_tmt_btn.set_callback(move |_| answer_tmt_window(
			(nmb_tmt_input.value(), nmb_tmt_choice.choice().unwrap()),
			(time_input.value(), time_choice.choice().unwrap()),
			(stopid_input.value(), stopid_choice.choice().unwrap()),
			(price_input.value(), price_choice.choice().unwrap()),
			wknd_input.value()
		));

		filter_window.end();
		filter_window.show();
	}

	fn answer_trns_window(root: (String, String), start_id: (String, String), stop_id: (String, String), trnstype: Option<String>, date: (String, String)) {
		let mut ans_window = Window::default()
			.with_size(800, 800)
			.with_label("Результат");

		let resvec = filter_controller::prepare_trns_query(root, start_id, stop_id, trnstype, date);
		if resvec.is_empty() {
			alert_default("Результаты запроса отсутствуют");
			return;
		}
		let rowcount = resvec.len();

		let mut table = SmartTable::default()
			.with_size(600, 400)
			.center_of_parent()
			.with_opts(TableOpts{
				rows: rowcount as i32,
				cols: 5,
				editable: false,
				..Default::default()
			});

		table.set_col_header_value(0, "Номер маршрута");
		table.set_col_header_value(1, "Ид-р начальной остановки");
		table.set_col_header_value(2, "Ид-р конечной остановки");
		table.set_col_header_value(3, "Тип");
		table.set_col_header_value(4, "Время ввода");

		for (row_index, row) in resvec.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "trtype" {
					let value: TrType = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "date" {
					let value: NaiveDate = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}

		ans_window.end();
		ans_window.show();
	}

	fn answer_fare_window(root: (String, String), start_id: (String, String), stop_id: (String, String), price: (String, String), daytime: String) {
		let mut ans_window = Window::default()
			.with_size(800, 800)
			.with_label("Результат");

		let resvec = filter_controller::prepare_fare_query(root, start_id, stop_id, price, daytime);
		if resvec.is_empty() {
			alert_default("Результаты запроса отсутствуют");
			return;
		}
		let rowcount = resvec.len();

		let mut table = SmartTable::default()
			.with_size(600, 400)
			.center_of_parent()
			.with_opts(TableOpts{
				rows: rowcount as i32,
				cols: 5,
				editable: false,
				..Default::default()
			});

			table.set_col_header_value(0, "Номер маршрута");
			table.set_col_header_value(1, "Цена билета");
			table.set_col_header_value(2, "Ид-р начальной остановки");
			table.set_col_header_value(3, "Ид-р конечной остановки");
			table.set_col_header_value(4, "Время");

		for (row_index, row) in resvec.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "text" {
					let value: &str = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value);
				} else if col_type == "float8" {
					let value: f64 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}

		ans_window.end();
		ans_window.show();
	}

	fn answer_trst_window(id: (String, String), name: String, address: String, request: bool, install_year: (String, String), electricity: bool, rails: bool) {
		let mut ans_window = Window::default()
			.with_size(800, 800)
			.with_label("Результат");

		let resvec = filter_controller::prepare_trst_query(id, name, address, request, install_year, electricity, rails);
		if resvec.is_empty() {
			alert_default("Результаты запроса отсутствуют");
			return;
		}
		let rowcount = resvec.len();

		let mut table = SmartTable::default()
			.with_size(600, 400)
			.center_of_parent()
			.with_opts(TableOpts{
				rows: rowcount as i32,
				cols: 7,
				editable: false,
				..Default::default()
			});

		table.set_col_header_value(0, "Ид-р остановки");
		table.set_col_header_value(1, "Название");
		table.set_col_header_value(2, "Адрес");
		table.set_col_header_value(3, "По требованию");
		table.set_col_header_value(4, "Год установки");
		table.set_col_header_value(5, "Контактный провод");
		table.set_col_header_value(6, "Рельсы");

		for (row_index, row) in resvec.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "text" {
					let value: &str = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value);
				} else if col_type == "date" {
					let value: NaiveDate = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "bool" {
					let value: bool = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, if value { "есть" } else { "нет" });
				}
			}
		}

		ans_window.end();
		ans_window.show();
	}

	fn answer_tmt_window(root: (String, String), time: (String, String), stop_id: (String, String), price: (String, String), weekends: bool) {
		let mut ans_window = Window::default()
			.with_size(800, 800)
			.with_label("Результат");

		let resvec = filter_controller::prepare_tmt_query(root, time, stop_id, price, weekends);
		if resvec.is_empty() {
			alert_default("Результаты запроса отсутствуют");
			return;
		}
		let rowcount = resvec.len();

		let mut table = SmartTable::default()
			.with_size(600, 400)
			.center_of_parent()
			.with_opts(TableOpts{
				rows: rowcount as i32,
				cols: 5,
				editable: false,
				..Default::default()
			});

		table.set_col_header_value(0, "Время прибытия");
		table.set_col_header_value(1, "Ид-р остановки");
		table.set_col_header_value(2, "Номер маршрута");
		table.set_col_header_value(3, "Работа по выходным");
		table.set_col_header_value(4, "Максимальная цена за проезд");

		for (row_index, row) in resvec.iter().enumerate() {
			for (col_index, col) in row.columns().iter().enumerate() {
				let col_type: String = col.type_().to_string();

				if col_type == "int4" {
					let value: i32 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "bool" {
					let value: bool = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, if value { "есть" } else { "нет" });
				} else if col_type == "time" {
					let value: NaiveTime = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				} else if col_type == "float8" {
					let value: f64 = row.get(col_index);
					table.set_cell_value(row_index as i32, col_index as i32, &value.to_string());
				}
			}
		}

		ans_window.end();
		ans_window.show();
	}
}