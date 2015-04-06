#![allow(unused_imports)]

extern crate libc;

// use raw;
// use common;

use std;
use std::io;
use std::error::{Error};
use std::borrow::ToOwned;
use std::num::FromPrimitive;

use serialize::hex::ToHex;

use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};


// #[derive(Debug, Clone, PartialEq)]
// pub enum HciError {
// 	Errno(i32),
// 	Nix(nix::NixError),
// 	Io(std::io::Error),
// }

// impl std::error::Error for HciError {
//     fn description(&self) -> &str {
//         "error"
//     }
// }

// impl std::fmt::Display for HciError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         self.description().fmt(f)
//     }
// }

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


// pub enum HciLeScanType {
// 	Passive = 0x00,
// 	Active = 0x01,
// }

// pub enum HciLeScanAddressType {
// 	Public = 0x00,
// 	Random = 0x01,
// 	ResolvablePrivateOrPublic = 0x02,
// 	ResolvablePrivateOrRandom = 0x03,
// }

// pub enum HciLeScanFilter {
// 	Default = 0x00,
// 	Dunno1 = 0x01,
// 	Dunno2 = 0x02,
// 	Dunno3 = 0x03,
// }


mod consts {
	pub mod opcode {
		pub mod nop {
			pub const OGF: u16 = 0x00;

			pub const NOP: u16 = 0x0000;
		}

		pub mod link_control {
			pub const OGF: u16 = 0x01;

			pub const INQUIRY: u16 = 0x0001;
			pub const INQUIRY_CANCEL: u16 = 0x0002;
			pub const PERIODIC_INQUIRY_MODE: u16 = 0x0003;
			pub const EXIT_PERIODIC_INQUIRY_MODE: u16 = 0x0004;
			pub const CREATE_CONNECTION: u16 = 0x0005;
			pub const DISCONNECT: u16 = 0x0006;
			// …
		}

		pub mod link_policy {
			pub const OGF: u16 = 0x02;
		}

		pub mod controller {
			pub const OGF: u16 = 0x03;

			pub const SET_EVENT_MASK: u16 = 0x0001;
			pub const RESET: u16 = 0x0003;
			pub const SET_EVENT_FILTER: u16 = 0x0005;
			pub const FLUSH: u16 = 0x0008;
			pub const READ_PIN_TYPE: u16 = 0x0009;
			pub const WRITE_PIN_TYPE: u16 = 0x000A;
		}

		pub mod informational {
			pub const OGF: u16 = 0x04;

			pub const READ_LOCAL_VERSION_INFORMATION: u16 = 0x0001;
			pub const READ_LOCAL_SUPPORTED_COMMANDS: u16 = 0x0002;
			pub const READ_LOCAL_SUPPORTED_FEATURES: u16 = 0x0003;
			pub const READ_LOCAL_EXTENDED_FEATURES: u16 = 0x0004;
			pub const READ_BUFFER_SIZE: u16 = 0x0005;
			pub const READ_BD_ADDR: u16 = 0x0009;
			pub const READ_DATA_BLOCK_SIZE: u16 = 0x000A;
		}

		pub mod status_parameters {
			pub const OGF: u16 = 0x05;

			pub const READ_FAILED_CONTACT_COUNTER: u16 = 0x0001;
			pub const RESET_FAILED_CONTACT_COUNTER: u16 = 0x0002;
			pub const READ_LINK_QUALITY: u16 = 0x0003;
			pub const READ_RSSI: u16 = 0x0005;
			pub const READ_AFH_CHANNEL_MAP: u16 = 0x0006;
			pub const READ_CLOCK: u16 = 0x0007;
			pub const READ_ENCRYPTION_KEY_SIZE: u16 = 0x0008;
			pub const READ_LOCAL_AMP_INFO: u16 = 0x0009;
			pub const READ_LOCAL_AMP_ASSOC: u16 = 0x000A;
			pub const WRITE_REMOTE_AMP_ASSOC: u16 = 0x000B;
		}

		pub mod testing {
			pub const OGF: u16 = 0x06;
		}

		pub mod le_controller {
			pub const OGF: u16 = 0x08;

			pub const SET_EVENT_MASK: u16 = 0x0001;
			pub const READ_LOCAL_SUPPORTED_FEATURES: u16 = 0x0003;
			pub const SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
			pub const SET_ADVERTISING_DATA: u16 = 0x0008;
			pub const SET_SCAN_RESPONSE_DATA: u16 = 0x0009;
			pub const SET_ADVERTISE_ENABLE: u16 = 0x000A;
			pub const SET_SCAN_PARAMETERS: u16 = 0x000B;
			pub const SET_SCAN_ENABLE: u16 = 0x000C;
		}

		pub mod vendor {
			pub const OGF: u16 = 0x3F;
		}
	}

