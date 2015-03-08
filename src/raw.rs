extern crate libc;
extern crate std;

use std::ffi;
use std::fmt;
use std::str;


#[repr(C, packed)]
#[derive(Copy,Debug)]
pub struct bdaddr_t(pub [libc::uint8_t; 6]);

pub static BDADDR_ANY: bdaddr_t = bdaddr_t([0, 0, 0, 0, 0, 0]);
pub static BDADDR_ALL: bdaddr_t = bdaddr_t([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
pub static BDADDR_LOCAL: bdaddr_t = bdaddr_t([0, 0, 0, 0xff, 0xff, 0xff]);


#[repr(C)]
#[derive(Copy,Debug)]
pub struct hci_version {
	pub manufacturer: libc::uint16_t,
	pub hci_ver: libc::uint8_t,
	pub hci_rev: libc::uint16_t,
	pub lmp_ver: libc::uint8_t,
	pub lmp_subver: libc::uint16_t,
}

impl hci_version {
	pub fn hci_ver_str(&self) -> Option<&'static str> {
		let s = unsafe { hci_vertostr(self.hci_ver as libc::c_uint) };
		if s == 0 as *const libc::c_char {
			None
		} else {
			let buf: &[u8] = unsafe { ffi::CStr::from_ptr(s).to_bytes() };
			str::from_utf8(buf).ok()
		}
	}

	pub fn lmp_ver_str(&self) -> Option<&'static str> {
		let s = unsafe { lmp_vertostr(self.lmp_ver as libc::c_uint) };
		if s == 0 as *const libc::c_char {
			None
		} else {
			let buf: &[u8] = unsafe { ffi::CStr::from_ptr(s).to_bytes() };
			str::from_utf8(buf).ok()
		}
	}
}

#[repr(C)]
pub struct hci_commands(pub [libc::uint8_t; 64]);

impl hci_commands {
	pub fn command_name(command: u32) -> Option<&'static str> {
		let s = unsafe { hci_cmdtostr(command as libc::c_uint) };
		if s == 0 as *const libc::c_char {
			None
		} else {
			let buf: &[u8] = unsafe { ffi::CStr::from_ptr(s).to_bytes() };
			str::from_utf8(buf).ok()
		}
	}
}

impl std::fmt::Debug for hci_commands {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(formatter, "hci_commands {{}}")
	}
}


#[link(name = "bluetooth")]
extern {

	pub fn hci_get_route(bdaddr: * const bdaddr_t) -> libc::c_int;

	pub fn hci_open_dev(dev_id: libc::c_int) -> libc::c_int;
	pub fn hci_close_dev(dd: libc::c_int) -> libc::c_int;

	pub fn hci_vertostr(ver: libc::c_uint) -> *const libc::c_char;
	pub fn lmp_vertostr(ver: libc::c_uint) -> *const libc::c_char;
	pub fn hci_cmdtostr(ver: libc::c_uint) -> *const libc::c_char;

	pub fn hci_read_local_name(dd: libc::c_int, len: libc::c_int, name: *mut libc::c_char, to: libc::c_int) -> libc::c_int;
	pub fn hci_write_local_name(dd: libc::c_int, name: *const libc::c_char, to: libc::c_int) -> libc::c_int;
	pub fn hci_read_local_version(dd: libc::c_int, ver: *mut hci_version, to: libc::c_int) -> libc::c_int;
	pub fn hci_read_local_commands(dd: libc::c_int, commands: *mut hci_commands, to: libc::c_int) -> libc::c_int;
	// pub fn hci_read_local_features(dd: libc::c_int, features: *mut uint8_t, to: libc::c_int) -> libc::c_int;
	// pub fn hci_read_local_ext_features(dd: libc::c_int, page: libc::uint8_t, max_page: *mut libc::uint8_t, features: *mut libc::uint8_t, to: libc::c_int) -> libc::c_int;

	pub fn hci_read_remote_name(dd: libc::c_int, bdaddr: *const bdaddr_t, len: libc::c_int, name: *mut libc::c_char, to: libc::c_int) -> libc::c_int;
	// int hci_read_remote_name_with_clock_offset(int dd, const bdaddr_t *bdaddr, uint8_t pscan_rep_mode, uint16_t clkoffset, int len, char *name, int to);
	// int hci_read_remote_name_cancel(int dd, const bdaddr_t *bdaddr, int to);
	// int hci_read_remote_version(int dd, uint16_t handle, struct hci_version *ver, int to);
	// int hci_read_remote_features(int dd, uint16_t handle, uint8_t *features, int to);
	// int hci_read_remote_ext_features(int dd, uint16_t handle, uint8_t page, uint8_t *max_page, uint8_t *features, int to);
	// int hci_read_clock_offset(int dd, uint16_t handle, uint16_t *clkoffset, int to);


}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let r = unsafe { hci_get_route(&BDADDR_ANY) };
		let _ = unsafe { hci_get_route(&BDADDR_ALL) };
		let _ = unsafe { hci_get_route(&BDADDR_LOCAL) };

		if r > 0 {
			let d = unsafe { hci_open_dev(r) };
			if d > 0 {
				unsafe { hci_close_dev(d) };
			}
		}
	}
}
