extern crate libc;

// use raw;
// use common;

use std;
use std::error::{Error};
use std::borrow::ToOwned;
use std::num::FromPrimitive;

use serialize::hex::ToHex;

use bytes;
use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};
use nix;


#[derive(Debug, Clone, PartialEq)]
pub enum HciError {
	Errno(i32),
	Nix(nix::NixError),
	Io(std::io::Error),
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

// impl<E: std::error::Error> std::error::FromError<E> for HciError {
//     fn from_error(err: E) -> HciError {
//     	match err {
//     		nix::NixError::Sys(_) => HciError::Nix(err),
//     	}
// 	}
// }


// pub struct HciVersion {
// 	manufacturer: u16,
// 	hci_ver: u8,
// 	hci_rev: u16,
// 	lmp_ver: u8,
// 	lmp_rev: u16,
// };

// impl HciVersion {
// 	pub fn manufacturer(&self) -> u16 {
// 		self.0.manufacturer
// 	}
// 	pub fn manufacturer_str(&self) -> Option<&'static str> {
// 		self.0.manufacturer_str()
// 	}
// 	pub fn hci_ver(&self) -> u8 {
// 		self.0.hci_ver
// 	}
// 	pub fn hci_ver_str(&self) -> Option<&'static str> {
// 		self.0.hci_ver_str()
// 	}
// 	pub fn hci_rev(&self) -> u16 {
// 		self.0.hci_rev
// 	}
// 	pub fn lmp_ver(&self) -> u8 {
// 		self.0.lmp_ver
// 	}
// 	pub fn lmp_ver_str(&self) -> Option<&'static str> {
// 		self.0.lmp_ver_str()
// 	}
// 	pub fn lmp_subver(&self) -> u16 {
// 		self.0.lmp_subver
// 	}
// }

// impl std::fmt::Debug for HciVersion {
// 	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
// 		write!(formatter, "HciVersion {{ manufacturer: {}, hci_ver: {}, hci_rev: {}, lmp_ver: {}, lmp_subver: {} }}",
// 			self.manufacturer_str().unwrap_or(&format!("unknown({})", self.manufacturer())),
// 			self.hci_ver_str().unwrap_or(&format!("unknown({})", self.hci_ver())), self.hci_rev(),
// 			self.lmp_ver_str().unwrap_or(&format!("unknown({})", self.lmp_ver())), self.lmp_subver(),
// 			)
// 	}
// }


// pub struct HciCommands(raw::hci::hci_commands);

// impl HciCommands {
// 	pub fn iter(&self) -> HciCommandIterator {
// 		HciCommandIterator { commands: &self, cur: 0 }
// 	}
// }

// #[derive(Debug)]
// pub enum HciCommand {
// 	Known(u32, &'static str),
// 	Unknown(u32),
// }


// pub struct HciCommandIterator<'a> {
// 	commands: &'a HciCommands,
// 	cur: u32,
// }

// impl<'a> Iterator for HciCommandIterator<'a> {
// 	type Item = HciCommand;

// 	fn next(&mut self) -> Option<HciCommand> {
// 		for i in range(self.cur, 8 * 64) {
// 			let supported: bool = (*((self.commands.0).0.get((i / 8) as usize).unwrap()) & (1 << (i % 8) as u8)) != 0;
// 			if supported {
// 				self.cur = i + 1;
// 				return Some(match raw::hci::hci_commands::command_name(i) {
// 					Some(name) => HciCommand::Known(i, name),
// 					None => HciCommand::Unknown(i),
// 				});
// 			}
// 		}
// 		None
// 	}
// }


pub enum HciLeScanType {
	Passive = 0x00,
	Active = 0x01,
}

pub enum HciLeScanAddressType {
	Public = 0x00,
	Random = 0x01,
	ResolvablePrivateOrPublic = 0x02,
	ResolvablePrivateOrRandom = 0x03,
}

pub enum HciLeScanFilter {
	Default = 0x00,
	Dunno1 = 0x01,
	Dunno2 = 0x02,
	Dunno3 = 0x03,
}


#[derive(Copy,Debug,PartialEq,Eq)]
pub enum HciOpcodeGroup {
	Controller = 0x03,
	Informational = 0x04,
	LeController = 0x08,
}

impl HciOpcodeGroup {
	pub fn from_u16(opcode: u16) -> Option<HciOpcodeGroup> {
		match opcode >> 10 {
			0x03 => Some(HciOpcodeGroup::Controller),
			0x04 => Some(HciOpcodeGroup::Informational),
			0x08 => Some(HciOpcodeGroup::LeController),
			_ => None,
		}
	}
}

#[derive(Copy,Debug,PartialEq,Eq)]
pub enum HciOpcode {
	Controller(HciControllerOpcode),
	Informational(HciInformationalOpcode),
	LeController(HciLeControllerOpcode),
}

impl HciOpcode {
	pub fn from_u16(opcode: u16) -> Option<HciOpcode> {
		let group = match HciOpcodeGroup::from_u16(opcode) {
			Some(g) => g,
			None => return None,
		};
		match group {
			HciOpcodeGroup::Controller => match HciControllerOpcode::from_u16(opcode) {
				Some(opcode) => Some(HciOpcode::Controller(opcode)),
				None => None,
			},
			HciOpcodeGroup::Informational => match HciInformationalOpcode::from_u16(opcode) {
				Some(opcode) => Some(HciOpcode::Informational(opcode)),
				None => None,
			},
			HciOpcodeGroup::LeController => match HciLeControllerOpcode::from_u16(opcode) {
				Some(opcode) => Some(HciOpcode::LeController(opcode)),
				None => None,
			},
		}
	}

