extern crate libc;
// extern crate core;
extern crate std;

use raw;
use common;

use std::error::{Error};
use std::borrow::ToOwned;


#[derive(Debug, Clone, PartialEq)]
pub struct HciError {
	errno: i32,
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


#[derive(Debug)]
pub struct HciVersion {
	raw: raw::hci_version,
}

impl HciVersion {
	pub fn manufacturer(&self) -> u16 {
		self.raw.manufacturer
	}
	pub fn hci_ver(&self) -> u8 {
		self.raw.hci_ver
	}
	pub fn hci_rev(&self) -> u16 {
		self.raw.hci_rev
	}
	pub fn lmp_ver(&self) -> u8 {
		self.raw.lmp_ver
	}
	pub fn lmp_subver(&self) -> u16 {
		self.raw.lmp_subver
	}
}


pub struct HciDeviceHandle {
	d: libc::c_int,
}

impl HciDeviceHandle {

	pub fn new(addr: &common::ToBdAddr) -> Result<HciDeviceHandle, HciError> {
		let a = addr.to_bdaddr();

		let d = unsafe { raw::hci_get_route(&a.to_raw()) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		let d = unsafe { raw::hci_open_dev(d) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		Ok(HciDeviceHandle { d: d })
	}


	pub fn read_local_name(&self) -> Result<String, HciError> {
		let mut name = [0 as u8; 248];
		let rv = unsafe { raw::hci_read_local_name(self.d, 248, &mut name as *mut _ as *mut libc::c_char, 0) };
		if rv < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		match String::from_utf8(name.to_owned()) {
			Ok(name) => Ok(name),
			Err(_) => Err(HciError { errno: 0 }),
		}
	}

	pub fn write_local_name(&self, name: &str) -> Result<(), HciError> {
		let rv = unsafe { raw::hci_write_local_name(self.d, name.as_bytes().as_ptr() as *const libc::c_char, 0) };
		if rv < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		Ok(())
	}

	pub fn read_local_version(&self) -> Result<HciVersion, HciError> {
		let mut v = raw::hci_version { manufacturer: 0, hci_ver: 0, hci_rev: 0, lmp_ver: 0, lmp_subver: 0 };
		let rv = unsafe { raw::hci_read_local_version(self.d, &mut v, 0) };
		if rv < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		Ok(HciVersion { raw: v })
	}

	pub fn read_remote_name(&self, addr: &common::ToBdAddr) -> Result<String, HciError> {
		let a = addr.to_bdaddr();
		let mut name = [0 as u8; 248];
		let rv = unsafe { raw::hci_read_remote_name(self.d, &a.to_raw(), 248, &mut name as *mut _ as *mut libc::c_char, 0) };
		if rv < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		match String::from_utf8(name.to_owned()) {
			Ok(name) => Ok(name),
			Err(_) => Err(HciError { errno: 0 }),
		}
	}

}

impl Drop for HciDeviceHandle {

	fn drop(&mut self) {
		let rv = unsafe { raw::hci_close_dev(self.d) };
		if rv != 0 {
			panic!("failed to close hci_dev: {}", std::os::errno())
		}
	}

}


#[cfg(test)]
mod tests {
	use super::*;
	use common;

	#[test]
	fn smoke() {
		if let Ok(d) = HciDeviceHandle::new(&common::BDADDR_ANY) {
			let name = d.read_local_name().unwrap();
			let _ = name;
			let v = d.read_local_version().unwrap();
			let _ = v;
		}

		if let Ok(d) = HciDeviceHandle::new(&[0,0,0,0,0,0]) {
			let _ = d;
		}
	}
}
