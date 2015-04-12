#![allow(dead_code)]

extern crate rustc_serialize as serialize;

extern crate byteorder;

mod types;
pub use types::*;

pub mod hci;


pub static BDADDR_ANY: BdAddr = BdAddr([0, 0, 0, 0, 0, 0]);
pub static BDADDR_ALL: BdAddr = BdAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
pub static BDADDR_LOCAL: BdAddr = BdAddr([0, 0, 0, 0xff, 0xff, 0xff]);


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let _: BdAddr = [0, 0, 0, 0, 0, 0].into();
		let _: BdAddr = (&[0, 0, 0, 0, 0, 0]).into();
		let a: BdAddr = [0, 0, 0, 0, 0, 0].into();
		assert_eq!(*a, [0, 0, 0, 0, 0, 0]);
	}
}