extern crate bluetooth;

use bluetooth::*;


fn main() {
	let d = HciDeviceHandle::new(&BDADDR_ANY).unwrap();

	let name = d.read_local_name().unwrap();
	println!("name: {}", name);

	let v = d.read_local_version().unwrap();
	println!("version: {:?}", v);
}
