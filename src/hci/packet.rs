#![allow(unused_imports)]

extern crate libc;

use std;
use std::io;
use std::error::{Error};
use std::borrow::ToOwned;
use std::num::FromPrimitive;

use serialize::hex::ToHex;

use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};


mod consts {
	pub mod packet_type {
		pub const COMMAND: u8 = 0x01;
		pub const ACL_DATA: u8 = 0x01;
		pub const SCO_DATA: u8 = 0x01;
		pub const EVENT: u8 = 0x01;
		pub const VENDOR: u8 = 0x01;
	}
}


// #[derive(Clone,Copy,Debug,PartialEq,Eq)]
// pub enum HciPacketType {
// 	Command,
// 	AclData,
// 	ScoData,
// 	Event,
// 	Vendor,
// }

// impl HciPacketType {
// 	fn from_readable(r: &mut std::io::Read) -> Option<HciPacketType> {
// 		match r.read_u8() {
// 			Ok(packet_type) => match packet_type {
// 				consts::packet_type::COMMAND => Some(HciPacketType::Command),
// 				consts::packet_type::ACL_DATA => Some(HciPacketType::AclData),
// 				consts::packet_type::SCO_DATA => Some(HciPacketType::ScoData),
// 				consts::packet_type::EVENT => Some(HciPacketType::Event),
// 				consts::packet_type::VENDOR => Some(HciPacketType::Vendor),
// 				_ => None,
// 			},
// 			Err(e) => None,
// 		}
// 	}
// }

// #[derive(Debug)]
// pub struct HciCommandPacket<'a> {
// 	opcode: HciOpcode,
// 	parameter_data: &'a [u8],
// }
// // #[derive(Debug)]
// // pub struct HciAclDataPacket<'a> {
// // 	data: &'a [u8],
// // }
// // #[derive(Debug)]
// // pub struct HciScoDataPacket<'a> {
// // 	data: &'a [u8],
// // }
// // #[derive(Debug)]
// // pub struct HciEventPacket<'a> {
// // 	event_code: HciEventCode,
// // 	parameter_data: &'a [u8],
// // }
// // #[derive(Debug)]
// // pub struct HciVendorPacket<'a> {
// // 	data: &'a [u8],
// // }

// #[derive(Debug)]
// pub enum HciPacket<'a> {
// 	Command(HciCommand<'a>),
// 	// AclData(HciAclDataPacket<'a>),
// 	// ScoData(HciScoDataPacket<'a>),
// 	// Event(HciEventPacket<'a>),
// 	// Vendor(HciVendorPacket<'a>),
// }

// impl<'a> HciPacket<'a> {
// 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciPacket<'a>> {
// 		let mut c = std::io::Cursor::new(bytes);
// 		let packet_type: HciPacketType = match HciPacketType::from_readable(&mut c) {
// 			Some(packet_type) => packet_type,
// 			None => return None,
// 		};
// 		match packet_type {
// 			HciPacketType::Command => {
// 				match HciCommandPacket::from_bytes(&bytes[1..]) {
// 					Some(p) => Some(HciPacket::Command(p)),
// 					None => None,
// 				}
// 			},
// 			_ => None,
// 			// HciPacketType::AclData => {
// 			// 	None
// 			// },
// 			// HciPacketType::ScoData => {
// 			// 	None
// 			// },
// 			// HciPacketType::Event => {
// 			// 	match HciEventPacket::from_bytes(&bytes[1..]) {
// 			// 		Some(p) => Some(HciPacket::Event(p)),
// 			// 		None => None,
// 			// 	}
// 			// },
// 			// HciPacketType::Vendor => {
// 			// 	None
// 			// },
// 		}
// 	}
// }


// impl<'a> HciCommandPacket<'a> {
// 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciCommandPacket<'a>> {
// 		let mut c = std::io::Cursor::new(bytes);
// 		let opcode = match HciOpcode::from_readable(&mut c) {
// 			Some(opcode) => opcode,
// 			None => return None,
// 		};
// 		let parameter_data_length = match c.read_u8() {
// 			Ok(length) => length,
// 			Err(e) => return None,
// 		};
// 		let parameter_data = &bytes[(c.position() as usize)..];
// 		if parameter_data.len() == parameter_data_length as usize {
// 			return Some(HciCommandPacket {
// 				opcode: opcode,
// 				parameter_data: parameter_data,
// 			});
// 		}
// 		None
// 	}
// }


