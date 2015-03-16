extern crate libc;
extern crate serialize;

extern crate byteorder;
extern crate bytes;
extern crate nix;


mod hci;

pub use hci::HciHandle;
pub use hci::{HciOpcode,HciControllerOpcode,HciLeControllerOpcode};
pub use hci::{HciPacket};


#[derive(Copy)]
pub struct BdAddr([u8; 6]);

impl BdAddr {
	pub fn to_raw(&self) -> &[u8; 6] {
		&self.0
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
		BdAddr(*self)
	}
}


pub static BDADDR_ANY: BdAddr = BdAddr([0, 0, 0, 0, 0, 0]);
pub static BDADDR_ALL: BdAddr = BdAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
pub static BDADDR_LOCAL: BdAddr = BdAddr([0, 0, 0, 0xff, 0xff, 0xff]);
