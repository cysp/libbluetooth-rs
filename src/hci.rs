extern crate libc;

// use raw;
// use common;

use std;
use std::error::{Error};
use std::borrow::ToOwned;

use serialize::hex::ToHex;

use nix;


#[derive(Debug, Clone, PartialEq)]
pub enum HciError {
	Errno(i32),
	Nix(nix::NixError),
	Io(std::io::Error),
}

impl std::error::Error for HciError {
    fn description(&self) -> &str {
        "error"
    }
}

impl std::fmt::Display for HciError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.description().fmt(f)
    }
}

// impl<E: std::error::Error> std::error::FromError<E> for HciError {
//     fn from_error(err: E) -> HciError {
//     	match err {
//     		nix::NixError::Sys(_) => HciError::Nix(err),
//     	}
// 	}
// }


// pub struct HciVersion {
// 	manufacturer: u16,
// 	hci_ver: u8,
// 	hci_rev: u16,
// 	lmp_ver: u8,
// 	lmp_rev: u16,
// };

// impl HciVersion {
// 	pub fn manufacturer(&self) -> u16 {
// 		self.0.manufacturer
// 	}
// 	pub fn manufacturer_str(&self) -> Option<&'static str> {
// 		self.0.manufacturer_str()
// 	}
// 	pub fn hci_ver(&self) -> u8 {
// 		self.0.hci_ver
// 	}
// 	pub fn hci_ver_str(&self) -> Option<&'static str> {
// 		self.0.hci_ver_str()
// 	}
// 	pub fn hci_rev(&self) -> u16 {
// 		self.0.hci_rev
// 	}
// 	pub fn lmp_ver(&self) -> u8 {
// 		self.0.lmp_ver
// 	}
// 	pub fn lmp_ver_str(&self) -> Option<&'static str> {
// 		self.0.lmp_ver_str()
// 	}
// 	pub fn lmp_subver(&self) -> u16 {
// 		self.0.lmp_subver
// 	}
// }

// impl std::fmt::Debug for HciVersion {
// 	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
// 		write!(formatter, "HciVersion {{ manufacturer: {}, hci_ver: {}, hci_rev: {}, lmp_ver: {}, lmp_subver: {} }}",
// 			self.manufacturer_str().unwrap_or(&format!("unknown({})", self.manufacturer())),
// 			self.hci_ver_str().unwrap_or(&format!("unknown({})", self.hci_ver())), self.hci_rev(),
// 			self.lmp_ver_str().unwrap_or(&format!("unknown({})", self.lmp_ver())), self.lmp_subver(),
// 			)
// 	}
// }


// pub struct HciCommands(raw::hci::hci_commands);

// impl HciCommands {
// 	pub fn iter(&self) -> HciCommandIterator {
// 		HciCommandIterator { commands: &self, cur: 0 }
// 	}
// }

// #[derive(Debug)]
// pub enum HciCommand {
// 	Known(u32, &'static str),
// 	Unknown(u32),
// }


// pub struct HciCommandIterator<'a> {
// 	commands: &'a HciCommands,
// 	cur: u32,
// }

// impl<'a> Iterator for HciCommandIterator<'a> {
// 	type Item = HciCommand;

// 	fn next(&mut self) -> Option<HciCommand> {
// 		for i in range(self.cur, 8 * 64) {
// 			let supported: bool = (*((self.commands.0).0.get((i / 8) as usize).unwrap()) & (1 << (i % 8) as u8)) != 0;
// 			if supported {
// 				self.cur = i + 1;
// 				return Some(match raw::hci::hci_commands::command_name(i) {
// 					Some(name) => HciCommand::Known(i, name),
// 					None => HciCommand::Unknown(i),
// 				});
// 			}
// 		}
// 		None
// 	}
// }


pub enum HciLeScanType {
	Passive = 0x00,
	Active = 0x01,
}

pub enum HciLeScanAddressType {
	Public = 0x00,
	Random = 0x01,
	ResolvablePrivateOrPublic = 0x02,
	ResolvablePrivateOrRandom = 0x03,
}

pub enum HciLeScanFilter {
	Default = 0x00,
	Dunno1 = 0x01,
	Dunno2 = 0x02,
	Dunno3 = 0x03,
}


pub struct HciHandle {
	device_number: u16,
	s: libc::c_int,
	// t: std::thread::JoinGuard<'a, u8>,
	// tx: std::sync::mpsc::Sender<u8>,
}


impl HciHandle {

	pub fn new(device_number: u16) -> Result<HciHandle, HciError> {
		let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK|nix::sys::socket::SOCK_CLOEXEC, 1) {
			Ok(s) => s,
			Err(e) => return Err(HciError::Nix(e)),
		};
		
