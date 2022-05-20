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
	// let insert_input = Input::default()
	// 	.with_size(180, 30)
	// 	.with_pos(460, 40);

	let mut insert_btn = Button::default()
		.with_size(180, 50)
		.with_pos(100, 80)
		.with_label("Список тарифов");
	insert_btn.set_callback(&fare_view::fare_window);

	// let mut insert_frame = Group::default()
	// 	.with_size(200, 100)
	// 	.with_pos(450, 30)
	// 	.with_label("0");
	// insert_frame.set_frame(BorderFrame);
	// insert_frame.set_color(Color::Red);
	// insert_frame.add(&insert_btn);
	// insert_frame.add(&insert_input);

	// let result_table = SmartTable::default()
	// 	.with_size(400, 600)
	// 	.with_opts(TableOpts{..Default::default()});
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
	// let mut client = Client::connect("host=localhost user=postgres password=pgadminkoro", NoTls)?;
	// for row in client.query("select start_id from fare limit 10", &[])? {
	// 	let start_id: i32 = row.get("start_id");
	// 	println!("{}", start_id);
	// }
	/* Event handling */
	app.run().unwrap();

	Ok(())
}