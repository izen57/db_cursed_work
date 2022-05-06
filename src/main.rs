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
use postgres::{Client, NoTls, Error};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 600;

struct Fare {
	price: f64,
	root_number: i32,
	start_id: i32,
	stop_id: i32,
	day_time: String
}

struct Timetable {
	timing: NaiveTime,
	root: i32,
	max_price: f64,
	transport_stop_id: i32,
	weekends: bool
}

struct Transport {
	root_number: i32,
	start_id: i32,
	stop_id: i32,
	transport_type: String,
	entry_date: NaiveDate
}

struct Transport_stop {
	id: i32,
	name: String,
	address: String,
	request_stop: bool,
	install_year: NaiveDate,
	electricity: bool,
	rails: bool
}

fn main() -> Result<(), Error> {
	let app = App::default();
	let mut main_window = Window::default()
		.with_size(WIDTH, HEIGHT)
		.center_screen();
		
	let insert_input = Input::default()
		.with_size(180, 30)
		.with_pos(460, 40);

	let insert_btn = Button::default()
		.with_size(180, 30)
		.with_pos(460, 80)
		.with_label("Вставить строку");

	let mut insert_frame = Group::default()
		.with_size(200, 100)
		.with_pos(450, 30)
		.with_label("0");
	insert_frame.set_frame(BorderFrame);
	insert_frame.set_color(Color::Red);
	insert_frame.add(&insert_btn);
	insert_frame.add(&insert_input);

	let result_table = SmartTable::default()
		.with_size(400, 600)
		.with_opts(TableOpts {
			..Default::default()
		});
	// let mut but_inc = Button::default()
	//     .size_of(&frame)
	//     .above_of(&frame, 0)
	//     .with_label("+");
	// let mut but_dec = Button::default()
	//     .size_of(&frame)
	//     .below_of(&frame, 0)
	//     .with_label("-");
	// wind.make_resizable(true);
	main_window.end();
	main_window.show();
	/* Event handling */
	app.run().unwrap();
	let mut client = Client::connect("host=localhost user=postgres password=pgadminkoro", NoTls)?;
	for row in client.query("select start_id from fare limit 10", &[])? {
		let start_id: i32 = row.get("start_id");
		println!("{}", start_id);
	}

	Ok(())
}