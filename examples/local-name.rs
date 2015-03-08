extern crate bluetooth;

use bluetooth::{HciDeviceHandle, BDADDR_ANY};


fn main() {
	let d = match HciDeviceHandle::new(&BDADDR_ANY) {
		Ok(d) => d,
		Err(_) => return (),
	};

	if let Ok(name) = d.read_local_name() {
		println!("name: {}", name);
	}

	if let Ok(version) = d.read_local_version() {
		println!("version: {:?}", version);
	}

	if let Ok(commands) = d.read_local_commands() {
		println!("commands:");
		for command in commands.names() {
			println!("  {:?}", command);
		}
	}
}
