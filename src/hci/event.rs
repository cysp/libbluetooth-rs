extern crate libc;

use std;
use std::io;
use std::error::{Error};
use std::borrow::ToOwned;
use std::num::FromPrimitive;

use serialize::hex::ToHex;

use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};


mod consts {
	pub const COMMAND_COMPLETE: u8 = 0x0E;
	pub const COMMAND_STATUS: u8 = 0x0F;
	pub const LE_META: u8 = 0x3E;

	pub mod le_subevent {
		pub const CONNECTION_COMPLETE: u8 = 0x01;
		pub const ADVERTISING_REPORT: u8 = 0x02;
		pub const CONNECTION_UPDATE_COMPLETE: u8 = 0x03;
		pub const READ_REMOTE_USED_FEATURES_COMPLETE: u8 = 0x04;
		pub const LONG_TERM_KEY_REQUEST: u8 = 0x05;
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciEventCode {
	CommandComplete,
	CommandStatus,
	LeMeta(HciLeMetaSubeventCode),
}

impl HciEventCode {
	pub fn from_readable(r: &mut std::io::Read) -> Option<HciEventCode> {
		let mut event_code = [0u8; 1];
		match r.read(&mut event_code) {
			Ok(1) => (),
			Ok(_) => return None,
			Err(_) => return None,
		}
		let event_code = event_code[0];
		Some(match event_code {
			consts::COMMAND_COMPLETE => HciEventCode::CommandComplete,
			consts::COMMAND_STATUS => HciEventCode::CommandStatus,
			consts::LE_META => {
				let mut subevent_code = [0u8; 1];
				match r.read(&mut subevent_code) {
					Ok(1) => (),
					Ok(_) => return None,
					Err(_) => return None,
				}
				let subevent_code = subevent_code[0];
				HciEventCode::LeMeta(match subevent_code {
					consts::le_subevent::CONNECTION_COMPLETE => HciLeMetaSubeventCode::ConnectionComplete,
					consts::le_subevent::ADVERTISING_REPORT => HciLeMetaSubeventCode::AdvertisingReport,
					consts::le_subevent::CONNECTION_UPDATE_COMPLETE => HciLeMetaSubeventCode::ConnectionUpdateComplete,
					consts::le_subevent::READ_REMOTE_USED_FEATURES_COMPLETE => HciLeMetaSubeventCode::ReadRemoteUsedFeaturesComplete,
					consts::le_subevent::LONG_TERM_KEY_REQUEST => HciLeMetaSubeventCode::LongTermKeyRequest,
					_ => return None,
				})
			},
			_ => return None,
		})
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum HciLeMetaSubeventCode {
	ConnectionComplete,
	AdvertisingReport,
	ConnectionUpdateComplete,
	ReadRemoteUsedFeaturesComplete,
	LongTermKeyRequest,
}


#[cfg(test)]
mod hcieventcode_tests {
	use super::*;

	#[test]
	fn test_hcieventcode() {
		let ec = HciEventCode::from_readable(&mut [0x00].as_mut().as_ref());
		assert_eq!(ec, None);

		let ec = HciEventCode::from_readable(&mut [0x0eu8].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::CommandComplete));

		let ec = HciEventCode::from_readable(&mut [0x0fu8].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::CommandStatus));

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x00].as_mut().as_ref());
		assert_eq!(ec, None);

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x01].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ConnectionComplete)));

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x02].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::AdvertisingReport)));

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x03].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ConnectionUpdateComplete)));

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x04].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::ReadRemoteUsedFeaturesComplete)));

		let ec = HciEventCode::from_readable(&mut [0x3eu8, 0x05].as_mut().as_ref());
		assert_eq!(ec, Some(HciEventCode::LeMeta(HciLeMetaSubeventCode::LongTermKeyRequest)));
	}
}
