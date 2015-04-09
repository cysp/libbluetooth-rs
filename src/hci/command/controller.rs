use super::super::opcode::{ControllerOpcode};
use super::{HciCommandBuilder, HciCommandBuilding};


pub enum InquiryMode {
	Standard,
	RSSI,
	RSSIOrExtended,
	Unknown(u8),
}

impl From<u8> for InquiryMode {
	fn from(value: u8) -> InquiryMode {
		match value {
			0x00 => InquiryMode::Standard,
			0x01 => InquiryMode::RSSI,
			0x02 => InquiryMode::RSSIOrExtended,
			value => InquiryMode::Unknown(value),
		}
	}
}

impl Into<u8> for InquiryMode {
	fn into(self) -> u8 {
		match self {
			InquiryMode::Standard => 0x00,
			InquiryMode::RSSI => 0x01,
			InquiryMode::RSSIOrExtended => 0x02,
			InquiryMode::Unknown(value) => value,
		}
	}
}


define_command!(ControllerOpcode::Reset);
define_command!(ControllerOpcode::WriteInquiryMode, mode: InquiryMode as u8);
