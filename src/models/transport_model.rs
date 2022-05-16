mod transport_models {
	struct Transport {
		root_number: i32,
		start_id: i32,
		stop_id: i32,
		transport_type: String,
		entry_date: NaiveDate
	}
	
	impl Transport {
		fn new(root_number: i32, start_id: i32, stop_id: i32, transport_type: String, entry_date: NaiveDate) -> Transport {
			Transport{root_number, start_id, stop_id, transport_type, entry_date}
		}
	}
}