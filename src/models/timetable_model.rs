mod timetable_model {
	struct Timetable {
		timing: NaiveTime,
		root: i32,
		max_price: f64,
		transport_stop_id: i32,
		weekends: bool
	}
	
	impl Timetable {
		fn new(timing: NaiveTime, root: i32, max_price: f64, transport_stop_id: i32, weekends: bool) -> Timetable {
			Timetable{timing, root, max_price, transport_stop_id, weekends}
		}
	}
}