	pub mod event_code {
		pub const COMMAND_COMPLETE: u8 = 0x0E;
		pub const COMMAND_STATUS: u8 = 0x0F;
		pub const LE_META: u8 = 0x3E;

		pub mod le_meta {
			pub const CONNECTION_COMPLETE: u8 = 0x01;
			pub const ADVERTISING_REPORT: u8 = 0x02;
			pub const CONNECTION_UPDATE_COMPLETE: u8 = 0x03;
			pub const READ_REMOTE_USED_FEATURES_COMPLETE: u8 = 0x04;
			pub const LONG_TERM_KEY_REQUEST: u8 = 0x05;
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciOpcodeGroup {
	Nop,
	LinkControl,
	LinkPolicy,
	Controller,
	Informational,
	StatusParameters,
	Testing,
	LeController,
	Vendor,
	Unknown(u16),
}

impl From<u16> for HciOpcodeGroup {
	fn from(value: u16) -> HciOpcodeGroup {
		match value >> 10 {
			consts::opcode::nop::OGF => HciOpcodeGroup::Nop,
			consts::opcode::link_control::OGF => HciOpcodeGroup::LinkControl,
			consts::opcode::link_policy::OGF => HciOpcodeGroup::LinkPolicy,
			consts::opcode::controller::OGF => HciOpcodeGroup::Controller,
			consts::opcode::informational::OGF => HciOpcodeGroup::Informational,
			consts::opcode::status_parameters::OGF => HciOpcodeGroup::StatusParameters,
			consts::opcode::testing::OGF => HciOpcodeGroup::Testing,
			consts::opcode::le_controller::OGF => HciOpcodeGroup::LeController,
			consts::opcode::vendor::OGF => HciOpcodeGroup::Vendor,
			unknown_value => HciOpcodeGroup::Unknown(unknown_value),
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciOpcode {
	Nop(HciNopOpcode),
	LinkControl(HciLinkControlOpcode),
	LinkPolicy(HciLinkPolicyOpcode),
	Controller(HciControllerOpcode),
	Informational(HciInformationalOpcode),
	StatusParameters(HciStatusParametersOpcode),
	Testing(HciTestingOpcode),
	LeController(HciLeControllerOpcode),
	Vendor(HciVendorOpcode),
	Unknown(u16),
}

impl HciOpcode {
	pub fn from_readable(r: &mut std::io::Read) -> Option<HciOpcode> {
		let opcode = match r.read_u16::<LittleEndian>() {
			Ok(opcode) => opcode,
			Err(e) => return None,
		};
		Some(HciOpcode::from(opcode))
	}
}

impl From<u16> for HciOpcode {
	fn from(value: u16) -> HciOpcode {
		let ocf = value & ((1 << 10) - 1);
		match HciOpcodeGroup::from(value) {
			HciOpcodeGroup::Nop => HciOpcode::Nop(HciNopOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::LinkControl => HciOpcode::LinkControl(HciLinkControlOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::LinkPolicy => HciOpcode::LinkPolicy(HciLinkPolicyOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::Controller => HciOpcode::Controller(HciControllerOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::Informational => HciOpcode::Informational(HciInformationalOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::StatusParameters => HciOpcode::StatusParameters(HciStatusParametersOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::Testing => HciOpcode::Testing(HciTestingOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::LeController => HciOpcode::LeController(HciLeControllerOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::Vendor => HciOpcode::Vendor(HciVendorOpcode::from_trusted_u16(ocf)),
			HciOpcodeGroup::Unknown(value) => return HciOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciOpcode {
	fn into(self) -> u16 {
		match self {
			HciOpcode::Nop(opcode) => opcode.into(),
			HciOpcode::LinkControl(opcode) => opcode.into(),
			HciOpcode::LinkPolicy(opcode) => opcode.into(),
			HciOpcode::Controller(opcode) => opcode.into(),
			HciOpcode::Informational(opcode) => opcode.into(),
			HciOpcode::StatusParameters(opcode) => opcode.into(),
			HciOpcode::Testing(opcode) => opcode.into(),
			HciOpcode::LeController(opcode) => opcode.into(),
			HciOpcode::Vendor(opcode) => opcode.into(),
			HciOpcode::Unknown(opcode) => opcode,
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciNopOpcode {
	Nop,
	Unknown(u16),
}

impl HciNopOpcode {
	fn from_trusted_u16(value: u16) -> HciNopOpcode {
		match value & ((1 << 10) - 1) {
			0 => HciNopOpcode::Nop,
			value => HciNopOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciNopOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciNopOpcode::Nop => consts::opcode::nop::NOP,
			HciNopOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::nop::OGF << 10) | ocf
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciLinkControlOpcode {
	Unknown(u16),
}

impl HciLinkControlOpcode {
	fn from_trusted_u16(value: u16) -> HciLinkControlOpcode {
		match value & ((1 << 10) - 1) {
			value => HciLinkControlOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciLinkControlOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciLinkControlOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::link_control::OGF << 10) | ocf
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciLinkPolicyOpcode {
	Unknown(u16),
}

impl HciLinkPolicyOpcode {
	fn from_trusted_u16(value: u16) -> HciLinkPolicyOpcode {
		match value & ((1 << 10) - 1) {
			value => HciLinkPolicyOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciLinkPolicyOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciLinkPolicyOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::link_policy::OGF << 10) | ocf
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciControllerOpcode {
	SetEventMask,
	Reset,
	SetEventFilter,
	Flush,
	ReadPINType,
	WritePINType,
	Unknown(u16),
}

impl HciControllerOpcode {
	fn from_trusted_u16(value: u16) -> HciControllerOpcode {
		match value & ((1 << 10) - 1) {
			consts::opcode::controller::SET_EVENT_MASK => HciControllerOpcode::SetEventMask,
			consts::opcode::controller::RESET => HciControllerOpcode::Reset,
			consts::opcode::controller::SET_EVENT_FILTER => HciControllerOpcode::SetEventFilter,
			consts::opcode::controller::FLUSH => HciControllerOpcode::Flush,
			consts::opcode::controller::READ_PIN_TYPE => HciControllerOpcode::ReadPINType,
			consts::opcode::controller::WRITE_PIN_TYPE => HciControllerOpcode::WritePINType,
			value => HciControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciControllerOpcode::SetEventMask => consts::opcode::controller::SET_EVENT_MASK,
			HciControllerOpcode::Reset => consts::opcode::controller::RESET,
			HciControllerOpcode::SetEventFilter => consts::opcode::controller::SET_EVENT_FILTER,
			HciControllerOpcode::Flush => consts::opcode::controller::FLUSH,
			HciControllerOpcode::ReadPINType => consts::opcode::controller::READ_PIN_TYPE,
			HciControllerOpcode::WritePINType => consts::opcode::controller::WRITE_PIN_TYPE,
			HciControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::controller::OGF << 10) | ocf
	}
}

#[cfg(test)]
mod hcicontrolleropcode_tests {
	use super::*;

	#[test]
	fn test_hcicontrolleropcode_seteventmask() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::SetEventMask).into();
			assert_eq!(oc, 0x0C01);
			let oc: u16 = HciControllerOpcode::SetEventMask.into();
			assert_eq!(oc, 0x0C01);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::SetEventMask));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_reset() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::Reset).into();
			assert_eq!(oc, 0x0C03);
			let oc: u16 = HciControllerOpcode::Reset.into();
			assert_eq!(oc, 0x0C03);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::Reset));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_seteventfilter() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::SetEventFilter).into();
			assert_eq!(oc, 0x0C05);
			let oc: u16 = HciControllerOpcode::SetEventFilter.into();
			assert_eq!(oc, 0x0C05);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::SetEventFilter));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_flush() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::Flush).into();
			assert_eq!(oc, 0x0C08);
			let oc: u16 = HciControllerOpcode::Flush.into();
			assert_eq!(oc, 0x0C08);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::Flush));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_readpintype() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::ReadPINType).into();
			assert_eq!(oc, 0x0C09);
			let oc: u16 = HciControllerOpcode::ReadPINType.into();
			assert_eq!(oc, 0x0C09);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::ReadPINType));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_writepintype() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::WritePINType).into();
			assert_eq!(oc, 0x0C0A);
			let oc: u16 = HciControllerOpcode::WritePINType.into();
			assert_eq!(oc, 0x0C0A);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::WritePINType));
		}
	}

	#[test]
	fn test_hcicontrolleropcode_unknown() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::Unknown(0x67)).into();
			assert_eq!(oc, 0x0C67);
			let oc: u16 = HciControllerOpcode::Unknown(0x67).into();
			assert_eq!(oc, 0x0C67);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::Unknown(0x67)));
		}
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciInformationalOpcode {
	ReadLocalVersionInformation,
	ReadLocalSupportedCommands,
	ReadLocalSupportedFeatures,
	ReadLocalExtendedFeatures,
	ReadBufferSize,
	ReadBdAddr,
	ReadDataBlockSize,
	Unknown(u16),
}

impl HciInformationalOpcode {
	fn from_trusted_u16(value: u16) -> HciInformationalOpcode {
		match value & ((1 << 10) - 1) {
			consts::opcode::informational::READ_LOCAL_VERSION_INFORMATION => HciInformationalOpcode::ReadLocalVersionInformation,
			consts::opcode::informational::READ_LOCAL_SUPPORTED_COMMANDS => HciInformationalOpcode::ReadLocalSupportedCommands,
			consts::opcode::informational::READ_LOCAL_SUPPORTED_FEATURES => HciInformationalOpcode::ReadLocalSupportedFeatures,
			consts::opcode::informational::READ_LOCAL_EXTENDED_FEATURES => HciInformationalOpcode::ReadLocalExtendedFeatures,
			consts::opcode::informational::READ_BUFFER_SIZE => HciInformationalOpcode::ReadBufferSize,
			consts::opcode::informational::READ_BD_ADDR => HciInformationalOpcode::ReadBdAddr,
			consts::opcode::informational::READ_DATA_BLOCK_SIZE => HciInformationalOpcode::ReadDataBlockSize,
			value => HciInformationalOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciInformationalOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciInformationalOpcode::ReadLocalVersionInformation => consts::opcode::informational::READ_LOCAL_VERSION_INFORMATION,
			HciInformationalOpcode::ReadLocalSupportedCommands => consts::opcode::informational::READ_LOCAL_SUPPORTED_COMMANDS,
			HciInformationalOpcode::ReadLocalSupportedFeatures => consts::opcode::informational::READ_LOCAL_SUPPORTED_FEATURES,
			HciInformationalOpcode::ReadLocalExtendedFeatures => consts::opcode::informational::READ_LOCAL_EXTENDED_FEATURES,
			HciInformationalOpcode::ReadBufferSize => consts::opcode::informational::READ_BUFFER_SIZE,
			HciInformationalOpcode::ReadBdAddr => consts::opcode::informational::READ_BD_ADDR,
			HciInformationalOpcode::ReadDataBlockSize => consts::opcode::informational::READ_DATA_BLOCK_SIZE,
			HciInformationalOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::informational::OGF << 10) | ocf
	}
}

#[cfg(test)]
mod hciinformationalopcode_tests {
	use super::*;

