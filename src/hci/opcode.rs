extern crate libc;

use std;
use std::io;
use std::error::{Error};
use std::borrow::ToOwned;
use std::num::FromPrimitive;

use serialize::hex::ToHex;

use byteorder::{ReadBytesExt,WriteBytesExt,LittleEndian};


mod consts {
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

		pub const HOLD_MODE: u16 = 0x0001;
		pub const SNIFF_MODE: u16 = 0x0003;
		pub const EXIT_SNIFF_MODE: u16 = 0x0004;
		pub const PARK_STATE: u16 = 0x0005;
		pub const EXIT_PARK_STATE: u16 = 0x0006;
		pub const QOS_SETUP: u16 = 0x0007;
		pub const ROLE_DISCOVERY: u16 = 0x0009;
		pub const SWITCH_ROLE: u16 = 0x000B;
		pub const READ_LINK_POLICY_SETTINGS: u16 = 0x000C;
		pub const WRITE_LINK_POLICY_SETTINGS: u16 = 0x000D;
		pub const READ_DEFAULT_LINK_POLICY_SETTINGS: u16 = 0x000E;
		pub const WRITE_DEFAULT_LINK_POLICY_SETTINGS: u16 = 0x000F;
		pub const FLOW_SPECIFICATION: u16 = 0x0010;
		pub const SNIFF_SUBRATING: u16 = 0x0011;
	}

	pub mod controller {
		pub const OGF: u16 = 0x03;

		pub const SET_EVENT_MASK: u16 = 0x0001;
		pub const RESET: u16 = 0x0003;
		pub const SET_EVENT_FILTER: u16 = 0x0005;
		pub const FLUSH: u16 = 0x0008;
		pub const READ_PIN_TYPE: u16 = 0x0009;
		pub const WRITE_PIN_TYPE: u16 = 0x000A;
		pub const CREATE_NEW_UNIT_KEY: u16 = 0x000B;
		pub const READ_STORED_LINK_KEY: u16 = 0x000D;
		pub const WRITE_STORED_LINK_KEY: u16 = 0x0011;
		pub const DELETE_STORED_LINK_KEY: u16 = 0x0012;
		pub const WRITE_LOCAL_NAME: u16 = 0x0013;
		pub const READ_LOCAL_NAME: u16 = 0x0014;
		pub const READ_CONNECTION_ACCEPT_TIMEOUT: u16 = 0x0015;
		pub const WRITE_CONNECTION_ACCEPT_TIMEOUT: u16 = 0x0016;
		pub const READ_SCAN_ENABLE: u16 = 0x0019;
		pub const WRITE_SCAN_ENABLE: u16 = 0x001A;
		pub const READ_PAGE_SCAN_ACTIVITY: u16 = 0x001B;
		pub const WRITE_PAGE_SCAN_ACTIVITY: u16 = 0x001C;
		pub const READ_INQUIRY_SCAN_ACTIVITY: u16 = 0x001D;
		pub const WRITE_INQUIRY_SCAN_ACTIVITY: u16 = 0x001E;
		pub const READ_CLASS_OF_DEVICE: u16 = 0x0023;
		pub const WRITE_CLASS_OF_DEVICE: u16 = 0x0024;
		pub const READ_VOICE_SETTING: u16 = 0x0025;
		pub const WRITE_VOICE_SETTING: u16 = 0x0026;
		pub const READ_NUMBER_OF_SUPPORTED_IAC: u16 = 0x0038;
		pub const READ_CURRENT_IAC_LAP: u16 = 0x0039;
		pub const READ_INQUIRY_SCAN_TYPE: u16 = 0x0042;
		pub const WRITE_INQUIRY_SCAN_TYPE: u16 = 0x0043;
		pub const READ_INQUIRY_MODE: u16 = 0x0044;
		pub const WRITE_INQUIRY_MODE: u16 = 0x0045;
		pub const READ_PAGE_SCAN_TYPE: u16 = 0x0046;
		pub const WRITE_PAGE_SCAN_TYPE: u16 = 0x0047;
		pub const READ_AFH_CHANNEL_ASSESSMENT_MODE: u16 = 0x0048;
		pub const WRITE_AFH_CHANNEL_ASSESSMENT_MODE: u16 = 0x0049;
		pub const READ_EXTENDED_INQUIRY_RESPONSE: u16 = 0x0051;
		pub const WRITE_EXTENDED_INQUIRY_RESPONSE: u16 = 0x0052;
		pub const REFRESH_ENCRYPTION_KEY: u16 = 0x0053;
		pub const READ_SIMPLE_PAIRING_MODE: u16 = 0x0055;
		pub const WRITE_SIMPLE_PAIRING_MODE: u16 = 0x0056;
		pub const READ_LOCAL_OOB_DATA: u16 = 0x0057;
		pub const READ_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL: u16 = 0x0058;
		pub const WRITE_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL: u16 = 0x0059;
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
		pub const READ_BUFFER_SIZE: u16 = 0x0002;
		pub const READ_LOCAL_SUPPORTED_FEATURES: u16 = 0x0003;
		pub const SET_RANDOM_ADDRESS: u16 = 0x005;
		pub const SET_ADVERTISING_PARAMETERS: u16 = 0x0006;
		pub const READ_ADVERTISING_CHANNEL_TX_POWER: u16 = 0x0007;
		pub const SET_ADVERTISING_DATA: u16 = 0x0008;
		pub const SET_SCAN_RESPONSE_DATA: u16 = 0x0009;
		pub const SET_ADVERTISE_ENABLE: u16 = 0x000A;
		pub const SET_SCAN_PARAMETERS: u16 = 0x000B;
		pub const SET_SCAN_ENABLE: u16 = 0x000C;
		pub const CREATE_CONNECTION: u16 = 0x000D;
		pub const CREATE_CONNECTION_CANCEL: u16 = 0x000E;
		pub const READ_WHITELIST_SIZE: u16 = 0x000F;
		pub const CLEAR_WHITELIST: u16 = 0x0010;
		pub const ADD_DEVICE_TO_WHITELIST: u16 = 0x0011;
		pub const REMOVE_DEVICE_FROM_WHITELIST: u16 = 0x0012;
		pub const CONNECTION_UPDATE: u16 = 0x0013;
		pub const SET_HOST_CHANNEL_CLASSIFICATION: u16 = 0x0014;
		pub const READ_CHANNEL_MAP: u16 = 0x0015;
		pub const READ_REMOTE_USED_FEATURES: u16 = 0x0016;
		pub const ENCRYPT: u16 = 0x0017;
		pub const RAND: u16 = 0x0018;
		pub const START_ENCRYPTION: u16 = 0x0019;
		pub const LONG_TERM_KEY_REQUEST_REPLY: u16 = 0x001A;
		pub const LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY: u16 = 0x001B;
		pub const READ_SUPPORTED_STATES: u16 = 0x001C;
		pub const RECEIVER_TEST: u16 = 0x001D;
		pub const TRANSMITTER_TEST: u16 = 0x001E;
		pub const TEST_END: u16 = 0x001F;
	}

	pub mod vendor {
		pub const OGF: u16 = 0x3F;
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
			consts::nop::OGF => HciOpcodeGroup::Nop,
			consts::link_control::OGF => HciOpcodeGroup::LinkControl,
			consts::link_policy::OGF => HciOpcodeGroup::LinkPolicy,
			consts::controller::OGF => HciOpcodeGroup::Controller,
			consts::informational::OGF => HciOpcodeGroup::Informational,
			consts::status_parameters::OGF => HciOpcodeGroup::StatusParameters,
			consts::testing::OGF => HciOpcodeGroup::Testing,
			consts::le_controller::OGF => HciOpcodeGroup::LeController,
			consts::vendor::OGF => HciOpcodeGroup::Vendor,
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
			HciNopOpcode::Nop => consts::nop::NOP,
			HciNopOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::nop::OGF << 10) | ocf
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
		(consts::link_control::OGF << 10) | ocf
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciLinkPolicyOpcode {
	HoldMode,
	SniffMode,
	ExitSniffMode,
	ParkState,
	ExitParkState,
	QosSetup,
	RoleDiscovery,
	SwitchRole,
	ReadLinkPolicySettings,
	WriteLinkPolicySettings,
	ReadDefaultLinkPolicySettings,
	WriteDefaultLinkPolicySettings,
	FlowSpecification,
	SniffSubrating,
	Unknown(u16),
}

