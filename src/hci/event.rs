#![allow(unused_imports)]

use std;
use std::io;
use std::error::{Error};

use serialize::hex::ToHex;

use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};

mod consts {
	pub use hci::consts::events::*;
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EventCode {
	CommandComplete,
	CommandStatus,
	LeMeta(LeMetaSubeventCode),
}

impl EventCode {
	pub fn from_readable(r: &mut std::io::Read) -> Option<EventCode> {
		let mut event_code = [0u8; 1];
		match r.read(&mut event_code) {
			Ok(1) => (),
			Ok(_) => return None,
			Err(_) => return None,
		}
		let event_code = event_code[0];
		Some(match event_code {
			consts::codes::COMMAND_COMPLETE => EventCode::CommandComplete,
			consts::codes::COMMAND_STATUS => EventCode::CommandStatus,
			consts::codes::LE_META => {
				let mut subevent_code = [0u8; 1];
				match r.read(&mut subevent_code) {
					Ok(1) => (),
					Ok(_) => return None,
					Err(_) => return None,
				}
				let subevent_code = subevent_code[0];
				EventCode::LeMeta(match subevent_code {
					consts::codes::le_subevent::CONNECTION_COMPLETE => LeMetaSubeventCode::ConnectionComplete,
					consts::codes::le_subevent::ADVERTISING_REPORT => LeMetaSubeventCode::AdvertisingReport,
					consts::codes::le_subevent::CONNECTION_UPDATE_COMPLETE => LeMetaSubeventCode::ConnectionUpdateComplete,
					consts::codes::le_subevent::READ_REMOTE_USED_FEATURES_COMPLETE => LeMetaSubeventCode::ReadRemoteUsedFeaturesComplete,
					consts::codes::le_subevent::LONG_TERM_KEY_REQUEST => LeMetaSubeventCode::LongTermKeyRequest,
					_ => return None,
				})
			},
			_ => return None,
		})
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,FromPrimitive)]
pub enum LeMetaSubeventCode {
	ConnectionComplete,
	AdvertisingReport,
	ConnectionUpdateComplete,
	ReadRemoteUsedFeaturesComplete,
	LongTermKeyRequest,
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hcieventcode() {
		let ec = EventCode::from_readable(&mut [0x00].as_mut().as_ref());
		assert_eq!(ec, None);

		let ec = EventCode::from_readable(&mut [0x0eu8].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::CommandComplete));

		let ec = EventCode::from_readable(&mut [0x0fu8].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::CommandStatus));

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x00].as_mut().as_ref());
		assert_eq!(ec, None);

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x01].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::LeMeta(LeMetaSubeventCode::ConnectionComplete)));

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x02].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::LeMeta(LeMetaSubeventCode::AdvertisingReport)));

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x03].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::LeMeta(LeMetaSubeventCode::ConnectionUpdateComplete)));

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x04].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::LeMeta(LeMetaSubeventCode::ReadRemoteUsedFeaturesComplete)));

		let ec = EventCode::from_readable(&mut [0x3eu8, 0x05].as_mut().as_ref());
		assert_eq!(ec, Some(EventCode::LeMeta(LeMetaSubeventCode::LongTermKeyRequest)));
	}
}