	#[test]
	fn test_hciinformationalopcode_readlocalversioninformation() {
		{
			let oc: u16 = HciOpcode::Informational(HciInformationalOpcode::ReadLocalVersionInformation).into();
			assert_eq!(oc, 0x1001);
			let oc: u16 = HciInformationalOpcode::ReadLocalVersionInformation.into();
			assert_eq!(oc, 0x1001);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Informational(HciInformationalOpcode::ReadLocalVersionInformation));
		}
	}

	#[test]
	fn test_hciinformationalopcode_readlocalsupportedcommands() {
		{
			let oc: u16 = HciOpcode::Informational(HciInformationalOpcode::ReadLocalSupportedCommands).into();
			assert_eq!(oc, 0x1002);
			let oc: u16 = HciInformationalOpcode::ReadLocalSupportedCommands.into();
			assert_eq!(oc, 0x1002);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Informational(HciInformationalOpcode::ReadLocalSupportedCommands));
		}
	}

	#[test]
	fn test_hciinformationalopcode_readlocalsupportedfeatures() {
		{
			let oc: u16 = HciOpcode::Informational(HciInformationalOpcode::ReadLocalSupportedFeatures).into();
			assert_eq!(oc, 0x1003);
			let oc: u16 = HciInformationalOpcode::ReadLocalSupportedFeatures.into();
			assert_eq!(oc, 0x1003);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Informational(HciInformationalOpcode::ReadLocalSupportedFeatures));
		}
	}