	pub fn to_u16(&self) -> u16 {
		match self {
			&HciOpcode::Controller(opcode) => opcode.to_u16(),
			&HciOpcode::Informational(opcode) => opcode.to_u16(),
			&HciOpcode::LeController(opcode) => opcode.to_u16(),
		}
	}
}

#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciControllerOpcode {
	SetEventMask = 0x0001,
	Reset = 0x0003,
	SetEventFilter = 0x0005,
	Flush = 0x0008,
	ReadPINType = 0x0009,
	WritePINType = 0x000A,
}

impl HciControllerOpcode {
	pub fn from_u16(opcode: u16) -> Option<HciControllerOpcode> {
		match HciOpcodeGroup::from_u16(opcode) {
			Some(HciOpcodeGroup::Controller) => HciControllerOpcode::from_trusted_u16(opcode),
			_ => None,
		}
	}

	fn from_trusted_u16(opcode: u16) -> Option<HciControllerOpcode> {
		FromPrimitive::from_u16(opcode & ((1 << 10) - 1))
	}

	pub fn to_u16(&self) -> u16 {
		((HciOpcodeGroup::Controller as u16) << 10) | (*self as u16)
	}
}

#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciInformationalOpcode {
	ReadLocalVersionInformation = 0x0001,
	ReadLocalSupportedCommands = 0x0002,
	ReadLocalSupportedFeatures = 0x0003,
	ReadLocalExtendedFeatures = 0x0004,
	ReadBufferSize = 0x0005,
	ReadBdAddr = 0x0009,
	ReadDataBlockSize = 0x000A,
}

impl HciInformationalOpcode {
	pub fn from_u16(opcode: u16) -> Option<HciInformationalOpcode> {
		match HciOpcodeGroup::from_u16(opcode) {
			Some(HciOpcodeGroup::Informational) => HciInformationalOpcode::from_trusted_u16(opcode),
			_ => None,
		}
	}

	fn from_trusted_u16(opcode: u16) -> Option<HciInformationalOpcode> {
		FromPrimitive::from_u16(opcode & ((1 << 10) - 1))
	}

	pub fn to_u16(&self) -> u16 {
		((HciOpcodeGroup::Informational as u16) << 10) | (*self as u16)
	}
}

#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciLeControllerOpcode {
	SetEventMask = 0x0001,
	ReadLocalSupportedFeatures = 0x0003,
	SetAdvertisingParameters = 0x0006,
	SetAdvertisingData = 0x0008,
	SetScanResponseData = 0x0009,
	SetAdvertiseEnable = 0x000A,
	SetScanParameters = 0x000B,
	SetScanEnable = 0x000C,
}

impl HciLeControllerOpcode {
	pub fn from_u16(opcode: u16) -> Option<HciLeControllerOpcode> {
		match HciOpcodeGroup::from_u16(opcode) {
			Some(HciOpcodeGroup::LeController) => HciLeControllerOpcode::from_trusted_u16(opcode),
			_ => None,
		}
	}

	fn from_trusted_u16(opcode: u16) -> Option<HciLeControllerOpcode> {
		FromPrimitive::from_u16(opcode & ((1 << 10) - 1))
	}

	pub fn to_u16(&self) -> u16 {
		((HciOpcodeGroup::LeController as u16) << 10) | (*self as u16)
	}
}