// // impl<'a> HciEventPacket<'a> {
// // 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEventPacket<'a>> {
// // 		let mut c = std::io::Cursor::new(bytes);
// // 		let event_code = match HciEventCode::from_readable(&mut c) {
// // 			Some(event_code) => event_code,
// // 			None => return None,
// // 		};
// // 		let parameter_data_length = match c.read_u8() {
// // 			Ok(length) => length,
// // 			Err(e) => return None,
// // 		};
// // 		let parameter_data = &bytes[(c.position() as usize)..];
// // 		if parameter_data.len() == parameter_data_length as usize {
// // 			return Some(HciEventPacket {
// // 				event_code: event_code,
// // 				parameter_data: parameter_data,
// // 			});
// // 		}
// // 		None
// // 	}

// // 	pub fn to_event(&self) -> Option<HciEvent<'a>> {
// // 		match self.event_code {
// // 			HciEventCode::CommandComplete => {
// // 				let mut c = std::io::Cursor::new(self.parameter_data);
// // 				let num_hci_command_packets = match c.read_u8() {
// // 					Ok(num_hci_command_packets) => num_hci_command_packets,
// // 					Err(e) => return None,
// // 				};
// // 				let command_opcode = match c.read_u16::<LittleEndian>() {
// // 					Ok(command_opcode) => command_opcode,
// // 					Err(e) => return None,
// // 				};
// // 				let command_opcode = match HciOpcode::from_u16(command_opcode) {
// // 					Some(command_opcode) => command_opcode,
// // 					None => return None,
// // 				};
// // 				let return_parameter_data = &self.parameter_data[(c.position() as usize)..];
// // 				Some(HciEvent::CommandComplete {
// // 					num_hci_command_packets: num_hci_command_packets,
// // 					command_opcode: command_opcode,
// // 					return_parameter_data: return_parameter_data,
// // 				})
// // 			},
// // 			HciEventCode::CommandStatus => {
// // 				let mut c = std::io::Cursor::new(self.parameter_data);
// // 				let status = match c.read_u8() {
// // 					Ok(status) => status,
// // 					Err(e) => return None,
// // 				};
// // 				let num_hci_command_packets = match c.read_u8() {
// // 					Ok(num_hci_command_packets) => num_hci_command_packets,
// // 					Err(e) => return None,
// // 				};
// // 				let command_opcode = match c.read_u16::<LittleEndian>() {
// // 					Ok(command_opcode) => command_opcode,
// // 					Err(e) => return None,
// // 				};
// // 				let command_opcode = match HciOpcode::from_u16(command_opcode) {
// // 					Some(command_opcode) => command_opcode,
// // 					None => return None,
// // 				};
// // 				Some(HciEvent::CommandStatus {
// // 					status: status,
// // 					num_hci_command_packets: num_hci_command_packets,
// // 					command_opcode: command_opcode,
// // 				})
// // 			},
// // 			HciEventCode::LeMeta(le_subeventcode) => {
// // 				None
// // 				// let mut c = std::io::Cursor::new(self.parameter_data);
// // 				// let subevent_code = match c.read_u8() {
// // 				// 	Ok(subevent_code) => subevent_code,
// // 				// 	Err(e) => return None,
// // 				// };
// // 				// let subevent_code = match HciLeMetaSubeventCode::from_u8(subevent_code) {
// // 				// 	Ok(subevent_code) => subevent_code,
// // 				// 	Err(e) => return None,
// // 				// };
// // 				// match subevent_code {
// // 				// 	HciLeMetaSubeventCode::AdvertisingReport => {
// // 				// 		let mut c = std::io::Cursor::new(self.parameter_data);
// // 				// 		let num_reports = match c.read_u8() {
// // 				// 			Ok(num_reports) => num_reports,
// // 				// 			Err(e) => return None,
// // 				// 		};
// // 				// 	},
// // 				// 	_ => None,
// // 				// }
// // 			},
// // 		}
// // 	}
// // }


