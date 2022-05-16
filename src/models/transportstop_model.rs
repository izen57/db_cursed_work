mod transportstop_model {
	struct TransportStop {
		id: i32,
		name: String,
		address: String,
		request_stop: bool,
		install_year: NaiveDate,
		electricity: bool,
		rails: bool
	}
	
	impl TransportStop {
		fn new(id: i32, name: String, address: String, request_stop: bool, install_year: NaiveDate, electricity: bool, rails: bool) -> TransportStop {
			TransportStop{id, name, address, request_stop, install_year, electricity, rails}
		}
	}
}