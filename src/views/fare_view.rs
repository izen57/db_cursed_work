pub mod fare_view {
	use fltk::{prelude::*, window::Window};
	use crate::controllers::fare_controller::*;

	pub fn fare_window(w: &mut impl WidgetExt) {
		let mut fare_window = Window::default()
			.with_size(1000,600)
			//.center_screen()
			.with_label("Тарифы");

		let _ = fare_controller::table();
		//println!("{:?}", fare_window.parent());
		fare_window.end();
		fare_window.show();
	}
}