// // #[derive(Clone,Copy,Debug)]
// // pub enum HciEvent<'a> {
// // 	CommandComplete{ num_hci_command_packets: u8, command_opcode: HciOpcode, return_parameter_data: &'a [u8] },
// // 	CommandStatus{ status: u8, num_hci_command_packets: u8, command_opcode: HciOpcode },
// // 	LeMeta(HciLeMetaEvent<'a>),
// // }

// // #[derive(Clone,Copy,Debug)]
// // pub enum HciLeMetaEvent<'a> {
// // 	AdvertisingReport{ reports: &'a [HciLeAdvertisingReport<'a>] },
// // }

// // #[derive(Clone,Copy,Debug)]
// // pub struct HciLeAdvertisingReport<'a> {
// // 	pub event_type: u8,
// // 	pub address_type: u8,
// // 	pub address: [u8; 6],
// // 	pub data: &'a [u8],
// // 	pub rssi: u8,
// // }

// // impl<'a> HciEvent<'a> {
// // 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEvent<'a>> {
// // 		None
// // 	}
// // }


// // // pub struct HciHandle {
// // // 	device_number: u16,
// // // 	s: libc::c_int,
// // // 	// t: std::thread::JoinGuard<'a, u8>,
// // // 	// tx: std::sync::mpsc::Sender<u8>,
// // // }

// // // impl HciHandle {

// // // 	pub fn new(device_number: u16) -> Result<HciHandle, HciError> {
// // // 		let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK|nix::sys::socket::SOCK_CLOEXEC, 1) {
// // // 			Ok(s) => s,
// // // 			Err(e) => return Err(HciError::Nix(e)),
// // // 		};
		
// // // 		let sockaddr = nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(device_number), nix::sys::socket::HciChannel::Raw);
// // // 		match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(sockaddr)) {
// // // 			Ok(()) => (),
// // // 			Err(e) => return Err(HciError::Nix(e)),
// // // 		}

// // // 		// let tb = std::thread::Builder::new().name("Bluetooth HCI".to_string());

// // // 		// let (tx, rx) = std::sync::mpsc::channel();

// // // 		// let t = match tb.scoped(move || {
// // // 		// 	loop {
// // // 		// 		let mut buf = [0u8; 256];
// // // 		// 		match nix::unistd::read(s, &mut buf) {
// // // 		// 			Ok(size) => {
// // // 		// 				println!("read {} bytes:", size);
// // // 		// 				println!("    {}", buf[0..size].to_hex())
// // // 		// 			},
// // // 		// 			Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
// // // 		// 			Err(e) => panic!("err: {:?}", e),
// // // 		// 		}
// // // 		// 	}
// // // 		// }) {
// // // 		// 	Ok(t) => t,
// // // 		// 	Err(e) => return Err(HciError::Io(e)),
// // // 		// };

// // // 		// Ok(HciHandle{ device_number: device_number, s: s, t: t, tx: tx })
// // // 		Ok(HciHandle{ device_number: device_number, s: s })
// // // 	}

// // // 	pub fn reset(&self) -> Result<(), HciError> {
// // // 		Ok(())
// // // 	}

// // // 	// pub fn new<T>(addr: &T) -> Result<HciHandle, HciError> where T: super::ToBdAddr {
// // // 	// 	let a = addr.to_bdaddr();

// // // 	// 	let d = unsafe { raw::hci::hci_get_route(&a.to_raw()) };
// // // 	// 	if d < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	let d = unsafe { raw::hci::hci_open_dev(d) };
// // // 	// 	if d < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(HciDeviceHandle(d))
// // // 	// }


// // // 	// pub fn read_local_name(&self) -> Result<String, HciError> {
// // // 	// 	let mut name = [0 as u8; 248];
// // // 	// 	let rv = unsafe { raw::hci::hci_read_local_name(self.0, 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}
// // // 	// 
// // // 	// 	match String::from_utf8(name.to_owned()) {
// // // 	// 		Ok(name) => Ok(name),
// // // 	// 		Err(_) => Err(HciError { errno: 0 }),
// // // 	// 	}
// // // 	// }

