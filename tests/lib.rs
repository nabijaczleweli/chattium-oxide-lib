extern crate chattium_oxide_lib as cho;  // Chang
extern crate rand;


#[cfg(test)]
mod user {
	use rand::{self, Rng};
	use cho::*;
	use std::net::Ipv4Addr;


	fn random_ip<Rand>(rng: &mut Rand) -> (Ipv4Addr, u16) where Rand: Rng {
		(Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen()), rng.gen())
	}

	fn random_name<Rand>(rng: &mut Rand) -> String where Rand: Rng {
		rng.gen_ascii_chars().take(10).collect()
	}

	#[test]
	fn equal_because_of_different_names_same_ips() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {
		            	100000
		            } else {
		            	1000
		            };

		for _ in 1..times {
			let ip = random_ip(&mut rng);
			let user_1 = ChatUser::get(random_name(&mut rng), ip);
			let user_2 = ChatUser::get(random_name(&mut rng), ip);
			assert_eq!(user_1, user_2);
		}
	}

	#[test]
	fn unequal_because_of_different_ips_same_names() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let user_1 = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
			let user_2 = ChatUser::get(user_1.name.clone(), random_ip(&mut rng));
			assert!(user_1 != user_2);
		}
	}

	#[test]
	fn name_of_instance_from_get_is_equal_to_passed_name() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let name = random_name(&mut rng);
			let user = ChatUser::get(name.clone(), "0.0.0.0:0");
			assert_eq!(user.name, name);
		}
	}

	#[test]
	fn cloner_eq_clonee() {
		let mut rng = rand::thread_rng();

		let user = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
		let clone = user.clone();
		assert_eq!(user, clone);
		assert_eq!(user.name, clone.name);
	}
}
