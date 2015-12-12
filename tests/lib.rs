extern crate chattium_oxide_lib as cho;  // Chang
extern crate rand;


use rand::Rng;
use std::net::Ipv4Addr;

fn random_ip<Rand: Rng>(rng: &mut Rand) -> (Ipv4Addr, u16) {
	(Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen()), rng.gen())
}

fn random_name<Rand: Rng>(rng: &mut Rand) -> String {
	rng.gen_ascii_chars().take(10).collect()
}


#[cfg(test)]
mod user {
	use random_ip;
	use random_name;
	use rand;
	use cho::*;

	#[test]
	fn self_eq_self() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let user = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
			assert_eq!(user, user);
		}
	}

	#[test]
	fn equal_because_of_different_names_same_ips() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

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

	mod j_son {
		use random_ip;
		use random_name;
		use rand;
		use cho::*;
		use cho::json::*;


		#[test]
		fn full_transserializes_properly() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let user = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
				let trans = ChatUser::from_json(user.to_json()).expect("Full transserialization");
				assert_eq!(user, trans);
				assert_eq!(user.name, trans.name);
			}
		}

		#[test]
		fn ipless_transserializes_properly() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let user = ChatUser::me(random_name(&mut rng));
				let trans = ChatUser::from_json(user.to_json()).expect("IP-less transserialization");
				assert_eq!(user, trans);
				assert_eq!(user.name, trans.name);
			}
		}
		#[test]
		fn full_transserializes_properly_through_string() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let user = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
				let user_s = user.to_json_string().expect("Full serialization to string");
				let trans = ChatUser::from_json_string(&user_s).expect("Full deserialization from string");
				assert_eq!(user, trans);
				assert_eq!(user.name, trans.name);
			}
		}

		#[test]
		fn ipless_transserializes_properly_through_string() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let user = ChatUser::me(random_name(&mut rng));
				let user_s = user.to_json_string().expect("IP-less serialization to string");
				let trans = ChatUser::from_json_string(&user_s).expect("IP-less deserialization from string");
				assert_eq!(user, trans);
				assert_eq!(user.name, trans.name);
			}
		}

		#[test]
		//#[should_fail]  // The attribute `should_fail` is currently unknown to the compiler and may have meaning added to it in the future
		fn deserialization_from_malformed_fails() {
			ChatUser::from_json_string(&"{\"user\": \"you\"}".to_string()).unwrap_err();
		}
	}
}