#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciEventCode {
	CommandComplete = 0x0e,
	CommandStatus = 0x0f,
	LeMeta = 0x3e,
}

impl HciEventCode {
	pub fn from_u8(event_code: u8) -> Option<HciEventCode> {
		FromPrimitive::from_u8(event_code)
	}
}

#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciLeMetaSubeventCode {
	ConnectionComplete = 0x01,
	AdvertisingReport = 0x02,
	ConnectionUpdateComplete = 0x03,
	ReadRemoteUsedFeaturesComplete = 0x04,
	LongTermKeyRequest = 0x05,
}

impl HciLeMetaSubeventCode {
	pub fn from_u8(event_code: u8) -> Option<HciLeMetaSubeventCode> {
		FromPrimitive::from_u8(event_code)
	}
}


#[derive(Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciPacketType {
	Command = 0x01,
	AclData = 0x02,
	ScoData = 0x03,
	Event = 0x04,
	Vendor = 0xff,
}

#[derive(Debug)]
pub struct HciCommandPacket<'a> {
	opcode: HciOpcode,
	parameter_data: &'a [u8],
}
#[derive(Debug)]
pub struct HciAclDataPacket<'a> {
	data: &'a [u8],
}
#[derive(Debug)]
pub struct HciScoDataPacket<'a> {
	data: &'a [u8],
}
#[derive(Debug)]
pub struct HciEventPacket<'a> {
	event_code: HciEventCode,
	parameter_data: &'a [u8],
}
#[derive(Debug)]
pub struct HciVendorPacket<'a> {
	data: &'a [u8],
}

#[derive(Debug)]
pub enum HciPacket<'a> {
	Command(HciCommandPacket<'a>),
	AclData(HciAclDataPacket<'a>),
	ScoData(HciScoDataPacket<'a>),
	Event(HciEventPacket<'a>),
	Vendor(HciVendorPacket<'a>),
}

impl<'a> HciPacket<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciPacket<'a>> {
		let mut c = std::io::Cursor::new(bytes);
		let packet_type = match c.read_u8() {
			Ok(packet_type) => packet_type,
			Err(e) => return None,
		};
		let packet_type: HciPacketType = match FromPrimitive::from_u8(packet_type) {
			Some(packet_type) => packet_type,
			None => return None,
		};
		match packet_type {
			HciPacketType::Command => {
				match HciCommandPacket::from_bytes(&bytes[1..]) {
					Some(p) => Some(HciPacket::Command(p)),
					None => None,
				}
			},
			HciPacketType::AclData => {
				None
			},
			HciPacketType::ScoData => {
				None
			},
			HciPacketType::Event => {
				match HciEventPacket::from_bytes(&bytes[1..]) {
					Some(p) => Some(HciPacket::Event(p)),
					None => None,
				}
			},
			HciPacketType::Vendor => {
				None
			},
		}
	}
}


impl<'a> HciCommandPacket<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciCommandPacket<'a>> {
		let mut c = std::io::Cursor::new(bytes);
		let opcode = match c.read_u16::<LittleEndian>() {
			Ok(opcode) => opcode,
			Err(e) => return None,
		};
		let opcode = match HciOpcode::from_u16(opcode) {
			Some(opcode) => opcode,
			None => return None,
		};
		let parameter_data_length = match c.read_u8() {
			Ok(length) => length,
			Err(e) => return None,
		};
		let parameter_data = &bytes[(c.position() as usize)..];
		if parameter_data.len() == parameter_data_length as usize {
			return Some(HciCommandPacket {
				opcode: opcode,
				parameter_data: parameter_data,
			});
		}
		None
	}
}


impl<'a> HciEventPacket<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEventPacket<'a>> {
		let mut c = std::io::Cursor::new(bytes);
		let event_code = match c.read_u8() {
			Ok(event_code) => event_code,
			Err(e) => return None,
		};
		let event_code = match HciEventCode::from_u8(event_code) {
			Some(event_code) => event_code,
			None => return None,
		};
		let parameter_data_length = match c.read_u8() {
			Ok(length) => length,
			Err(e) => return None,
		};
		let parameter_data = &bytes[(c.position() as usize)..];
		if parameter_data.len() == parameter_data_length as usize {
			return Some(HciEventPacket {
				event_code: event_code,
				parameter_data: parameter_data,
			});
		}
		None
	}

	pub fn to_event(&self) -> Option<HciEvent<'a>> {
		match self.event_code {
			HciEventCode::CommandComplete => {
				let mut c = std::io::Cursor::new(self.parameter_data);
				let num_hci_command_packets = match c.read_u8() {
					Ok(num_hci_command_packets) => num_hci_command_packets,
					Err(e) => return None,
				};
				let command_opcode = match c.read_u16::<LittleEndian>() {
					Ok(command_opcode) => command_opcode,
					Err(e) => return None,
				};
				let command_opcode = match HciOpcode::from_u16(command_opcode) {
					Some(command_opcode) => command_opcode,
					None => return None,
				};
				let return_parameter_data = &self.parameter_data[(c.position() as usize)..];
				Some(HciEvent::CommandComplete {
					num_hci_command_packets: num_hci_command_packets,
					command_opcode: command_opcode,
					return_parameter_data: return_parameter_data,
				})
			},
			HciEventCode::CommandStatus => {
				let mut c = std::io::Cursor::new(self.parameter_data);
				let status = match c.read_u8() {
					Ok(status) => status,
					Err(e) => return None,
				};
				let num_hci_command_packets = match c.read_u8() {
					Ok(num_hci_command_packets) => num_hci_command_packets,
					Err(e) => return None,
				};
				let command_opcode = match c.read_u16::<LittleEndian>() {
					Ok(command_opcode) => command_opcode,
					Err(e) => return None,
				};
				let command_opcode = match HciOpcode::from_u16(command_opcode) {
					Some(command_opcode) => command_opcode,
					None => return None,
				};
				Some(HciEvent::CommandStatus {
					status: status,
					num_hci_command_packets: num_hci_command_packets,
					command_opcode: command_opcode,
				})
			},
			HciEventCode::LeMeta => {
				None
				// let mut c = std::io::Cursor::new(self.parameter_data);
				// let subevent_code = match c.read_u8() {
				// 	Ok(subevent_code) => subevent_code,
				// 	Err(e) => return None,
				// };
				// let subevent_code = match HciLeMetaSubeventCode::from_u8(subevent_code) {
				// 	Ok(subevent_code) => subevent_code,
				// 	Err(e) => return None,
				// };
				// match subevent_code {
				// 	HciLeMetaSubeventCode::AdvertisingReport => {
				// 		let mut c = std::io::Cursor::new(self.parameter_data);
				// 		let num_reports = match c.read_u8() {
				// 			Ok(num_reports) => num_reports,
				// 			Err(e) => return None,
				// 		};
				// 	},
				// 	_ => None,
				// }
			},
		}
	}
}


