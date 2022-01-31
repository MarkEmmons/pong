const CONFIG_FILE: &str = "conf/log4rs.yml";

pub fn init_logger() {

	match log4rs::init_file(CONFIG_FILE, Default::default()) {

		Ok(_) => (),
		Err(e) => panic!("Error initial logger: {}", e),
	}
}
