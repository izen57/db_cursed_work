pub mod fare_model {
	struct Fare {
		price: f64,
		root_number: i32,
		start_id: i32,
		stop_id: i32,
		day_time: String
	}

	impl Fare {
		fn new(price: f64, root_number: i32, start_id: i32, stop_id: i32, day_time: String) -> Fare {
			Fare{price, root_number, start_id, stop_id, day_time}
		}
	}
}