#[derive(Copy,Debug)]
pub enum HciEvent<'a> {
	CommandComplete{ num_hci_command_packets: u8, command_opcode: HciOpcode, return_parameter_data: &'a [u8] },
	CommandStatus{ status: u8, num_hci_command_packets: u8, command_opcode: HciOpcode },
	LeMeta(HciLeMetaEvent<'a>),
}

#[derive(Copy,Debug)]
pub enum HciLeMetaEvent<'a> {
	AdvertisingReport{ reports: &'a [HciLeAdvertisingReport<'a>] },
}

#[derive(Copy,Debug)]
pub struct HciLeAdvertisingReport<'a> {
	pub event_type: u8,
	pub address_type: u8,
	pub address: [u8; 6],
	pub data: &'a [u8],
	pub rssi: u8,
}

impl<'a> HciEvent<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEvent<'a>> {
		None
	}
}


pub struct HciHandle {
	device_number: u16,
	s: libc::c_int,
	// t: std::thread::JoinGuard<'a, u8>,
	// tx: std::sync::mpsc::Sender<u8>,
}

impl HciHandle {

	pub fn new(device_number: u16) -> Result<HciHandle, HciError> {
		let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK|nix::sys::socket::SOCK_CLOEXEC, 1) {
			Ok(s) => s,
			Err(e) => return Err(HciError::Nix(e)),
		};
		
