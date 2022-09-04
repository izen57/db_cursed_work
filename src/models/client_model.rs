pub mod roles {
	use postgres::{ Client, NoTls };

	pub enum User {
		None,
		Passenger(Client),
		Manager(Client)
	}
	pub static mut U: User = User::None;

	impl User {
		fn set_passenger() -> Self {
			Self::Passenger(Client::connect(
				"host=localhost user=passenger password=1111 dbname=test",
				NoTls
			).expect("Что-то пошло не так..."))
		}

		fn set_manager(password: String) -> Self {
			Self::Manager(Client::connect(
				&format!("host=localhost user=operator password={password} dbname=test"),
				NoTls
			).expect("Что-то пошло не так..."))
		}

		pub unsafe fn set_role(choice: String, password: String) -> String {
			match choice.get(..) {
				Some("Пассажир") => {
					U = Self::set_passenger();
					"Пассажир".to_string()
				},
				Some("Диспетчер") => {
					U = Self::set_manager(password);
					"Диспетчер".to_string()
				},
				Some(_) | None => "Ошибка".to_string()
			}
		}

		pub fn get_valid(&mut self) -> &mut Client {
			match self {
				Self::None => panic!("Соединение не установлено"),
				Self::Passenger(p) => p,
				Self::Manager(m) => m
			}
		}
	}
}