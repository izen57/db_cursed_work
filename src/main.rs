use chrono::{NaiveDate, NaiveTime};
use fltk::{
	app::App,
	button::Button,
	enums::{Color, FrameType::BorderFrame},
	group::Group,
	input::Input,
	prelude::*,
	window::Window
};
use fltk_table::*;
use postgres::{Client, Error, NoTls};

use work::views::fare_view::*;
use work::models::client::*;

const WIDTH: i32 = 400;
const HEIGHT: i32 = 700;

fn main() -> Result<(), Error> {
	let app = App::default();
	let mut main_window = Window::default()
		.with_size(WIDTH, HEIGHT)
		.center_screen();
	unsafe {
		roles::U = roles::User::set_passenger();
	}

	let mut fare = Button::default()
		.with_size(180, 50)
		.with_pos(100, 80)
		.with_label("Список тарифов");
	fare.set_callback(&fare_view::fare_window);

	main_window.end();
	main_window.show();
	app.run().unwrap();

	Ok(())
}