		let sockaddr = nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(device_number), nix::sys::socket::HciChannel::Raw);
		match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(sockaddr)) {
			Ok(()) => (),
			Err(e) => return Err(HciError::Nix(e)),
		}

		// let tb = std::thread::Builder::new().name("Bluetooth HCI".to_string());

		// let (tx, rx) = std::sync::mpsc::channel();

		// let t = match tb.scoped(move || {
		// 	loop {
		// 		let mut buf = [0u8; 256];
		// 		match nix::unistd::read(s, &mut buf) {
		// 			Ok(size) => {
		// 				println!("read {} bytes:", size);
		// 				println!("    {}", buf[0..size].to_hex())
		// 			},
		// 			Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
		// 			Err(e) => panic!("err: {:?}", e),
		// 		}
		// 	}
		// }) {
		// 	Ok(t) => t,
		// 	Err(e) => return Err(HciError::Io(e)),
		// };

		// Ok(HciHandle{ device_number: device_number, s: s, t: t, tx: tx })
		Ok(HciHandle{ device_number: device_number, s: s })
	}

	pub fn reset(&self) -> Result<(), HciError> {
		Ok(())
	}

	// pub fn new<T>(addr: &T) -> Result<HciHandle, HciError> where T: super::ToBdAddr {
	// 	let a = addr.to_bdaddr();

	// 	let d = unsafe { raw::hci::hci_get_route(&a.to_raw()) };
	// 	if d < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	let d = unsafe { raw::hci::hci_open_dev(d) };
	// 	if d < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciDeviceHandle(d))
	// }


	// pub fn read_local_name(&self) -> Result<String, HciError> {
	// 	let mut name = [0 as u8; 248];
	// 	let rv = unsafe { raw::hci::hci_read_local_name(self.0, 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}
	// 
	// 	match String::from_utf8(name.to_owned()) {
	// 		Ok(name) => Ok(name),
	// 		Err(_) => Err(HciError { errno: 0 }),
	// 	}
	// }

	// pub fn write_local_name(&self, name: &str) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_write_local_name(self.0, name.as_bytes().as_ptr() as *const libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn read_local_version(&self) -> Result<HciVersion, HciError> {
	// 	let mut v = raw::hci::hci_version { manufacturer: 0, hci_ver: 0, hci_rev: 0, lmp_ver: 0, lmp_subver: 0 };
	// 	let rv = unsafe { raw::hci::hci_read_local_version(self.0, &mut v, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciVersion(v))
	// }

	// pub fn read_local_commands(&self) -> Result<HciCommands, HciError> {
	// 	let mut c = raw::hci::hci_commands([0u8; 64]);
	// 	let rv = unsafe { raw::hci::hci_read_local_commands(self.0, &mut c, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciCommands(c))
	// }

	// pub fn read_remote_name<T>(&self, addr: &T) -> Result<String, HciError> where T: common::ToBdAddr {
	// 	let a = addr.to_bdaddr();
	// 	let mut name = [0 as u8; 248];
	// 	let rv = unsafe { raw::hci::hci_read_remote_name(self.0, &a.to_raw(), 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	match String::from_utf8(name.to_owned()) {
	// 		Ok(name) => Ok(name),
	// 		Err(_) => Err(HciError { errno: 0 }),
	// 	}
	// }

	// pub fn le_set_scan_enable(&self, enable: bool, filter_dup: bool) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_scan_enable(self.0, enable as libc::uint8_t, filter_dup as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn le_set_scan_parameters(&self, scan_type: HciLeScanType, interval: u16, window: u16, addr_type: HciLeScanAddressType, filter: HciLeScanFilter) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_scan_parameters(self.0, scan_type as libc::uint8_t, interval as libc::uint16_t, window as libc::uint16_t, addr_type as libc::uint8_t, filter as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn le_set_advertise_enable(&self, enable: bool) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_advertise_enable(self.0, enable as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }
}

// impl<'a> Drop for HciHandle<'a> {

// 	fn drop(&mut self) {
// 		let rv = unsafe { libc::close(self.s) };
// 		if rv != 0 {
// 			panic!("failed to close hci socket: {}", std::os::errno())
// 		}
// 	}

// }

// impl std::os::unix::AsRawFd for HciHandle {
//     fn as_raw_fd(&self) -> std::os::unix::Fd {
//         self.s
//     }
// }


#[cfg(test)]
mod tests {
	use super::*;
	// use common;

	#[test]
	fn smoke() {
		if let Ok(d) = HciHandle::new(0) {
			d.reset();
			// let name = d.read_local_name().unwrap();
			// let _ = name;
			// let v = d.read_local_version().unwrap();
			// let _ = v;
			// let c = d.read_local_commands().unwrap();
			// let _ = c.iter();
		}

		// if let Ok(d) = HciDeviceHandle::new(&[0,0,0,0,0,0]) {
		// 	let _ = d;
		// }
	}
}