// // // 	// pub fn write_local_name(&self, name: &str) -> Result<(), HciError> {
// // // 	// 	let rv = unsafe { raw::hci::hci_write_local_name(self.0, name.as_bytes().as_ptr() as *const libc::c_char, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(())
// // // 	// }

// // // 	// pub fn read_local_version(&self) -> Result<HciVersion, HciError> {
// // // 	// 	let mut v = raw::hci::hci_version { manufacturer: 0, hci_ver: 0, hci_rev: 0, lmp_ver: 0, lmp_subver: 0 };
// // // 	// 	let rv = unsafe { raw::hci::hci_read_local_version(self.0, &mut v, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(HciVersion(v))
// // // 	// }

// // // 	// pub fn read_local_commands(&self) -> Result<HciCommands, HciError> {
// // // 	// 	let mut c = raw::hci::hci_commands([0u8; 64]);
// // // 	// 	let rv = unsafe { raw::hci::hci_read_local_commands(self.0, &mut c, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(HciCommands(c))
// // // 	// }

// // // 	// pub fn read_remote_name<T>(&self, addr: &T) -> Result<String, HciError> where T: common::ToBdAddr {
// // // 	// 	let a = addr.to_bdaddr();
// // // 	// 	let mut name = [0 as u8; 248];
// // // 	// 	let rv = unsafe { raw::hci::hci_read_remote_name(self.0, &a.to_raw(), 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	match String::from_utf8(name.to_owned()) {
// // // 	// 		Ok(name) => Ok(name),
// // // 	// 		Err(_) => Err(HciError { errno: 0 }),
// // // 	// 	}
// // // 	// }

// // // 	// pub fn le_set_scan_enable(&self, enable: bool, filter_dup: bool) -> Result<(), HciError> {
// // // 	// 	let rv = unsafe { raw::hci::hci_le_set_scan_enable(self.0, enable as libc::uint8_t, filter_dup as libc::uint8_t, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(())
// // // 	// }

// // // 	// pub fn le_set_scan_parameters(&self, scan_type: HciLeScanType, interval: u16, window: u16, addr_type: HciLeScanAddressType, filter: HciLeScanFilter) -> Result<(), HciError> {
// // // 	// 	let rv = unsafe { raw::hci::hci_le_set_scan_parameters(self.0, scan_type as libc::uint8_t, interval as libc::uint16_t, window as libc::uint16_t, addr_type as libc::uint8_t, filter as libc::uint8_t, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(())
// // // 	// }

// // // 	// pub fn le_set_advertise_enable(&self, enable: bool) -> Result<(), HciError> {
// // // 	// 	let rv = unsafe { raw::hci::hci_le_set_advertise_enable(self.0, enable as libc::uint8_t, 1000) };
// // // 	// 	if rv < 0 {
// // // 	// 		return Err(HciError { errno: std::os::errno() });
// // // 	// 	}

// // // 	// 	Ok(())
// // // 	// }
// // // }

// // // // impl<'a> Drop for HciHandle<'a> {

// // // // 	fn drop(&mut self) {
// // // // 		let rv = unsafe { libc::close(self.s) };
// // // // 		if rv != 0 {
// // // // 			panic!("failed to close hci socket: {}", std::os::errno())
// // // // 		}
// // // // 	}

// // // // }

// // // // impl std::os::unix::AsRawFd for HciHandle {
// // // //     fn as_raw_fd(&self) -> std::os::unix::Fd {
// // // //         self.s
// // // //     }
// // // // }


// // // #[cfg(test)]
// // // mod tests {
// // // 	use super::*;

// // // 	#[test]
// // // 	fn smoke() {
// // // 		if let Ok(d) = HciHandle::new(0) {
// // // 			d.reset();
// // // 			// let name = d.read_local_name().unwrap();
// // // 			// let _ = name;
// // // 			// let v = d.read_local_version().unwrap();
// // // 			// let _ = v;
// // // 			// let c = d.read_local_commands().unwrap();
// // // 			// let _ = c.iter();
// // // 		}

// // // 		// if let Ok(d) = HciDeviceHandle::new(&[0,0,0,0,0,0]) {
// // // 		// 	let _ = d;
// // // 		// }
// // // 	}
// // // }
