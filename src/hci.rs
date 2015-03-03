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




pub struct HciDevice {
	d: libc::c_int,
}

impl HciDevice {

	pub fn new(addr: common::BdAddr) -> Result<HciDevice, HciError> {
		let a = addr.as_raw();

		let d = unsafe { raw::hci_get_route(&a) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		let d = unsafe { raw::hci_open_dev(d) };
		if d < 0 {
			return Err(HciError { errno: std::os::errno() });
		}

		Ok(HciDevice { d: d })
	}

}

impl Drop for HciDevice {

	fn drop(&mut self) {
		let rv = unsafe { raw::hci_close_dev(self.d) };
		if rv != 0 {
			panic!("failed to close hci_dev: {}", std::os::errno())
		}
	}

}
