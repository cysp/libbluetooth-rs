extern crate libc;
// extern crate core;
extern crate std;

use raw;
use common;

use std::error::{Error};


#[derive(Debug, Clone, PartialEq)]
pub struct HciError {
	errno: i32,
}

impl std::error::Error for HciError {
    fn description(&self) -> &str {
    	"dunno lol"
    }
}

impl std::fmt::Display for HciError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.description().fmt(f)
    }
}




pub struct HciDeviceHandle {
	d: libc::c_int,
}

impl HciDeviceHandle {

	pub fn new(addr: common::BdAddr) -> Result<HciDeviceHandle, HciError> {
		let a = addr.as_raw();

		let d = unsafe { raw::hci_get_route(&a) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		let d = unsafe { raw::hci_open_dev(d) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		Ok(HciDeviceHandle { d: d })
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
		let d = HciDeviceHandle::new(common::BdAddr::Any).unwrap();
		let _ = d;
	}
}
