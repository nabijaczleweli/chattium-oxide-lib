extern crate chattium_oxide_lib as cho;  // Chang
extern crate rand;
extern crate time;


use time::Tm;
use rand::Rng;
use std::net::Ipv4Addr;


fn random_ip<Rand: Rng>(rng: &mut Rand) -> (Ipv4Addr, u16) {
	(Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen()), rng.gen())
}

fn random_name<Rand: Rng>(rng: &mut Rand) -> String {
	rng.gen_ascii_chars().take(10).collect()
}

fn random_text<Rand: Rng>(rng: &mut Rand) -> String {
	rng.gen_ascii_chars().take(100).collect()
}

fn random_time<Rand: Rng>(rng: &mut Rand) -> Tm {
	use time::{at_utc, Timespec};

	at_utc(Timespec::new(rng.gen_range(1420070400 /*1 Jan. 2015*/, 1893456000 /*1. Jan 2030*/), rng.gen_range(0, 1000000000)))
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

#[cfg(test)]
mod message {
	use random_ip;
	use random_name;
	use random_text;
	use rand;
	use cho::*;
	use std::time::Duration;
	use std::thread::sleep;


	#[test]
	fn self_eq_self() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let message = ChatMessage::new(ChatUser::get(random_name(&mut rng), random_ip(&mut rng)), random_text(&mut rng));
			assert_eq!(message, message);
		}
	}

	#[test]
	fn new_neq_new_time() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let user = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
			let text = random_text(&mut rng);
			let message_1 = ChatMessage::new(user.clone(), text.clone());
			sleep(Duration::new(0, 1));
			let message_2 = ChatMessage::new(user.clone(), text.clone());
			assert!(message_1.time_posted != message_2.time_posted);
		}
	}

	#[test]
	fn user_of_instance_from_new_is_equal_to_passed_user() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let sender = ChatUser::get(random_name(&mut rng), random_ip(&mut rng));
			let message = ChatMessage::new(sender.clone(), random_text(&mut rng));
			assert_eq!(message.sender, sender);
		}
	}

	#[test]
	fn content_of_instance_from_new_is_equal_to_passed_content() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let value = random_text(&mut rng);
			let message = ChatMessage::new(ChatUser::get(random_name(&mut rng), random_ip(&mut rng)), value.clone());
			assert_eq!(message.value, value);
		}
	}

	#[test]
	fn cloner_eq_clonee() {
		let mut rng = rand::thread_rng();

		let message = ChatMessage::new(ChatUser::get(random_name(&mut rng), random_ip(&mut rng)), random_text(&mut rng));
		let clone = message.clone();
		assert_eq!(message, clone);
	}

	mod j_son {
		use random_ip;
		use random_name;
		use random_text;
		use rand;
		use cho::*;
		use cho::json::*;


		#[test]
		fn full_transserializes_properly() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let message = ChatMessage::new(ChatUser::get(random_name(&mut rng), random_ip(&mut rng)), random_text(&mut rng));
				let trans = ChatMessage::from_json(message.to_json()).expect("Full transserialization via ChatMessage");
				assert_eq!(message, trans);
			}
		}

		#[test]
		fn ipless_transserializes_properly() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let message = ChatMessage::new(ChatUser::me(random_name(&mut rng)), random_text(&mut rng));;
				let trans = ChatMessage::from_json(message.to_json()).expect("IP-less transserialization via ChatMessage");
				assert_eq!(message, trans);
			}
		}
		#[test]
		fn full_transserializes_properly_through_string() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let message = ChatMessage::new(ChatUser::get(random_name(&mut rng), random_ip(&mut rng)), random_text(&mut rng));
				let message_s = message.to_json_string().expect("Full serialization to string via ChatMessage");
				let trans = ChatMessage::from_json_string(&message_s).expect("Full deserialization from string via ChatMessage");
				assert_eq!(message, trans);
			}
		}

		#[test]
		fn ipless_transserializes_properly_through_string() {
			let mut rng = rand::thread_rng();
			let times = if cfg!(feature = "ci") {100000} else {1000};

			for _ in 1..times {
				let message = ChatMessage::new(ChatUser::me(random_name(&mut rng)), random_text(&mut rng));;
				let message_s = message.to_json_string().expect("IP-less serialization to string via ChatMessage");
				let trans = ChatMessage::from_json_string(&message_s).expect("IP-less deserialization from string via ChatMessage");
				assert_eq!(message, trans);
			}
		}

		#[test]
		//#[should_fail]  // The attribute `should_fail` is currently unknown to the compiler and may have meaning added to it in the future
		fn deserialization_from_malformed_fails() {
			ChatMessage::from_json_string(&"{\"user\": \"you\"}".to_string()).unwrap_err();
		}
	}
}

#[cfg(test)]
mod json_impl {
	use rand;
	use time::Tm;
	use random_time;
	use cho::json::*;


	#[test]
	fn time_transserializes_properly() {
		let mut rng = rand::thread_rng();
		let times = if cfg!(feature = "ci") {100000} else {1000};

		for _ in 1..times {
			let time = random_time(&mut rng);
			let time_s = time.to_json_string().expect("Serialization to string via time::Tm");
			let trans = Tm::from_json_string(&time_s).expect("Deserialization from string via time::Tm");
			assert_eq!(time, trans);
		}
	}
}
