use fltk::{
	app::{ App, handle },
	button::Button,
	dialog::message_default,
	enums::Event,
	frame::Frame,
	input::SecretInput,
	menu::Choice,
	prelude::*,
	window::Window
};
use postgres::Error;

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

	let _role_lbl = Frame::default()
		.with_pos(255, 110)
		.with_label("Выберите роль:");
	let mut role_choice = Choice::default()
		.with_size(100, 30)
		.with_pos(205, 130);
	role_choice.add_choice("Пассажир|Диспетчер");
	role_choice.set_value(1);
	role_choice.set_callback({
		let temp_w = role_window.clone();
		move |chc|
			if chc.choice() == Some(String::from("Диспетчер")) {
				handle(Event::Activate, &temp_w);
			} else if chc.choice() == Some(String::from("Пассажир")) {
				handle(Event::Deactivate, &temp_w);
			}
	});

	let _pswd_lbl = Frame::default()
		.with_pos(255, 180)
		.with_label("Введите пароль:");
	let mut pswd_input = SecretInput::default()
		.with_pos(205, 200)
		.with_size(100, 30);
	// сделать так, чтобы события обрабатывались не один раз
	pswd_input.handle(move |inp, event| {
		if event == Event::Deactivate {
			inp.deactivate();
			true
		} else if event == Event::Activate {
			inp.activate();
			true
		} else {
			false
		}
	});

	let mut role_btn = Button::default()
		.with_size(100, 50)
		.with_pos(205, 300)
		.with_label("Выбрать");
	role_btn.set_callback(move |_| unsafe {
		let user_string = roles::User::set_role(role_choice.choice().unwrap(), pswd_input.value());
		message_default(&format!("Вы вошли в систему как {user_string}."));
		menu_window();
	});

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
		.with_size(190, 50)
		.with_pos(100, 80)
		.with_label("Редактирование тарифов");
	fare.set_callback(&fare_view::fare_window);

	let mut transport = Button::default()
		.with_size(190, 50)
		.with_pos(100, 135)
		.with_label("Редактирование транспорта");
	transport.set_callback(&transport_view::transport_window);

	let mut transport_stops = Button::default()
		.with_size(190, 50)
		.with_pos(100, 190)
		.with_label("Редактирование остановок");
	transport_stops.set_callback(&transportstop_view::transportstop_window);

	let mut timetable = Button::default()
		.with_size(190, 50)
		.with_pos(100, 245)
		.with_label("Редактирование расписания");
	timetable.set_callback(&timetable_view::timetable_window);

	let mut filter = Button::default()
		.with_size(190, 50)
		.with_pos(100, 300)
		.with_label("Фильтр");
	filter.set_callback(&filter_view::filter_window);

	unsafe {
		match roles::U {
			roles::User::Passenger(_) => {
				fare.deactivate();
				transport.deactivate();
				transport_stops.deactivate();
				timetable.deactivate();
				println!("p");
			},
			roles::User::Manager(_) => println!("m"),
			roles::User::None => println!("n")
		}
	}

	main_window.end();
	main_window.show();
}