impl HciLinkPolicyOpcode {
	fn from_trusted_u16(value: u16) -> HciLinkPolicyOpcode {
		match value & ((1 << 10) - 1) {
			consts::link_policy::HOLD_MODE => HciLinkPolicyOpcode::HoldMode,
			consts::link_policy::SNIFF_MODE => HciLinkPolicyOpcode::SniffMode,
			consts::link_policy::EXIT_SNIFF_MODE => HciLinkPolicyOpcode::ExitSniffMode,
			consts::link_policy::PARK_STATE => HciLinkPolicyOpcode::ParkState,
			consts::link_policy::EXIT_PARK_STATE => HciLinkPolicyOpcode::ExitParkState,
			consts::link_policy::QOS_SETUP => HciLinkPolicyOpcode::QosSetup,
			consts::link_policy::ROLE_DISCOVERY => HciLinkPolicyOpcode::RoleDiscovery,
			consts::link_policy::SWITCH_ROLE => HciLinkPolicyOpcode::SwitchRole,
			consts::link_policy::READ_LINK_POLICY_SETTINGS => HciLinkPolicyOpcode::ReadLinkPolicySettings,
			consts::link_policy::WRITE_LINK_POLICY_SETTINGS => HciLinkPolicyOpcode::WriteLinkPolicySettings,
			consts::link_policy::READ_DEFAULT_LINK_POLICY_SETTINGS => HciLinkPolicyOpcode::ReadDefaultLinkPolicySettings,
			consts::link_policy::WRITE_DEFAULT_LINK_POLICY_SETTINGS => HciLinkPolicyOpcode::WriteDefaultLinkPolicySettings,
			consts::link_policy::FLOW_SPECIFICATION => HciLinkPolicyOpcode::FlowSpecification,
			consts::link_policy::SNIFF_SUBRATING => HciLinkPolicyOpcode::SniffSubrating,
			value => HciLinkPolicyOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciLinkPolicyOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciLinkPolicyOpcode::HoldMode => consts::link_policy::HOLD_MODE,
			HciLinkPolicyOpcode::SniffMode => consts::link_policy::SNIFF_MODE,
			HciLinkPolicyOpcode::ExitSniffMode => consts::link_policy::EXIT_SNIFF_MODE,
			HciLinkPolicyOpcode::ParkState => consts::link_policy::PARK_STATE,
			HciLinkPolicyOpcode::ExitParkState => consts::link_policy::EXIT_PARK_STATE,
			HciLinkPolicyOpcode::QosSetup => consts::link_policy::QOS_SETUP,
			HciLinkPolicyOpcode::RoleDiscovery => consts::link_policy::ROLE_DISCOVERY,
			HciLinkPolicyOpcode::SwitchRole => consts::link_policy::SWITCH_ROLE,
			HciLinkPolicyOpcode::ReadLinkPolicySettings => consts::link_policy::READ_LINK_POLICY_SETTINGS,
			HciLinkPolicyOpcode::WriteLinkPolicySettings => consts::link_policy::WRITE_LINK_POLICY_SETTINGS,
			HciLinkPolicyOpcode::ReadDefaultLinkPolicySettings => consts::link_policy::READ_DEFAULT_LINK_POLICY_SETTINGS,
			HciLinkPolicyOpcode::WriteDefaultLinkPolicySettings => consts::link_policy::WRITE_DEFAULT_LINK_POLICY_SETTINGS,
			HciLinkPolicyOpcode::FlowSpecification => consts::link_policy::FLOW_SPECIFICATION,
			HciLinkPolicyOpcode::SniffSubrating => consts::link_policy::SNIFF_SUBRATING,
			HciLinkPolicyOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::link_policy::OGF << 10) | ocf
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
	CreateNewUnitKey,
	ReadStoredLinkKey,
	WriteStoredLinkKey,
	DeleteStoredLinkKey,
	WriteLocalName,
	ReadLocalName,
	ReadConnectionAcceptTimeout,
	WriteConnectionAcceptTimeout,
	ReadScanEnable,
	WriteScanEnable,
	ReadPageScanActivity,
	WritePageScanActivity,
	ReadInquiryScanActivity,
	WriteInquiryScanActivity,
	ReadClassOfDevice,
	WriteClassOfDevice,
	ReadVoiceSetting,
	WriteVoiceSetting,
	ReadNumberOfSupportedIac,
	ReadCurrentIacLap,
	ReadInquiryScanType,
	WriteInquiryScanType,
	ReadInquiryMode,
	WriteInquiryMode,
	ReadPageScanType,
	WritePageScanType,
	ReadAfhChannelAssessmentMode,
	WriteAfhChannelAssessmentMode,
	ReadExtendedInquiryResponse,
	WriteExtendedInquiryResponse,
	RefreshEncryptionKey,
	ReadSimplePairingMode,
	WriteSimplePairingMode,
	ReadLocalOobData,
	ReadInquiryResponseTransmitPowerLevel,
	WriteInquiryResponseTransmitPowerLevel,
	Unknown(u16),
}

impl HciControllerOpcode {
	fn from_trusted_u16(value: u16) -> HciControllerOpcode {
		match value & ((1 << 10) - 1) {
			consts::controller::SET_EVENT_MASK => HciControllerOpcode::SetEventMask,
			consts::controller::RESET => HciControllerOpcode::Reset,
			consts::controller::SET_EVENT_FILTER => HciControllerOpcode::SetEventFilter,
			consts::controller::FLUSH => HciControllerOpcode::Flush,
			consts::controller::READ_PIN_TYPE => HciControllerOpcode::ReadPINType,
			consts::controller::WRITE_PIN_TYPE => HciControllerOpcode::WritePINType,
			consts::controller::CREATE_NEW_UNIT_KEY => HciControllerOpcode::CreateNewUnitKey,
			consts::controller::READ_STORED_LINK_KEY => HciControllerOpcode::ReadStoredLinkKey,
			consts::controller::WRITE_STORED_LINK_KEY => HciControllerOpcode::WriteStoredLinkKey,
			consts::controller::DELETE_STORED_LINK_KEY => HciControllerOpcode::DeleteStoredLinkKey,
			consts::controller::WRITE_LOCAL_NAME => HciControllerOpcode::WriteLocalName,
			consts::controller::READ_LOCAL_NAME => HciControllerOpcode::ReadLocalName,
			consts::controller::READ_CONNECTION_ACCEPT_TIMEOUT => HciControllerOpcode::ReadConnectionAcceptTimeout,
			consts::controller::WRITE_CONNECTION_ACCEPT_TIMEOUT => HciControllerOpcode::WriteConnectionAcceptTimeout,
			consts::controller::READ_SCAN_ENABLE => HciControllerOpcode::ReadScanEnable,
			consts::controller::WRITE_SCAN_ENABLE => HciControllerOpcode::WriteScanEnable,
			consts::controller::READ_PAGE_SCAN_ACTIVITY => HciControllerOpcode::ReadPageScanActivity,
			consts::controller::WRITE_PAGE_SCAN_ACTIVITY => HciControllerOpcode::WritePageScanActivity,
			consts::controller::READ_INQUIRY_SCAN_ACTIVITY => HciControllerOpcode::ReadInquiryScanActivity,
			consts::controller::WRITE_INQUIRY_SCAN_ACTIVITY => HciControllerOpcode::WriteInquiryScanActivity,
			consts::controller::READ_CLASS_OF_DEVICE => HciControllerOpcode::ReadClassOfDevice,
			consts::controller::WRITE_CLASS_OF_DEVICE => HciControllerOpcode::WriteClassOfDevice,
			consts::controller::READ_VOICE_SETTING => HciControllerOpcode::ReadVoiceSetting,
			consts::controller::WRITE_VOICE_SETTING => HciControllerOpcode::WriteVoiceSetting,
			consts::controller::READ_NUMBER_OF_SUPPORTED_IAC => HciControllerOpcode::ReadNumberOfSupportedIac,
			consts::controller::READ_CURRENT_IAC_LAP => HciControllerOpcode::ReadCurrentIacLap,
			consts::controller::READ_INQUIRY_SCAN_TYPE => HciControllerOpcode::ReadInquiryScanType,
			consts::controller::WRITE_INQUIRY_SCAN_TYPE => HciControllerOpcode::WriteInquiryScanType,
			consts::controller::READ_INQUIRY_MODE => HciControllerOpcode::ReadInquiryMode,
			consts::controller::WRITE_INQUIRY_MODE => HciControllerOpcode::WriteInquiryMode,
			consts::controller::READ_PAGE_SCAN_TYPE => HciControllerOpcode::ReadPageScanType,
			consts::controller::WRITE_PAGE_SCAN_TYPE => HciControllerOpcode::WritePageScanType,
			consts::controller::READ_AFH_CHANNEL_ASSESSMENT_MODE => HciControllerOpcode::ReadAfhChannelAssessmentMode,
			consts::controller::WRITE_AFH_CHANNEL_ASSESSMENT_MODE => HciControllerOpcode::WriteAfhChannelAssessmentMode,
			consts::controller::READ_EXTENDED_INQUIRY_RESPONSE => HciControllerOpcode::ReadExtendedInquiryResponse,
			consts::controller::WRITE_EXTENDED_INQUIRY_RESPONSE => HciControllerOpcode::WriteExtendedInquiryResponse,
			consts::controller::REFRESH_ENCRYPTION_KEY => HciControllerOpcode::RefreshEncryptionKey,
			consts::controller::READ_SIMPLE_PAIRING_MODE => HciControllerOpcode::ReadSimplePairingMode,
			consts::controller::WRITE_SIMPLE_PAIRING_MODE => HciControllerOpcode::WriteSimplePairingMode,
			consts::controller::READ_LOCAL_OOB_DATA => HciControllerOpcode::ReadLocalOobData,
			consts::controller::READ_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL => HciControllerOpcode::ReadInquiryResponseTransmitPowerLevel,
			consts::controller::WRITE_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL => HciControllerOpcode::WriteInquiryResponseTransmitPowerLevel,
			value => HciControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciControllerOpcode::SetEventMask => consts::controller::SET_EVENT_MASK,
			HciControllerOpcode::Reset => consts::controller::RESET,
			HciControllerOpcode::SetEventFilter => consts::controller::SET_EVENT_FILTER,
			HciControllerOpcode::Flush => consts::controller::FLUSH,
			HciControllerOpcode::ReadPINType => consts::controller::READ_PIN_TYPE,
			HciControllerOpcode::WritePINType => consts::controller::WRITE_PIN_TYPE,
			HciControllerOpcode::CreateNewUnitKey => consts::controller::CREATE_NEW_UNIT_KEY,
			HciControllerOpcode::ReadStoredLinkKey => consts::controller::READ_STORED_LINK_KEY,
			HciControllerOpcode::WriteStoredLinkKey => consts::controller::WRITE_STORED_LINK_KEY,
			HciControllerOpcode::DeleteStoredLinkKey => consts::controller::DELETE_STORED_LINK_KEY,
			HciControllerOpcode::WriteLocalName => consts::controller::WRITE_LOCAL_NAME,
			HciControllerOpcode::ReadLocalName => consts::controller::READ_LOCAL_NAME,
			HciControllerOpcode::ReadConnectionAcceptTimeout => consts::controller::READ_CONNECTION_ACCEPT_TIMEOUT,
			HciControllerOpcode::WriteConnectionAcceptTimeout => consts::controller::WRITE_CONNECTION_ACCEPT_TIMEOUT,
			HciControllerOpcode::ReadScanEnable => consts::controller::READ_SCAN_ENABLE,
			HciControllerOpcode::WriteScanEnable => consts::controller::WRITE_SCAN_ENABLE,
			HciControllerOpcode::ReadPageScanActivity => consts::controller::READ_PAGE_SCAN_ACTIVITY,
			HciControllerOpcode::WritePageScanActivity => consts::controller::WRITE_PAGE_SCAN_ACTIVITY,
			HciControllerOpcode::ReadInquiryScanActivity => consts::controller::READ_INQUIRY_SCAN_ACTIVITY,
			HciControllerOpcode::WriteInquiryScanActivity => consts::controller::WRITE_INQUIRY_SCAN_ACTIVITY,
			HciControllerOpcode::ReadClassOfDevice => consts::controller::READ_CLASS_OF_DEVICE,
			HciControllerOpcode::WriteClassOfDevice => consts::controller::WRITE_CLASS_OF_DEVICE,
			HciControllerOpcode::ReadVoiceSetting => consts::controller::READ_VOICE_SETTING,
			HciControllerOpcode::WriteVoiceSetting => consts::controller::WRITE_VOICE_SETTING,
			HciControllerOpcode::ReadNumberOfSupportedIac => consts::controller::READ_NUMBER_OF_SUPPORTED_IAC,
			HciControllerOpcode::ReadCurrentIacLap => consts::controller::READ_CURRENT_IAC_LAP,
			HciControllerOpcode::ReadInquiryScanType => consts::controller::READ_INQUIRY_SCAN_TYPE,
			HciControllerOpcode::WriteInquiryScanType => consts::controller::WRITE_INQUIRY_SCAN_TYPE,
			HciControllerOpcode::ReadInquiryMode => consts::controller::READ_INQUIRY_MODE,
			HciControllerOpcode::WriteInquiryMode => consts::controller::WRITE_INQUIRY_MODE,
			HciControllerOpcode::ReadPageScanType => consts::controller::READ_PAGE_SCAN_TYPE,
			HciControllerOpcode::WritePageScanType => consts::controller::WRITE_PAGE_SCAN_TYPE,
			HciControllerOpcode::ReadAfhChannelAssessmentMode => consts::controller::READ_AFH_CHANNEL_ASSESSMENT_MODE,
			HciControllerOpcode::WriteAfhChannelAssessmentMode => consts::controller::WRITE_AFH_CHANNEL_ASSESSMENT_MODE,
			HciControllerOpcode::ReadExtendedInquiryResponse => consts::controller::READ_EXTENDED_INQUIRY_RESPONSE,
			HciControllerOpcode::WriteExtendedInquiryResponse => consts::controller::WRITE_EXTENDED_INQUIRY_RESPONSE,
			HciControllerOpcode::RefreshEncryptionKey => consts::controller::REFRESH_ENCRYPTION_KEY,
			HciControllerOpcode::ReadSimplePairingMode => consts::controller::READ_SIMPLE_PAIRING_MODE,
			HciControllerOpcode::WriteSimplePairingMode => consts::controller::WRITE_SIMPLE_PAIRING_MODE,
			HciControllerOpcode::ReadLocalOobData => consts::controller::READ_LOCAL_OOB_DATA,
			HciControllerOpcode::ReadInquiryResponseTransmitPowerLevel => consts::controller::READ_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL,
			HciControllerOpcode::WriteInquiryResponseTransmitPowerLevel => consts::controller::WRITE_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL,
			HciControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::controller::OGF << 10) | ocf
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
			consts::informational::READ_LOCAL_VERSION_INFORMATION => HciInformationalOpcode::ReadLocalVersionInformation,
			consts::informational::READ_LOCAL_SUPPORTED_COMMANDS => HciInformationalOpcode::ReadLocalSupportedCommands,
			consts::informational::READ_LOCAL_SUPPORTED_FEATURES => HciInformationalOpcode::ReadLocalSupportedFeatures,
			consts::informational::READ_LOCAL_EXTENDED_FEATURES => HciInformationalOpcode::ReadLocalExtendedFeatures,
			consts::informational::READ_BUFFER_SIZE => HciInformationalOpcode::ReadBufferSize,
			consts::informational::READ_BD_ADDR => HciInformationalOpcode::ReadBdAddr,
			consts::informational::READ_DATA_BLOCK_SIZE => HciInformationalOpcode::ReadDataBlockSize,
			value => HciInformationalOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciInformationalOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciInformationalOpcode::ReadLocalVersionInformation => consts::informational::READ_LOCAL_VERSION_INFORMATION,
			HciInformationalOpcode::ReadLocalSupportedCommands => consts::informational::READ_LOCAL_SUPPORTED_COMMANDS,
			HciInformationalOpcode::ReadLocalSupportedFeatures => consts::informational::READ_LOCAL_SUPPORTED_FEATURES,
			HciInformationalOpcode::ReadLocalExtendedFeatures => consts::informational::READ_LOCAL_EXTENDED_FEATURES,
			HciInformationalOpcode::ReadBufferSize => consts::informational::READ_BUFFER_SIZE,
			HciInformationalOpcode::ReadBdAddr => consts::informational::READ_BD_ADDR,
			HciInformationalOpcode::ReadDataBlockSize => consts::informational::READ_DATA_BLOCK_SIZE,
			HciInformationalOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::informational::OGF << 10) | ocf
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
		(consts::status_parameters::OGF << 10) | ocf
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
		(consts::testing::OGF << 10) | ocf
	}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HciLeControllerOpcode {
	SetEventMask,
	ReadBufferSize,
	ReadLocalSupportedFeatures,
	SetRandomAddress,
	SetAdvertisingParameters,
	ReadAdvertisingChannelTxPower,
	SetAdvertisingData,
	SetScanResponseData,
	SetAdvertiseEnable,
	SetScanParameters,
	SetScanEnable,
	CreateConnection,
	CreateConnectionCancel,
	ReadWhitelistSize,
	ClearWhitelist,
	AddDeviceToWhitelist,
	RemoveDeviceFromWhitelist,
	ConnectionUpdate,
	SetHostChannelClassification,
	ReadChannelMap,
	ReadRemoteUsedFeatures,
	Encrypt,
	Rand,
	StartEncryption,
	LongTermKeyRequestReply,
	LongTermKeyRequestNegativeReply,
	ReadSupportedStates,
	ReceiverTest,
	TransmitterTest,
	TestEnd,
	Unknown(u16),
}

impl HciLeControllerOpcode {
	fn from_trusted_u16(value: u16) -> HciLeControllerOpcode {
		match value & ((1 << 10) - 1) {
			consts::le_controller::SET_EVENT_MASK => HciLeControllerOpcode::SetEventMask,
			consts::le_controller::READ_BUFFER_SIZE => HciLeControllerOpcode::ReadBufferSize,
			consts::le_controller::READ_LOCAL_SUPPORTED_FEATURES => HciLeControllerOpcode::ReadLocalSupportedFeatures,
			consts::le_controller::SET_RANDOM_ADDRESS => HciLeControllerOpcode::SetRandomAddress,
			consts::le_controller::SET_ADVERTISING_PARAMETERS => HciLeControllerOpcode::SetAdvertisingParameters,
			consts::le_controller::READ_ADVERTISING_CHANNEL_TX_POWER => HciLeControllerOpcode::ReadAdvertisingChannelTxPower,
			consts::le_controller::SET_ADVERTISING_DATA => HciLeControllerOpcode::SetAdvertisingData,
			consts::le_controller::SET_SCAN_RESPONSE_DATA => HciLeControllerOpcode::SetScanResponseData,
			consts::le_controller::SET_ADVERTISE_ENABLE => HciLeControllerOpcode::SetAdvertiseEnable,
			consts::le_controller::SET_SCAN_PARAMETERS => HciLeControllerOpcode::SetScanParameters,
			consts::le_controller::SET_SCAN_ENABLE => HciLeControllerOpcode::SetScanEnable,
			consts::le_controller::CREATE_CONNECTION => HciLeControllerOpcode::CreateConnection,
			consts::le_controller::CREATE_CONNECTION_CANCEL => HciLeControllerOpcode::CreateConnectionCancel,
			consts::le_controller::READ_WHITELIST_SIZE => HciLeControllerOpcode::ReadWhitelistSize,
			consts::le_controller::CLEAR_WHITELIST => HciLeControllerOpcode::ClearWhitelist,
			consts::le_controller::ADD_DEVICE_TO_WHITELIST => HciLeControllerOpcode::AddDeviceToWhitelist,
			consts::le_controller::REMOVE_DEVICE_FROM_WHITELIST => HciLeControllerOpcode::RemoveDeviceFromWhitelist,
			consts::le_controller::CONNECTION_UPDATE => HciLeControllerOpcode::ConnectionUpdate,
			consts::le_controller::SET_HOST_CHANNEL_CLASSIFICATION => HciLeControllerOpcode::SetHostChannelClassification,
			consts::le_controller::READ_CHANNEL_MAP => HciLeControllerOpcode::ReadChannelMap,
			consts::le_controller::READ_REMOTE_USED_FEATURES => HciLeControllerOpcode::ReadRemoteUsedFeatures,
			consts::le_controller::ENCRYPT => HciLeControllerOpcode::Encrypt,
			consts::le_controller::RAND => HciLeControllerOpcode::Rand,
			consts::le_controller::START_ENCRYPTION => HciLeControllerOpcode::StartEncryption,
			consts::le_controller::LONG_TERM_KEY_REQUEST_REPLY => HciLeControllerOpcode::LongTermKeyRequestReply,
			consts::le_controller::LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY => HciLeControllerOpcode::LongTermKeyRequestNegativeReply,
			consts::le_controller::READ_SUPPORTED_STATES => HciLeControllerOpcode::ReadSupportedStates,
			consts::le_controller::RECEIVER_TEST => HciLeControllerOpcode::ReceiverTest,
			consts::le_controller::TRANSMITTER_TEST => HciLeControllerOpcode::TransmitterTest,
			consts::le_controller::TEST_END => HciLeControllerOpcode::TestEnd,
			value => HciLeControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for HciLeControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			HciLeControllerOpcode::SetEventMask => consts::le_controller::SET_EVENT_MASK,
			HciLeControllerOpcode::ReadBufferSize => consts::le_controller::READ_BUFFER_SIZE,
			HciLeControllerOpcode::ReadLocalSupportedFeatures => consts::le_controller::READ_LOCAL_SUPPORTED_FEATURES,
			HciLeControllerOpcode::SetRandomAddress => consts::le_controller::SET_RANDOM_ADDRESS,
			HciLeControllerOpcode::SetAdvertisingParameters => consts::le_controller::SET_ADVERTISING_PARAMETERS,
			HciLeControllerOpcode::ReadAdvertisingChannelTxPower => consts::le_controller::READ_ADVERTISING_CHANNEL_TX_POWER,
			HciLeControllerOpcode::SetAdvertisingData => consts::le_controller::SET_ADVERTISING_DATA,
			HciLeControllerOpcode::SetScanResponseData => consts::le_controller::SET_SCAN_RESPONSE_DATA,
			HciLeControllerOpcode::SetAdvertiseEnable => consts::le_controller::SET_ADVERTISE_ENABLE,
			HciLeControllerOpcode::SetScanParameters => consts::le_controller::SET_SCAN_PARAMETERS,
			HciLeControllerOpcode::SetScanEnable => consts::le_controller::SET_SCAN_ENABLE,
			HciLeControllerOpcode::CreateConnection => consts::le_controller::CREATE_CONNECTION,
			HciLeControllerOpcode::CreateConnectionCancel => consts::le_controller::CREATE_CONNECTION_CANCEL,
			HciLeControllerOpcode::ReadWhitelistSize => consts::le_controller::READ_WHITELIST_SIZE,
			HciLeControllerOpcode::ClearWhitelist => consts::le_controller::CLEAR_WHITELIST,
			HciLeControllerOpcode::AddDeviceToWhitelist => consts::le_controller::ADD_DEVICE_TO_WHITELIST,
			HciLeControllerOpcode::RemoveDeviceFromWhitelist => consts::le_controller::REMOVE_DEVICE_FROM_WHITELIST,
			HciLeControllerOpcode::ConnectionUpdate => consts::le_controller::CONNECTION_UPDATE,
			HciLeControllerOpcode::SetHostChannelClassification => consts::le_controller::SET_HOST_CHANNEL_CLASSIFICATION,
			HciLeControllerOpcode::ReadChannelMap => consts::le_controller::READ_CHANNEL_MAP,
			HciLeControllerOpcode::ReadRemoteUsedFeatures => consts::le_controller::READ_REMOTE_USED_FEATURES,
			HciLeControllerOpcode::Encrypt => consts::le_controller::ENCRYPT,
			HciLeControllerOpcode::Rand => consts::le_controller::RAND,
			HciLeControllerOpcode::StartEncryption => consts::le_controller::START_ENCRYPTION,
			HciLeControllerOpcode::LongTermKeyRequestReply => consts::le_controller::LONG_TERM_KEY_REQUEST_REPLY,
			HciLeControllerOpcode::LongTermKeyRequestNegativeReply => consts::le_controller::LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY,
			HciLeControllerOpcode::ReadSupportedStates => consts::le_controller::READ_SUPPORTED_STATES,
			HciLeControllerOpcode::ReceiverTest => consts::le_controller::RECEIVER_TEST,
			HciLeControllerOpcode::TransmitterTest => consts::le_controller::TRANSMITTER_TEST,
			HciLeControllerOpcode::TestEnd => consts::le_controller::TEST_END,
			HciLeControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		((consts::le_controller::OGF as u16) << 10) | ocf
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
		(consts::vendor::OGF << 10) | ocf
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// }