	#[test]
	fn test_hciinformationalopcode_readlocalextendedfeatures() {
		{
			let oc: u16 = HciOpcode::Informational(HciInformationalOpcode::ReadLocalExtendedFeatures).into();
			assert_eq!(oc, 0x1004);
			let oc: u16 = HciInformationalOpcode::ReadLocalExtendedFeatures.into();
			assert_eq!(oc, 0x1004);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Informational(HciInformationalOpcode::ReadLocalExtendedFeatures));
		}
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciStatusParametersOpcode {
	Unknown(u16),
}

impl HciStatusParametersOpcode {
	fn from_trusted_u16(value: u16) -> HciStatusParametersOpcode {
		match value & ((1 << 10) - 1) {
			value => HciStatusParametersOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciStatusParametersOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciStatusParametersOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::status_parameters::OGF << 10) | ocf
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciTestingOpcode {
	Unknown(u16),
}

impl HciTestingOpcode {
	fn from_trusted_u16(value: u16) -> HciTestingOpcode {
		match value & ((1 << 10) - 1) {
			value => HciTestingOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciTestingOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciTestingOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::testing::OGF << 10) | ocf
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciLeControllerOpcode {
	SetEventMask,
	ReadLocalSupportedFeatures,
	SetAdvertisingParameters,
	SetAdvertisingData,
	SetScanResponseData,
	SetAdvertiseEnable,
	SetScanParameters,
	SetScanEnable,
	Unknown(u16),
}

impl HciLeControllerOpcode {
	fn from_trusted_u16(value: u16) -> HciLeControllerOpcode {
		match value & ((1 << 10) - 1) {
			value => HciLeControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciLeControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciLeControllerOpcode::SetEventMask => consts::opcode::le_controller::SET_EVENT_MASK,
			HciLeControllerOpcode::ReadLocalSupportedFeatures => consts::opcode::le_controller::READ_LOCAL_SUPPORTED_FEATURES,
			HciLeControllerOpcode::SetAdvertisingParameters => consts::opcode::le_controller::SET_ADVERTISING_PARAMETERS,
			HciLeControllerOpcode::SetAdvertisingData => consts::opcode::le_controller::SET_ADVERTISING_DATA,
			HciLeControllerOpcode::SetScanResponseData => consts::opcode::le_controller::SET_SCAN_RESPONSE_DATA,
			HciLeControllerOpcode::SetAdvertiseEnable => consts::opcode::le_controller::SET_ADVERTISE_ENABLE,
			HciLeControllerOpcode::SetScanParameters => consts::opcode::le_controller::SET_SCAN_PARAMETERS,
			HciLeControllerOpcode::SetScanEnable => consts::opcode::le_controller::SET_SCAN_ENABLE,
			HciLeControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		((consts::opcode::le_controller::OGF as u16) << 10) | ocf
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciVendorOpcode {
	Unknown(u16),
}

impl HciVendorOpcode {
	fn from_trusted_u16(value: u16) -> HciVendorOpcode {
		match value & ((1 << 10) - 1) {
			value => HciVendorOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciVendorOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciVendorOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::opcode::vendor::OGF << 10) | ocf
	}
}

#[cfg(test)]
mod hciopcode_tests {
	use super::*;

	#[test]
	fn test_hciopcode_controller() {
		{
			let oc: u16 = HciOpcode::Controller(HciControllerOpcode::Reset).into();
			assert_eq!(oc, 0x0C03);
			let oc: u16 = HciControllerOpcode::Reset.into();
			assert_eq!(oc, 0x0C03);
			let oc = HciOpcode::from(oc);
			assert_eq!(oc, HciOpcode::Controller(HciControllerOpcode::Reset));
		}
	}
}


// #[derive(Clone,Copy,Debug,PartialEq,Eq)]
// pub enum HciEventCode {
// 	CommandComplete,
// 	CommandStatus,
// 	LeMeta(HciLeMetaSubeventCode),
// }

// impl HciEventCode {
// 	pub fn from_readable(r: &mut std::io::Read) -> Option<HciEventCode> {
// 		let mut event_code = [0u8; 1];
// 		match r.read(&mut event_code) {
// 			Ok(1) => (),
// 			Ok(_) => return None,
// 			Err(_) => return None,
// 		}
// 		let event_code = event_code[0];
// 		Some(match event_code {
// 			consts::event_code::COMMAND_COMPLETE => HciEventCode::CommandComplete,
// 			consts::event_code::COMMAND_STATUS => HciEventCode::CommandStatus,
// 			consts::event_code::LE_META => {
// 				let mut subevent_code = [0u8; 1];
// 				match r.read(&mut subevent_code) {
// 					Ok(1) => (),
// 					Ok(_) => return None,
// 					Err(_) => return None,
// 				}
// 				let subevent_code = subevent_code[0];
// 				HciEventCode::LeMeta(match subevent_code {
// 					consts::event_code::le_meta::CONNECTION_COMPLETE => HciLeMetaSubeventCode::ConnectionComplete,
// 					consts::event_code::le_meta::ADVERTISING_REPORT => HciLeMetaSubeventCode::AdvertisingReport,
// 					consts::event_code::le_meta::CONNECTION_UPDATE_COMPLETE => HciLeMetaSubeventCode::ConnectionUpdateComplete,
// 					consts::event_code::le_meta::READ_REMOTE_USED_FEATURES_COMPLETE => HciLeMetaSubeventCode::ReadRemoteUsedFeaturesComplete,
// 					consts::event_code::le_meta::LONG_TERM_KEY_REQUEST => HciLeMetaSubeventCode::LongTermKeyRequest,
// 					_ => return None,
// 				})
// 			},
// 			_ => return None,
// 		})
// 	}
// }

// #[derive(Clone,Copy,Debug,PartialEq,Eq,FromPrimitive)]
// pub enum HciLeMetaSubeventCode {
// 	ConnectionComplete,
// 	AdvertisingReport,
// 	ConnectionUpdateComplete,
// 	ReadRemoteUsedFeaturesComplete,
// 	LongTermKeyRequest,
// }


// #[cfg(test)]
// mod hcieventcode_tests {
// 	use super::*;

// 	#[test]
// 	fn test_hcieventcode() {
// 		let ec = HciEventCode::from_readable(&mut [0x00].as_mut().as_ref());
// 		assert_eq!(ec, None);

// 		let ec = HciEventCode::from_readable(&mut [0x0eu8].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::CommandComplete));

// 		let ec = HciEventCode::from_readable(&mut [0x0fu8].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::CommandStatus));

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x00].as_mut().as_ref());
// 		assert_eq!(ec, None);

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x01].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ConnectionComplete)));

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x02].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::AdvertisingReport)));

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x03].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ConnectionUpdateComplete)));

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x04].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ReadRemoteUsedFeaturesComplete)));

