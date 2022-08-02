use fltk::{
	app::App,
	button::Button,
	enums::{ Color, FrameType::BorderFrame },
	frame::Frame,
	group::Group,
	input::Input,
	menu::Choice,
	prelude::*,
	window::Window
};
use fltk_table::*;
use postgres::{ Client, Error, NoTls };

use work::views::{
	fare_view::*,
	transport_view::*,
	transportstop_view::*,
	timetable_view::*,
	filter_view::*
};
use work::models::client_model::*;

const WIDTH: i32 = 400;
const HEIGHT: i32 = 700;

fn main() -> Result<(), Error> {
	let app = App::default();
	let mut role_window = Window::default()
		.with_size(500, 500)
		.center_screen();

	let role_lbl = Frame::default()
		.with_pos(255, 210)
		.with_label("Выберите роль:");
	let mut role_choice = Choice::default()
		.with_size(100, 30)
		.with_pos(205, 230);
	role_choice.add_choice("Пассажир|Диспетчер");

	let mut role_btn = Button::default()
		.with_size(100, 50)
		.with_pos(205, 300)
		.with_label("Выбрать");
	role_btn.set_callback(move |_|
		println!("Вы вошли в систему как {}", roles::User::set_role(role_choice.choice().unwrap()))
		// добавить для диспетчера ввод по паролю
	);

	role_window.end();
	role_window.show();
	app.run().unwrap();

	Ok(())
}

fn menu_window() {
	let mut main_window = Window::default()
		.with_size(WIDTH, HEIGHT)
		.center_screen();

	let mut fare = Button::default()
		.with_size(180, 50)
		.with_pos(100, 80)
		.with_label("Список тарифов");
	fare.set_callback(&fare_view::fare_window);

	let mut transport = Button::default()
		.with_size(180, 50)
		.with_pos(100, 135)
		.with_label("Список транспорта");
	transport.set_callback(&transport_view::transport_window);

	let mut transport_stops = Button::default()
		.with_size(180, 50)
		.with_pos(100, 190)
		.with_label("Список остановок");
	transport_stops.set_callback(&transportstop_view::transportstop_window);

	let mut timetable = Button::default()
		.with_size(180, 50)
		.with_pos(100, 245)
		.with_label("Расписание");
	timetable.set_callback(&timetable_view::timetable_window);

	let mut filter = Button::default()
		.with_size(180, 50)
		.with_pos(100, 300)
		.with_label("Фильтр");
	filter.set_callback(&filter_view::filter_window);

	main_window.end();
	main_window.show();
}