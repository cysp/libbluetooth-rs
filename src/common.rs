// use super::raw;
extern crate std;

use raw;


#[derive(Copy)]
pub struct BdAddr {
	a: [u8; 6],
}

impl std::marker::Sized for BdAddr {}


impl BdAddr {
	pub fn to_raw(&self) -> raw::hci::bdaddr_t {
		raw::hci::bdaddr_t(self.a)
	}
}


pub trait ToBdAddr {
	fn to_bdaddr(&self) -> BdAddr;
}


impl ToBdAddr for BdAddr {
	fn to_bdaddr(&self) -> BdAddr {
		*self
	}
}

impl ToBdAddr for [u8; 6] {
	fn to_bdaddr(&self) -> BdAddr {
		BdAddr {
			a: *self
		}
	}
}


pub static BDADDR_ANY: BdAddr = BdAddr { a: [0, 0, 0, 0, 0, 0] };
pub static BDADDR_ALL: BdAddr = BdAddr { a: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff] };
pub static BDADDR_LOCAL: BdAddr = BdAddr { a: [0, 0, 0, 0xff, 0xff, 0xff] };