// 		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x05].as_mut().as_ref());
// 		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::LongTermKeyRequest)));
// 	}
// }


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciPacketType {
	Command,
	AclData,
	ScoData,
	Event,
	Vendor,
}

impl HciPacketType {
	fn from_readable(r: &mut std::io::Read) -> Option<HciPacketType> {
		match r.read_u8() {
			Ok(packet_type) => match packet_type {
				0x01 => Some(HciPacketType::Command),
				0x02 => Some(HciPacketType::AclData),
				0x03 => Some(HciPacketType::ScoData),
				0x04 => Some(HciPacketType::Event),
				0xFF => Some(HciPacketType::Vendor),
				_ => None,
			},
			Err(e) => None,
		}
	}
}

#[derive(Debug)]
pub struct HciCommandPacket<'a> {
	opcode: HciOpcode,
	parameter_data: &'a [u8],
}
// #[derive(Debug)]
// pub struct HciAclDataPacket<'a> {
// 	data: &'a [u8],
// }
// #[derive(Debug)]
// pub struct HciScoDataPacket<'a> {
// 	data: &'a [u8],
// }
// #[derive(Debug)]
// pub struct HciEventPacket<'a> {
// 	event_code: HciEventCode,
// 	parameter_data: &'a [u8],
// }
// #[derive(Debug)]
// pub struct HciVendorPacket<'a> {
// 	data: &'a [u8],
// }

