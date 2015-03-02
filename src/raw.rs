extern crate libc;


#[repr(C, packed)]
pub struct bdaddr_t {
	b: [libc::uint8_t; 6],
}

pub static BDADDR_ANY: bdaddr_t = bdaddr_t { b: [0, 0, 0, 0, 0, 0] };
pub static BDADDR_ALL: bdaddr_t = bdaddr_t { b: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff] };
pub static BDADDR_LOCAL: bdaddr_t = bdaddr_t { b: [0, 0, 0, 0xff, 0xff, 0xff] };


#[link(name = "bluetooth")]
extern {

	pub fn hci_get_route(bdaddr: * const bdaddr_t) -> libc::c_int;

}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let _ = unsafe { hci_get_route(&BDADDR_ANY) };
		let _ = unsafe { hci_get_route(&BDADDR_ALL) };
		let _ = unsafe { hci_get_route(&BDADDR_LOCAL) };
	}
}
