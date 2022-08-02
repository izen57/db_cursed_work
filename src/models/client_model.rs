pub mod roles {
	use postgres::{Client, Error, NoTls};

	pub enum User {
		None,
		Passenger(Client),
		Manager(Client)
	}
	pub static mut U: User = User::None;

	impl User {
		pub fn set_passenger() -> Self {
			Self::Passenger(Client::connect("host=localhost user=Пассажир password=pgadminkoro dbname=test", NoTls).expect("Что-то пошло не так..."))
		}

		pub fn set_manager() -> Self {
			Self::Manager(Client::connect("host=localhost user=postgres password=pgadminkoro", NoTls).expect("Что-то пошло не так..."))
		}

		pub fn set_role(choice: String) -> String {
			match choice.get(..) {
				Some("Пассажир") => {
					Self::set_passenger();
					"Пассажир".to_string()
				},
				Some("Диспетчер") => {
					Self::set_manager();
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