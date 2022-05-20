pub mod roles {
	use postgres::{Client, Error, NoTls};

	pub enum User {
		No(),
		Passenger(Client),
		Manager(Client)
	}
	pub static mut U: User = User::No();

	impl User {
		pub fn set_passenger() -> Self {
			Self::Passenger(Client::connect("host=localhost user=postgres password=pgadminkoro", NoTls).expect("Что-то пошло не так..."))
		}

		pub fn set_manager() -> Self {
			Self::Manager(Client::connect("host=localhost user=postgres password=pgadminkoro", NoTls).expect("Что-то пошло не так..."))
		}

		pub fn get_valid(&mut self) -> &mut Client {
			match self {
				Self::No() => panic!("Соединение не установлено"),
				Self::Passenger(p) => p,
				Self::Manager(m) => m
			}
		}
	}
}