#[derive(Debug)]
pub enum HciPacket<'a> {
	Command(HciCommand<'a>),
	// AclData(HciAclDataPacket<'a>),
	// ScoData(HciScoDataPacket<'a>),
	// Event(HciEventPacket<'a>),
	// Vendor(HciVendorPacket<'a>),
}

impl<'a> HciPacket<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciPacket<'a>> {
		let mut c = std::io::Cursor::new(bytes);
		let packet_type: HciPacketType = match HciPacketType::from_readable(&mut c) {
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
			_ => None,
			// HciPacketType::AclData => {
			// 	None
			// },
			// HciPacketType::ScoData => {
			// 	None
			// },
			// HciPacketType::Event => {
			// 	match HciEventPacket::from_bytes(&bytes[1..]) {
			// 		Some(p) => Some(HciPacket::Event(p)),
			// 		None => None,
			// 	}
			// },
			// HciPacketType::Vendor => {
			// 	None
			// },
		}
	}
}


impl<'a> HciCommandPacket<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciCommandPacket<'a>> {
		let mut c = std::io::Cursor::new(bytes);
		let opcode = match HciOpcode::from_readable(&mut c) {
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


// impl<'a> HciEventPacket<'a> {
// 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEventPacket<'a>> {
// 		let mut c = std::io::Cursor::new(bytes);
// 		let event_code = match HciEventCode::from_readable(&mut c) {
// 			Some(event_code) => event_code,
// 			None => return None,
// 		};
// 		let parameter_data_length = match c.read_u8() {
// 			Ok(length) => length,
// 			Err(e) => return None,
// 		};
// 		let parameter_data = &bytes[(c.position() as usize)..];
// 		if parameter_data.len() == parameter_data_length as usize {
// 			return Some(HciEventPacket {
// 				event_code: event_code,
// 				parameter_data: parameter_data,
// 			});
// 		}
// 		None
// 	}

// 	pub fn to_event(&self) -> Option<HciEvent<'a>> {
// 		match self.event_code {
// 			HciEventCode::CommandComplete => {
// 				let mut c = std::io::Cursor::new(self.parameter_data);
// 				let num_hci_command_packets = match c.read_u8() {
// 					Ok(num_hci_command_packets) => num_hci_command_packets,
// 					Err(e) => return None,
// 				};
// 				let command_opcode = match c.read_u16::<LittleEndian>() {
// 					Ok(command_opcode) => command_opcode,
// 					Err(e) => return None,
// 				};
// 				let command_opcode = match HciOpcode::from_u16(command_opcode) {
// 					Some(command_opcode) => command_opcode,
// 					None => return None,
// 				};
// 				let return_parameter_data = &self.parameter_data[(c.position() as usize)..];
// 				Some(HciEvent::CommandComplete {
// 					num_hci_command_packets: num_hci_command_packets,
// 					command_opcode: command_opcode,
// 					return_parameter_data: return_parameter_data,
// 				})
// 			},
// 			HciEventCode::CommandStatus => {
// 				let mut c = std::io::Cursor::new(self.parameter_data);
// 				let status = match c.read_u8() {
// 					Ok(status) => status,
// 					Err(e) => return None,
// 				};
// 				let num_hci_command_packets = match c.read_u8() {
// 					Ok(num_hci_command_packets) => num_hci_command_packets,
// 					Err(e) => return None,
// 				};
// 				let command_opcode = match c.read_u16::<LittleEndian>() {
// 					Ok(command_opcode) => command_opcode,
// 					Err(e) => return None,
// 				};
// 				let command_opcode = match HciOpcode::from_u16(command_opcode) {
// 					Some(command_opcode) => command_opcode,
// 					None => return None,
// 				};
// 				Some(HciEvent::CommandStatus {
// 					status: status,
// 					num_hci_command_packets: num_hci_command_packets,
// 					command_opcode: command_opcode,
// 				})
// 			},
// 			HciEventCode::LeMeta(le_subeventcode) => {
// 				None
// 				// let mut c = std::io::Cursor::new(self.parameter_data);
// 				// let subevent_code = match c.read_u8() {
// 				// 	Ok(subevent_code) => subevent_code,
// 				// 	Err(e) => return None,
// 				// };
// 				// let subevent_code = match HciLeMetaSubeventCode::from_u8(subevent_code) {
// 				// 	Ok(subevent_code) => subevent_code,
// 				// 	Err(e) => return None,
// 				// };
// 				// match subevent_code {
// 				// 	HciLeMetaSubeventCode::AdvertisingReport => {
// 				// 		let mut c = std::io::Cursor::new(self.parameter_data);
// 				// 		let num_reports = match c.read_u8() {
// 				// 			Ok(num_reports) => num_reports,
// 				// 			Err(e) => return None,
// 				// 		};
// 				// 	},
// 				// 	_ => None,
// 				// }
// 			},
// 		}
// 	}
// }


// #[derive(Clone,Copy,Debug)]
// pub enum HciEvent<'a> {
// 	CommandComplete{ num_hci_command_packets: u8, command_opcode: HciOpcode, return_parameter_data: &'a [u8] },
// 	CommandStatus{ status: u8, num_hci_command_packets: u8, command_opcode: HciOpcode },
// 	LeMeta(HciLeMetaEvent<'a>),
// }

// #[derive(Clone,Copy,Debug)]
// pub enum HciLeMetaEvent<'a> {
// 	AdvertisingReport{ reports: &'a [HciLeAdvertisingReport<'a>] },
// }

// #[derive(Clone,Copy,Debug)]
// pub struct HciLeAdvertisingReport<'a> {
// 	pub event_type: u8,
// 	pub address_type: u8,
// 	pub address: [u8; 6],
// 	pub data: &'a [u8],
// 	pub rssi: u8,
// }

// impl<'a> HciEvent<'a> {
// 	pub fn from_bytes(bytes: &'a [u8]) -> Option<HciEvent<'a>> {
// 		None
// 	}
// }


// // pub struct HciHandle {
// // 	device_number: u16,
// // 	s: libc::c_int,
// // 	// t: std::thread::JoinGuard<'a, u8>,
// // 	// tx: std::sync::mpsc::Sender<u8>,
// // }

// // impl HciHandle {

// // 	pub fn new(device_number: u16) -> Result<HciHandle, HciError> {
// // 		let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK|nix::sys::socket::SOCK_CLOEXEC, 1) {
// // 			Ok(s) => s,
// // 			Err(e) => return Err(HciError::Nix(e)),
// // 		};
		
// // 		let sockaddr = nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(device_number), nix::sys::socket::HciChannel::Raw);
// // 		match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(sockaddr)) {
// // 			Ok(()) => (),
// // 			Err(e) => return Err(HciError::Nix(e)),
// // 		}

// // 		// let tb = std::thread::Builder::new().name("Bluetooth HCI".to_string());

// // 		// let (tx, rx) = std::sync::mpsc::channel();

// // 		// let t = match tb.scoped(move || {
// // 		// 	loop {
// // 		// 		let mut buf = [0u8; 256];
// // 		// 		match nix::unistd::read(s, &mut buf) {
// // 		// 			Ok(size) => {
// // 		// 				println!("read {} bytes:", size);
// // 		// 				println!("    {}", buf[0..size].to_hex())
// // 		// 			},
// // 		// 			Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
// // 		// 			Err(e) => panic!("err: {:?}", e),
// // 		// 		}
// // 		// 	}
// // 		// }) {
// // 		// 	Ok(t) => t,
// // 		// 	Err(e) => return Err(HciError::Io(e)),
// // 		// };

// // 		// Ok(HciHandle{ device_number: device_number, s: s, t: t, tx: tx })
// // 		Ok(HciHandle{ device_number: device_number, s: s })
// // 	}

// // 	pub fn reset(&self) -> Result<(), HciError> {
// // 		Ok(())
// // 	}

// // 	// pub fn new<T>(addr: &T) -> Result<HciHandle, HciError> where T: super::ToBdAddr {
// // 	// 	let a = addr.to_bdaddr();

// // 	// 	let d = unsafe { raw::hci::hci_get_route(&a.to_raw()) };
// // 	// 	if d < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	let d = unsafe { raw::hci::hci_open_dev(d) };
// // 	// 	if d < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(HciDeviceHandle(d))
// // 	// }


// // 	// pub fn read_local_name(&self) -> Result<String, HciError> {
// // 	// 	let mut name = [0 as u8; 248];
// // 	// 	let rv = unsafe { raw::hci::hci_read_local_name(self.0, 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}
// // 	// 
// // 	// 	match String::from_utf8(name.to_owned()) {
// // 	// 		Ok(name) => Ok(name),
// // 	// 		Err(_) => Err(HciError { errno: 0 }),
// // 	// 	}
// // 	// }

// // 	// pub fn write_local_name(&self, name: &str) -> Result<(), HciError> {
// // 	// 	let rv = unsafe { raw::hci::hci_write_local_name(self.0, name.as_bytes().as_ptr() as *const libc::c_char, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(())
// // 	// }

// // 	// pub fn read_local_version(&self) -> Result<HciVersion, HciError> {
// // 	// 	let mut v = raw::hci::hci_version { manufacturer: 0, hci_ver: 0, hci_rev: 0, lmp_ver: 0, lmp_subver: 0 };
// // 	// 	let rv = unsafe { raw::hci::hci_read_local_version(self.0, &mut v, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(HciVersion(v))
// // 	// }

// // 	// pub fn read_local_commands(&self) -> Result<HciCommands, HciError> {
// // 	// 	let mut c = raw::hci::hci_commands([0u8; 64]);
// // 	// 	let rv = unsafe { raw::hci::hci_read_local_commands(self.0, &mut c, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(HciCommands(c))
// // 	// }

// // 	// pub fn read_remote_name<T>(&self, addr: &T) -> Result<String, HciError> where T: common::ToBdAddr {
// // 	// 	let a = addr.to_bdaddr();
// // 	// 	let mut name = [0 as u8; 248];
// // 	// 	let rv = unsafe { raw::hci::hci_read_remote_name(self.0, &a.to_raw(), 248, &mut name as *mut _ as *mut libc::c_char, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	match String::from_utf8(name.to_owned()) {
// // 	// 		Ok(name) => Ok(name),
// // 	// 		Err(_) => Err(HciError { errno: 0 }),
// // 	// 	}
// // 	// }

// // 	// pub fn le_set_scan_enable(&self, enable: bool, filter_dup: bool) -> Result<(), HciError> {
// // 	// 	let rv = unsafe { raw::hci::hci_le_set_scan_enable(self.0, enable as libc::uint8_t, filter_dup as libc::uint8_t, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(())
// // 	// }

// // 	// pub fn le_set_scan_parameters(&self, scan_type: HciLeScanType, interval: u16, window: u16, addr_type: HciLeScanAddressType, filter: HciLeScanFilter) -> Result<(), HciError> {
// // 	// 	let rv = unsafe { raw::hci::hci_le_set_scan_parameters(self.0, scan_type as libc::uint8_t, interval as libc::uint16_t, window as libc::uint16_t, addr_type as libc::uint8_t, filter as libc::uint8_t, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(())
// // 	// }

// // 	// pub fn le_set_advertise_enable(&self, enable: bool) -> Result<(), HciError> {
// // 	// 	let rv = unsafe { raw::hci::hci_le_set_advertise_enable(self.0, enable as libc::uint8_t, 1000) };
// // 	// 	if rv < 0 {
// // 	// 		return Err(HciError { errno: std::os::errno() });
// // 	// 	}

// // 	// 	Ok(())
// // 	// }
// // }

// // // impl<'a> Drop for HciHandle<'a> {

// // // 	fn drop(&mut self) {
// // // 		let rv = unsafe { libc::close(self.s) };
// // // 		if rv != 0 {
// // // 			panic!("failed to close hci socket: {}", std::os::errno())
// // // 		}
// // // 	}

// // // }

// // // impl std::os::unix::AsRawFd for HciHandle {
// // //     fn as_raw_fd(&self) -> std::os::unix::Fd {
// // //         self.s
// // //     }
// // // }


// // #[cfg(test)]
// // mod tests {
// // 	use super::*;

// // 	#[test]
// // 	fn smoke() {
// // 		if let Ok(d) = HciHandle::new(0) {
// // 			d.reset();
// // 			// let name = d.read_local_name().unwrap();
// // 			// let _ = name;
// // 			// let v = d.read_local_version().unwrap();
// // 			// let _ = v;
// // 			// let c = d.read_local_commands().unwrap();
// // 			// let _ = c.iter();
// // 		}

// // 		// if let Ok(d) = HciDeviceHandle::new(&[0,0,0,0,0,0]) {
// // 		// 	let _ = d;
// // 		// }
// // 	}
// // }