		let sockaddr = nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(device_number), nix::sys::socket::HciChannel::Raw);
		match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(sockaddr)) {
			Ok(()) => (),
			Err(e) => return Err(HciError::Nix(e)),
		}

		// let tb = std::thread::Builder::new().name("Bluetooth HCI".to_string());

		// let (tx, rx) = std::sync::mpsc::channel();

		// let t = match tb.scoped(move || {
		// 	loop {
		// 		let mut buf = [0u8; 256];
		// 		match nix::unistd::read(s, &mut buf) {
		// 			Ok(size) => {
		// 				println!("read {} bytes:", size);
		// 				println!("    {}", buf[0..size].to_hex())
		// 			},
		// 			Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
		// 			Err(e) => panic!("err: {:?}", e),
		// 		}
		// 	}
		// }) {
		// 	Ok(t) => t,
		// 	Err(e) => return Err(HciError::Io(e)),
		// };

		// Ok(HciHandle{ device_number: device_number, s: s, t: t, tx: tx })
		Ok(HciHandle{ device_number: device_number, s: s })
	}

	pub fn reset(&self) -> Result<(), HciError> {
		Ok(())
	}

	// pub fn new<T>(addr: &T) -> Result<HciHandle, HciError> where T: super::ToBdAddr {
	// 	let a = addr.to_bdaddr();

	// 	let d = unsafe { raw::hci::hci_get_route(&a.to_raw()) };
	// 	if d < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	let d = unsafe { raw::hci::hci_open_dev(d) };
	// 	if d < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciDeviceHandle(d))
	// }


	// pub fn read_local_name(&self) -> Result<String, HciError> {
	// 	let mut name = [0 as u8; 248];
	// 	let rv = unsafe { raw::hci::hci_read_local_name(self.0, 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}
	// 
	// 	match String::from_utf8(name.to_owned()) {
	// 		Ok(name) => Ok(name),
	// 		Err(_) => Err(HciError { errno: 0 }),
	// 	}
	// }

	// pub fn write_local_name(&self, name: &str) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_write_local_name(self.0, name.as_bytes().as_ptr() as *const libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn read_local_version(&self) -> Result<HciVersion, HciError> {
	// 	let mut v = raw::hci::hci_version { manufacturer: 0, hci_ver: 0, hci_rev: 0, lmp_ver: 0, lmp_subver: 0 };
	// 	let rv = unsafe { raw::hci::hci_read_local_version(self.0, &mut v, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciVersion(v))
	// }

	// pub fn read_local_commands(&self) -> Result<HciCommands, HciError> {
	// 	let mut c = raw::hci::hci_commands([0u8; 64]);
	// 	let rv = unsafe { raw::hci::hci_read_local_commands(self.0, &mut c, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(HciCommands(c))
	// }

	// pub fn read_remote_name<T>(&self, addr: &T) -> Result<String, HciError> where T: common::ToBdAddr {
	// 	let a = addr.to_bdaddr();
	// 	let mut name = [0 as u8; 248];
	// 	let rv = unsafe { raw::hci::hci_read_remote_name(self.0, &a.to_raw(), 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	match String::from_utf8(name.to_owned()) {
	// 		Ok(name) => Ok(name),
	// 		Err(_) => Err(HciError { errno: 0 }),
	// 	}
	// }

	// pub fn le_set_scan_enable(&self, enable: bool, filter_dup: bool) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_scan_enable(self.0, enable as libc::uint8_t, filter_dup as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn le_set_scan_parameters(&self, scan_type: HciLeScanType, interval: u16, window: u16, addr_type: HciLeScanAddressType, filter: HciLeScanFilter) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_scan_parameters(self.0, scan_type as libc::uint8_t, interval as libc::uint16_t, window as libc::uint16_t, addr_type as libc::uint8_t, filter as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }

	// pub fn le_set_advertise_enable(&self, enable: bool) -> Result<(), HciError> {
	// 	let rv = unsafe { raw::hci::hci_le_set_advertise_enable(self.0, enable as libc::uint8_t, 1000) };
	// 	if rv < 0 {
	// 		return Err(HciError { errno: std::os::errno() });
	// 	}

	// 	Ok(())
	// }
}

// impl<'a> Drop for HciHandle<'a> {

// 	fn drop(&mut self) {
// 		let rv = unsafe { libc::close(self.s) };
// 		if rv != 0 {
// 			panic!("failed to close hci socket: {}", std::os::errno())
// 		}
// 	}

// }

// impl std::os::unix::AsRawFd for HciHandle {
//     fn as_raw_fd(&self) -> std::os::unix::Fd {
//         self.s
//     }
// }


#[cfg(test)]
mod tests {
	use super::*;
	// use common;

	#[test]
	fn smoke() {
		if let Ok(d) = HciHandle::new(0) {
			d.reset();
			// let name = d.read_local_name().unwrap();
			// let _ = name;
			// let v = d.read_local_version().unwrap();
			// let _ = v;
			// let c = d.read_local_commands().unwrap();
			// let _ = c.iter();
		}

		// if let Ok(d) = HciDeviceHandle::new(&[0,0,0,0,0,0]) {
		// 	let _ = d;
		// }
	}
}
