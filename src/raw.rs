extern crate libc;

use common;


#[repr(C, packed)]
#[derive(Copy)]
pub struct bdaddr_t {
	b: [libc::uint8_t; 6],
}

pub static BDADDR_ANY: bdaddr_t = bdaddr_t { b: [0, 0, 0, 0, 0, 0] };
pub static BDADDR_ALL: bdaddr_t = bdaddr_t { b: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff] };
pub static BDADDR_LOCAL: bdaddr_t = bdaddr_t { b: [0, 0, 0, 0xff, 0xff, 0xff] };

impl common::BdAddr {

	pub fn as_raw(&self) -> bdaddr_t {
		match self {
			&common::BdAddr::Any => BDADDR_ANY,
			&common::BdAddr::All => BDADDR_ALL,
			&common::BdAddr::Local => BDADDR_LOCAL,
			&common::BdAddr::Addr(a, b, c, d, e, f) => bdaddr_t { b: [a as u8, b as u8, c as u8, d as u8, e as u8, f as u8] },
		}
	}

}


#[link(name = "bluetooth")]
extern {

	pub fn hci_get_route(bdaddr: * const bdaddr_t) -> libc::c_int;


	pub fn hci_open_dev(dev_id: libc::c_int) -> libc::c_int;
	pub fn hci_close_dev(dd: libc::c_int) -> libc::c_int;

}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let r = unsafe { hci_get_route(&BDADDR_ANY) };
		let _ = unsafe { hci_get_route(&BDADDR_ALL) };
		let _ = unsafe { hci_get_route(&BDADDR_LOCAL) };

		if (r > 0) {
			let d = unsafe { hci_open_dev(r) };
			if (d > 0) {
				unsafe { hci_close_dev(d) };
			}
		}
	}
}
