extern crate libc;

use std;
use byteorder::{ReadBytesExt,LittleEndian};


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
		pub const CREATE_CONNECTION_CANCEL: u16 = 0x0008;
		pub const ACCEPT_CONNECTION_REQUEST: u16 = 0x0009;
		pub const REJECT_CONNECTION_REQUEST: u16 = 0x000A;
		pub const LINK_KEY_REQUEST_REPLY: u16 = 0x000B;
		pub const LINK_KEY_REQUEST_NEGATIVE_REPLY: u16 = 0x000C;
		pub const PIN_CODE_REQUEST_REPLY: u16 = 0x000D;
		pub const PIN_CODE_REQUEST_NEGATIVE_REPLY: u16 = 0x000E;
		pub const CHANGE_CONNECTION_PACKET_TYPE_REQUEST: u16 = 0x000F;
		pub const AUTHENTICATION_REQUESTED: u16 = 0x0011;
		pub const SET_CONNECTION_ENCRYPTION: u16 = 0x0013;
		pub const CHANGE_CONNECTION_LINK_KEY: u16 = 0x0015;
		pub const MASTER_LINK_KEY: u16 = 0x0017;
		pub const REMOTE_NAME_REQUEST: u16 = 0x0019;
		pub const REMOTE_NAME_REQUEST_CANCEL: u16 = 0x001A;
		pub const READ_REMOTE_SUPPORTED_FEATURES: u16 = 0x001B;
		pub const READ_REMOTE_EXTENDED_FEATURES: u16 = 0x001C;
		pub const READ_REMOTE_VERSION_INFORMATION: u16 = 0x001D;
		pub const READ_CLOCK_OFFSET: u16 = 0x001F;
		pub const READ_LMP_HANDLE: u16 = 0x0020;
		pub const SETUP_SYNCHRONOUS_CONNECTION: u16 = 0x0028;
		pub const ACCEPT_SYNCHRONOUS_CONNECTION_REQUEST: u16 = 0x0029;
		pub const REJECT_SYNCHRONOUS_CONNECTION_REQUEST: u16 = 0x002A;
		pub const IO_CAPABILITY_REQUEST_REPLY: u16 = 0x002B;
		pub const USER_CONFIRMATION_REQUEST_REPLY: u16 = 0x002C;
		pub const USER_CONFIRMATION_REQUEST_NEGATIVE_REPLY: u16 = 0x002D;
		pub const USER_PASSKEY_REQUEST_REPLY: u16 = 0x002E;
		pub const USER_PASSKEY_REQUEST_NEGATIVE_REPLY: u16 = 0x002F;
		pub const REMOTE_OOB_DATA_REQUEST_REPLY: u16 = 0x0030;
		pub const REMOTE_OOB_DATA_REQUEST_NEGATIVE_REPLY: u16 = 0x0033;
		pub const IO_CAPABILITY_REQUEST_NEGATIVE_REPLY: u16 = 0x0034;
		pub const CREATE_PHYSICAL_LINK: u16 = 0x0035;
		pub const ACCEPT_PHYSICAL_LINK: u16 = 0x0036;
		pub const DISCONNECT_PHYSICAL_LINK: u16 = 0x0037;
		pub const CREATE_LOGICAL_LINK: u16 = 0x0038;
		pub const ACCEPT_LOGICAL_LINK: u16 = 0x0039;
		pub const DISCONNECT_LOGICAL_LINK: u16 = 0x003A;
		pub const LOGICAL_LINK_CANCEL: u16 = 0x003B;
		pub const FLOW_SPEC_MODIFY: u16 = 0x003C;
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

		pub const READ_LOOPBACK_MODE: u16 = 0x0001;
		pub const WRITE_LOOPBACK_MODE: u16 = 0x0002;
		pub const ENABLE_DEVICE_UNDER_TEST_MODE: u16 = 0x0003;
		pub const WRITE_SIMPLE_PAIRING_DEBUG_MODE: u16 = 0x0004;
		pub const ENABLE_AMP_RECEIVER_REPORTS: u16 = 0x0007;
		pub const AMP_TEST_END: u16 = 0x0008;
		pub const AMP_TEST: u16 = 0x0009;
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
pub enum OpcodeGroup {
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

impl From<u16> for OpcodeGroup {
	fn from(value: u16) -> OpcodeGroup {
		match value >> 10 {
			consts::nop::OGF => OpcodeGroup::Nop,
			consts::link_control::OGF => OpcodeGroup::LinkControl,
			consts::link_policy::OGF => OpcodeGroup::LinkPolicy,
			consts::controller::OGF => OpcodeGroup::Controller,
			consts::informational::OGF => OpcodeGroup::Informational,
			consts::status_parameters::OGF => OpcodeGroup::StatusParameters,
			consts::testing::OGF => OpcodeGroup::Testing,
			consts::le_controller::OGF => OpcodeGroup::LeController,
			consts::vendor::OGF => OpcodeGroup::Vendor,
			unknown_ogf => OpcodeGroup::Unknown(unknown_ogf),
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Opcode {
	Nop(NopOpcode),
	LinkControl(LinkControlOpcode),
	LinkPolicy(LinkPolicyOpcode),
	Controller(ControllerOpcode),
	Informational(InformationalOpcode),
	StatusParameters(StatusParametersOpcode),
	Testing(TestingOpcode),
	LeController(LeControllerOpcode),
	Vendor(VendorOpcode),
	Unknown(u16, u16),
}

impl Opcode {
	pub fn from_readable(r: &mut std::io::Read) -> Option<Opcode> {
		let opcode = match r.read_u16::<LittleEndian>() {
			Ok(opcode) => opcode,
			Err(e) => return None,
		};
		Some(Opcode::from(opcode))
	}
}

impl From<u16> for Opcode {
	fn from(value: u16) -> Opcode {
		let ocf = value & ((1 << 10) - 1);
		match OpcodeGroup::from(value) {
			OpcodeGroup::Nop => Opcode::Nop(NopOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::LinkControl => Opcode::LinkControl(LinkControlOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::LinkPolicy => Opcode::LinkPolicy(LinkPolicyOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::Controller => Opcode::Controller(ControllerOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::Informational => Opcode::Informational(InformationalOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::StatusParameters => Opcode::StatusParameters(StatusParametersOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::Testing => Opcode::Testing(TestingOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::LeController => Opcode::LeController(LeControllerOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::Vendor => Opcode::Vendor(VendorOpcode::from_trusted_u16(ocf)),
			OpcodeGroup::Unknown(ogf) => return Opcode::Unknown(ogf, ocf),
		}
	}
}

impl Into<u16> for Opcode {
	fn into(self) -> u16 {
		match self {
			Opcode::Nop(opcode) => opcode.into(),
			Opcode::LinkControl(opcode) => opcode.into(),
			Opcode::LinkPolicy(opcode) => opcode.into(),
			Opcode::Controller(opcode) => opcode.into(),
			Opcode::Informational(opcode) => opcode.into(),
			Opcode::StatusParameters(opcode) => opcode.into(),
			Opcode::Testing(opcode) => opcode.into(),
			Opcode::LeController(opcode) => opcode.into(),
			Opcode::Vendor(opcode) => opcode.into(),
			Opcode::Unknown(ogf, ocf) => (ogf << 10) | ocf,
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NopOpcode {
	Nop,
	Unknown(u16),
}

impl NopOpcode {
	fn from_trusted_u16(value: u16) -> NopOpcode {
		match value & ((1 << 10) - 1) {
			0 => NopOpcode::Nop,
			value => NopOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for NopOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			NopOpcode::Nop => consts::nop::NOP,
			NopOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::nop::OGF << 10) | ocf
	}
}

impl Into<Opcode> for NopOpcode {
	fn into(self) -> Opcode {
		Opcode::Nop(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LinkControlOpcode {
	Unknown(u16),
}

impl LinkControlOpcode {
	fn from_trusted_u16(value: u16) -> LinkControlOpcode {
		match value & ((1 << 10) - 1) {
			value => LinkControlOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for LinkControlOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			LinkControlOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::link_control::OGF << 10) | ocf
	}
}

impl Into<Opcode> for LinkControlOpcode {
	fn into(self) -> Opcode {
		Opcode::LinkControl(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LinkPolicyOpcode {
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

impl LinkPolicyOpcode {
	fn from_trusted_u16(value: u16) -> LinkPolicyOpcode {
		match value & ((1 << 10) - 1) {
			consts::link_policy::HOLD_MODE => LinkPolicyOpcode::HoldMode,
			consts::link_policy::SNIFF_MODE => LinkPolicyOpcode::SniffMode,
			consts::link_policy::EXIT_SNIFF_MODE => LinkPolicyOpcode::ExitSniffMode,
			consts::link_policy::PARK_STATE => LinkPolicyOpcode::ParkState,
			consts::link_policy::EXIT_PARK_STATE => LinkPolicyOpcode::ExitParkState,
			consts::link_policy::QOS_SETUP => LinkPolicyOpcode::QosSetup,
			consts::link_policy::ROLE_DISCOVERY => LinkPolicyOpcode::RoleDiscovery,
			consts::link_policy::SWITCH_ROLE => LinkPolicyOpcode::SwitchRole,
			consts::link_policy::READ_LINK_POLICY_SETTINGS => LinkPolicyOpcode::ReadLinkPolicySettings,
			consts::link_policy::WRITE_LINK_POLICY_SETTINGS => LinkPolicyOpcode::WriteLinkPolicySettings,
			consts::link_policy::READ_DEFAULT_LINK_POLICY_SETTINGS => LinkPolicyOpcode::ReadDefaultLinkPolicySettings,
			consts::link_policy::WRITE_DEFAULT_LINK_POLICY_SETTINGS => LinkPolicyOpcode::WriteDefaultLinkPolicySettings,
			consts::link_policy::FLOW_SPECIFICATION => LinkPolicyOpcode::FlowSpecification,
			consts::link_policy::SNIFF_SUBRATING => LinkPolicyOpcode::SniffSubrating,
			value => LinkPolicyOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for LinkPolicyOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			LinkPolicyOpcode::HoldMode => consts::link_policy::HOLD_MODE,
			LinkPolicyOpcode::SniffMode => consts::link_policy::SNIFF_MODE,
			LinkPolicyOpcode::ExitSniffMode => consts::link_policy::EXIT_SNIFF_MODE,
			LinkPolicyOpcode::ParkState => consts::link_policy::PARK_STATE,
			LinkPolicyOpcode::ExitParkState => consts::link_policy::EXIT_PARK_STATE,
			LinkPolicyOpcode::QosSetup => consts::link_policy::QOS_SETUP,
			LinkPolicyOpcode::RoleDiscovery => consts::link_policy::ROLE_DISCOVERY,
			LinkPolicyOpcode::SwitchRole => consts::link_policy::SWITCH_ROLE,
			LinkPolicyOpcode::ReadLinkPolicySettings => consts::link_policy::READ_LINK_POLICY_SETTINGS,
			LinkPolicyOpcode::WriteLinkPolicySettings => consts::link_policy::WRITE_LINK_POLICY_SETTINGS,
			LinkPolicyOpcode::ReadDefaultLinkPolicySettings => consts::link_policy::READ_DEFAULT_LINK_POLICY_SETTINGS,
			LinkPolicyOpcode::WriteDefaultLinkPolicySettings => consts::link_policy::WRITE_DEFAULT_LINK_POLICY_SETTINGS,
			LinkPolicyOpcode::FlowSpecification => consts::link_policy::FLOW_SPECIFICATION,
			LinkPolicyOpcode::SniffSubrating => consts::link_policy::SNIFF_SUBRATING,
			LinkPolicyOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::link_policy::OGF << 10) | ocf
	}
}

impl Into<Opcode> for LinkPolicyOpcode {
	fn into(self) -> Opcode {
		Opcode::LinkPolicy(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ControllerOpcode {
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

impl ControllerOpcode {
	fn from_trusted_u16(value: u16) -> ControllerOpcode {
		match value & ((1 << 10) - 1) {
			consts::controller::SET_EVENT_MASK => ControllerOpcode::SetEventMask,
			consts::controller::RESET => ControllerOpcode::Reset,
			consts::controller::SET_EVENT_FILTER => ControllerOpcode::SetEventFilter,
			consts::controller::FLUSH => ControllerOpcode::Flush,
			consts::controller::READ_PIN_TYPE => ControllerOpcode::ReadPINType,
			consts::controller::WRITE_PIN_TYPE => ControllerOpcode::WritePINType,
			consts::controller::CREATE_NEW_UNIT_KEY => ControllerOpcode::CreateNewUnitKey,
			consts::controller::READ_STORED_LINK_KEY => ControllerOpcode::ReadStoredLinkKey,
			consts::controller::WRITE_STORED_LINK_KEY => ControllerOpcode::WriteStoredLinkKey,
			consts::controller::DELETE_STORED_LINK_KEY => ControllerOpcode::DeleteStoredLinkKey,
			consts::controller::WRITE_LOCAL_NAME => ControllerOpcode::WriteLocalName,
			consts::controller::READ_LOCAL_NAME => ControllerOpcode::ReadLocalName,
			consts::controller::READ_CONNECTION_ACCEPT_TIMEOUT => ControllerOpcode::ReadConnectionAcceptTimeout,
			consts::controller::WRITE_CONNECTION_ACCEPT_TIMEOUT => ControllerOpcode::WriteConnectionAcceptTimeout,
			consts::controller::READ_SCAN_ENABLE => ControllerOpcode::ReadScanEnable,
			consts::controller::WRITE_SCAN_ENABLE => ControllerOpcode::WriteScanEnable,
			consts::controller::READ_PAGE_SCAN_ACTIVITY => ControllerOpcode::ReadPageScanActivity,
			consts::controller::WRITE_PAGE_SCAN_ACTIVITY => ControllerOpcode::WritePageScanActivity,
			consts::controller::READ_INQUIRY_SCAN_ACTIVITY => ControllerOpcode::ReadInquiryScanActivity,
			consts::controller::WRITE_INQUIRY_SCAN_ACTIVITY => ControllerOpcode::WriteInquiryScanActivity,
			consts::controller::READ_CLASS_OF_DEVICE => ControllerOpcode::ReadClassOfDevice,
			consts::controller::WRITE_CLASS_OF_DEVICE => ControllerOpcode::WriteClassOfDevice,
			consts::controller::READ_VOICE_SETTING => ControllerOpcode::ReadVoiceSetting,
			consts::controller::WRITE_VOICE_SETTING => ControllerOpcode::WriteVoiceSetting,
			consts::controller::READ_NUMBER_OF_SUPPORTED_IAC => ControllerOpcode::ReadNumberOfSupportedIac,
			consts::controller::READ_CURRENT_IAC_LAP => ControllerOpcode::ReadCurrentIacLap,
			consts::controller::READ_INQUIRY_SCAN_TYPE => ControllerOpcode::ReadInquiryScanType,
			consts::controller::WRITE_INQUIRY_SCAN_TYPE => ControllerOpcode::WriteInquiryScanType,
			consts::controller::READ_INQUIRY_MODE => ControllerOpcode::ReadInquiryMode,
			consts::controller::WRITE_INQUIRY_MODE => ControllerOpcode::WriteInquiryMode,
			consts::controller::READ_PAGE_SCAN_TYPE => ControllerOpcode::ReadPageScanType,
			consts::controller::WRITE_PAGE_SCAN_TYPE => ControllerOpcode::WritePageScanType,
			consts::controller::READ_AFH_CHANNEL_ASSESSMENT_MODE => ControllerOpcode::ReadAfhChannelAssessmentMode,
			consts::controller::WRITE_AFH_CHANNEL_ASSESSMENT_MODE => ControllerOpcode::WriteAfhChannelAssessmentMode,
			consts::controller::READ_EXTENDED_INQUIRY_RESPONSE => ControllerOpcode::ReadExtendedInquiryResponse,
			consts::controller::WRITE_EXTENDED_INQUIRY_RESPONSE => ControllerOpcode::WriteExtendedInquiryResponse,
			consts::controller::REFRESH_ENCRYPTION_KEY => ControllerOpcode::RefreshEncryptionKey,
			consts::controller::READ_SIMPLE_PAIRING_MODE => ControllerOpcode::ReadSimplePairingMode,
			consts::controller::WRITE_SIMPLE_PAIRING_MODE => ControllerOpcode::WriteSimplePairingMode,
			consts::controller::READ_LOCAL_OOB_DATA => ControllerOpcode::ReadLocalOobData,
			consts::controller::READ_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL => ControllerOpcode::ReadInquiryResponseTransmitPowerLevel,
			consts::controller::WRITE_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL => ControllerOpcode::WriteInquiryResponseTransmitPowerLevel,
			value => ControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for ControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			ControllerOpcode::SetEventMask => consts::controller::SET_EVENT_MASK,
			ControllerOpcode::Reset => consts::controller::RESET,
			ControllerOpcode::SetEventFilter => consts::controller::SET_EVENT_FILTER,
			ControllerOpcode::Flush => consts::controller::FLUSH,
			ControllerOpcode::ReadPINType => consts::controller::READ_PIN_TYPE,
			ControllerOpcode::WritePINType => consts::controller::WRITE_PIN_TYPE,
			ControllerOpcode::CreateNewUnitKey => consts::controller::CREATE_NEW_UNIT_KEY,
			ControllerOpcode::ReadStoredLinkKey => consts::controller::READ_STORED_LINK_KEY,
			ControllerOpcode::WriteStoredLinkKey => consts::controller::WRITE_STORED_LINK_KEY,
			ControllerOpcode::DeleteStoredLinkKey => consts::controller::DELETE_STORED_LINK_KEY,
			ControllerOpcode::WriteLocalName => consts::controller::WRITE_LOCAL_NAME,
			ControllerOpcode::ReadLocalName => consts::controller::READ_LOCAL_NAME,
			ControllerOpcode::ReadConnectionAcceptTimeout => consts::controller::READ_CONNECTION_ACCEPT_TIMEOUT,
			ControllerOpcode::WriteConnectionAcceptTimeout => consts::controller::WRITE_CONNECTION_ACCEPT_TIMEOUT,
			ControllerOpcode::ReadScanEnable => consts::controller::READ_SCAN_ENABLE,
			ControllerOpcode::WriteScanEnable => consts::controller::WRITE_SCAN_ENABLE,
			ControllerOpcode::ReadPageScanActivity => consts::controller::READ_PAGE_SCAN_ACTIVITY,
			ControllerOpcode::WritePageScanActivity => consts::controller::WRITE_PAGE_SCAN_ACTIVITY,
			ControllerOpcode::ReadInquiryScanActivity => consts::controller::READ_INQUIRY_SCAN_ACTIVITY,
			ControllerOpcode::WriteInquiryScanActivity => consts::controller::WRITE_INQUIRY_SCAN_ACTIVITY,
			ControllerOpcode::ReadClassOfDevice => consts::controller::READ_CLASS_OF_DEVICE,
			ControllerOpcode::WriteClassOfDevice => consts::controller::WRITE_CLASS_OF_DEVICE,
			ControllerOpcode::ReadVoiceSetting => consts::controller::READ_VOICE_SETTING,
			ControllerOpcode::WriteVoiceSetting => consts::controller::WRITE_VOICE_SETTING,
			ControllerOpcode::ReadNumberOfSupportedIac => consts::controller::READ_NUMBER_OF_SUPPORTED_IAC,
			ControllerOpcode::ReadCurrentIacLap => consts::controller::READ_CURRENT_IAC_LAP,
			ControllerOpcode::ReadInquiryScanType => consts::controller::READ_INQUIRY_SCAN_TYPE,
			ControllerOpcode::WriteInquiryScanType => consts::controller::WRITE_INQUIRY_SCAN_TYPE,
			ControllerOpcode::ReadInquiryMode => consts::controller::READ_INQUIRY_MODE,
			ControllerOpcode::WriteInquiryMode => consts::controller::WRITE_INQUIRY_MODE,
			ControllerOpcode::ReadPageScanType => consts::controller::READ_PAGE_SCAN_TYPE,
			ControllerOpcode::WritePageScanType => consts::controller::WRITE_PAGE_SCAN_TYPE,
			ControllerOpcode::ReadAfhChannelAssessmentMode => consts::controller::READ_AFH_CHANNEL_ASSESSMENT_MODE,
			ControllerOpcode::WriteAfhChannelAssessmentMode => consts::controller::WRITE_AFH_CHANNEL_ASSESSMENT_MODE,
			ControllerOpcode::ReadExtendedInquiryResponse => consts::controller::READ_EXTENDED_INQUIRY_RESPONSE,
			ControllerOpcode::WriteExtendedInquiryResponse => consts::controller::WRITE_EXTENDED_INQUIRY_RESPONSE,
			ControllerOpcode::RefreshEncryptionKey => consts::controller::REFRESH_ENCRYPTION_KEY,
			ControllerOpcode::ReadSimplePairingMode => consts::controller::READ_SIMPLE_PAIRING_MODE,
			ControllerOpcode::WriteSimplePairingMode => consts::controller::WRITE_SIMPLE_PAIRING_MODE,
			ControllerOpcode::ReadLocalOobData => consts::controller::READ_LOCAL_OOB_DATA,
			ControllerOpcode::ReadInquiryResponseTransmitPowerLevel => consts::controller::READ_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL,
			ControllerOpcode::WriteInquiryResponseTransmitPowerLevel => consts::controller::WRITE_INQUIRY_RESPONSE_TRANSMIT_POWER_LEVEL,
			ControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::controller::OGF << 10) | ocf
	}
}

impl Into<Opcode> for ControllerOpcode {
	fn into(self) -> Opcode {
		Opcode::Controller(self)
	}
}

#[cfg(test)]
mod controlleropcode_tests {
	use super::*;

	#[test]
	fn test_controlleropcode_seteventmask() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::SetEventMask).into();
			assert_eq!(oc, 0x0C01);
			let oc: u16 = ControllerOpcode::SetEventMask.into();
			assert_eq!(oc, 0x0C01);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::SetEventMask));
		}
	}

	#[test]
	fn test_controlleropcode_reset() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::Reset).into();
			assert_eq!(oc, 0x0C03);
			let oc: u16 = ControllerOpcode::Reset.into();
			assert_eq!(oc, 0x0C03);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::Reset));
		}
	}

	#[test]
	fn test_controlleropcode_seteventfilter() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::SetEventFilter).into();
			assert_eq!(oc, 0x0C05);
			let oc: u16 = ControllerOpcode::SetEventFilter.into();
			assert_eq!(oc, 0x0C05);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::SetEventFilter));
		}
	}

	#[test]
	fn test_controlleropcode_flush() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::Flush).into();
			assert_eq!(oc, 0x0C08);
			let oc: u16 = ControllerOpcode::Flush.into();
			assert_eq!(oc, 0x0C08);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::Flush));
		}
	}

	#[test]
	fn test_controlleropcode_readpintype() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::ReadPINType).into();
			assert_eq!(oc, 0x0C09);
			let oc: u16 = ControllerOpcode::ReadPINType.into();
			assert_eq!(oc, 0x0C09);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::ReadPINType));
		}
	}

	#[test]
	fn test_controlleropcode_writepintype() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::WritePINType).into();
			assert_eq!(oc, 0x0C0A);
			let oc: u16 = ControllerOpcode::WritePINType.into();
			assert_eq!(oc, 0x0C0A);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::WritePINType));
		}
	}

	#[test]
	fn test_controlleropcode_unknown() {
		{
			let oc: u16 = Opcode::Controller(ControllerOpcode::Unknown(0x67)).into();
			assert_eq!(oc, 0x0C67);
			let oc: u16 = ControllerOpcode::Unknown(0x67).into();
			assert_eq!(oc, 0x0C67);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Controller(ControllerOpcode::Unknown(0x67)));
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum InformationalOpcode {
	ReadLocalVersionInformation,
	ReadLocalSupportedCommands,
	ReadLocalSupportedFeatures,
	ReadLocalExtendedFeatures,
	ReadBufferSize,
	ReadBdAddr,
	ReadDataBlockSize,
	Unknown(u16),
}

impl InformationalOpcode {
	fn from_trusted_u16(value: u16) -> InformationalOpcode {
		match value & ((1 << 10) - 1) {
			consts::informational::READ_LOCAL_VERSION_INFORMATION => InformationalOpcode::ReadLocalVersionInformation,
			consts::informational::READ_LOCAL_SUPPORTED_COMMANDS => InformationalOpcode::ReadLocalSupportedCommands,
			consts::informational::READ_LOCAL_SUPPORTED_FEATURES => InformationalOpcode::ReadLocalSupportedFeatures,
			consts::informational::READ_LOCAL_EXTENDED_FEATURES => InformationalOpcode::ReadLocalExtendedFeatures,
			consts::informational::READ_BUFFER_SIZE => InformationalOpcode::ReadBufferSize,
			consts::informational::READ_BD_ADDR => InformationalOpcode::ReadBdAddr,
			consts::informational::READ_DATA_BLOCK_SIZE => InformationalOpcode::ReadDataBlockSize,
			value => InformationalOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for InformationalOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			InformationalOpcode::ReadLocalVersionInformation => consts::informational::READ_LOCAL_VERSION_INFORMATION,
			InformationalOpcode::ReadLocalSupportedCommands => consts::informational::READ_LOCAL_SUPPORTED_COMMANDS,
			InformationalOpcode::ReadLocalSupportedFeatures => consts::informational::READ_LOCAL_SUPPORTED_FEATURES,
			InformationalOpcode::ReadLocalExtendedFeatures => consts::informational::READ_LOCAL_EXTENDED_FEATURES,
			InformationalOpcode::ReadBufferSize => consts::informational::READ_BUFFER_SIZE,
			InformationalOpcode::ReadBdAddr => consts::informational::READ_BD_ADDR,
			InformationalOpcode::ReadDataBlockSize => consts::informational::READ_DATA_BLOCK_SIZE,
			InformationalOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::informational::OGF << 10) | ocf
	}
}

impl Into<Opcode> for InformationalOpcode {
	fn into(self) -> Opcode {
		Opcode::Informational(self)
	}
}

#[cfg(test)]
mod informationalopcode_tests {
	use super::*;

	#[test]
	fn test_informationalopcode_readlocalversioninformation() {
		{
			let oc: u16 = Opcode::Informational(InformationalOpcode::ReadLocalVersionInformation).into();
			assert_eq!(oc, 0x1001);
			let oc: u16 = InformationalOpcode::ReadLocalVersionInformation.into();
			assert_eq!(oc, 0x1001);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Informational(InformationalOpcode::ReadLocalVersionInformation));
		}
	}

	#[test]
	fn test_informationalopcode_readlocalsupportedcommands() {
		{
			let oc: u16 = Opcode::Informational(InformationalOpcode::ReadLocalSupportedCommands).into();
			assert_eq!(oc, 0x1002);
			let oc: u16 = InformationalOpcode::ReadLocalSupportedCommands.into();
			assert_eq!(oc, 0x1002);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Informational(InformationalOpcode::ReadLocalSupportedCommands));
		}
	}

	#[test]
	fn test_informationalopcode_readlocalsupportedfeatures() {
		{
			let oc: u16 = Opcode::Informational(InformationalOpcode::ReadLocalSupportedFeatures).into();
			assert_eq!(oc, 0x1003);
			let oc: u16 = InformationalOpcode::ReadLocalSupportedFeatures.into();
			assert_eq!(oc, 0x1003);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Informational(InformationalOpcode::ReadLocalSupportedFeatures));
		}
	}

	#[test]
	fn test_informationalopcode_readlocalextendedfeatures() {
		{
			let oc: u16 = Opcode::Informational(InformationalOpcode::ReadLocalExtendedFeatures).into();
			assert_eq!(oc, 0x1004);
			let oc: u16 = InformationalOpcode::ReadLocalExtendedFeatures.into();
			assert_eq!(oc, 0x1004);
			let oc = Opcode::from(oc);
			assert_eq!(oc, Opcode::Informational(InformationalOpcode::ReadLocalExtendedFeatures));
		}
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StatusParametersOpcode {
	Unknown(u16),
}

impl StatusParametersOpcode {
	fn from_trusted_u16(value: u16) -> StatusParametersOpcode {
		match value & ((1 << 10) - 1) {
			value => StatusParametersOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for StatusParametersOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			StatusParametersOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::status_parameters::OGF << 10) | ocf
	}
}

impl Into<Opcode> for StatusParametersOpcode {
	fn into(self) -> Opcode {
		Opcode::StatusParameters(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum TestingOpcode {
	Unknown(u16),
}

impl TestingOpcode {
	fn from_trusted_u16(value: u16) -> TestingOpcode {
		match value & ((1 << 10) - 1) {
			value => TestingOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for TestingOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			TestingOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::testing::OGF << 10) | ocf
	}
}

impl Into<Opcode> for TestingOpcode {
	fn into(self) -> Opcode {
		Opcode::Testing(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LeControllerOpcode {
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

impl LeControllerOpcode {
	fn from_trusted_u16(value: u16) -> LeControllerOpcode {
		match value & ((1 << 10) - 1) {
			consts::le_controller::SET_EVENT_MASK => LeControllerOpcode::SetEventMask,
			consts::le_controller::READ_BUFFER_SIZE => LeControllerOpcode::ReadBufferSize,
			consts::le_controller::READ_LOCAL_SUPPORTED_FEATURES => LeControllerOpcode::ReadLocalSupportedFeatures,
			consts::le_controller::SET_RANDOM_ADDRESS => LeControllerOpcode::SetRandomAddress,
			consts::le_controller::SET_ADVERTISING_PARAMETERS => LeControllerOpcode::SetAdvertisingParameters,
			consts::le_controller::READ_ADVERTISING_CHANNEL_TX_POWER => LeControllerOpcode::ReadAdvertisingChannelTxPower,
			consts::le_controller::SET_ADVERTISING_DATA => LeControllerOpcode::SetAdvertisingData,
			consts::le_controller::SET_SCAN_RESPONSE_DATA => LeControllerOpcode::SetScanResponseData,
			consts::le_controller::SET_ADVERTISE_ENABLE => LeControllerOpcode::SetAdvertiseEnable,
			consts::le_controller::SET_SCAN_PARAMETERS => LeControllerOpcode::SetScanParameters,
			consts::le_controller::SET_SCAN_ENABLE => LeControllerOpcode::SetScanEnable,
			consts::le_controller::CREATE_CONNECTION => LeControllerOpcode::CreateConnection,
			consts::le_controller::CREATE_CONNECTION_CANCEL => LeControllerOpcode::CreateConnectionCancel,
			consts::le_controller::READ_WHITELIST_SIZE => LeControllerOpcode::ReadWhitelistSize,
			consts::le_controller::CLEAR_WHITELIST => LeControllerOpcode::ClearWhitelist,
			consts::le_controller::ADD_DEVICE_TO_WHITELIST => LeControllerOpcode::AddDeviceToWhitelist,
			consts::le_controller::REMOVE_DEVICE_FROM_WHITELIST => LeControllerOpcode::RemoveDeviceFromWhitelist,
			consts::le_controller::CONNECTION_UPDATE => LeControllerOpcode::ConnectionUpdate,
			consts::le_controller::SET_HOST_CHANNEL_CLASSIFICATION => LeControllerOpcode::SetHostChannelClassification,
			consts::le_controller::READ_CHANNEL_MAP => LeControllerOpcode::ReadChannelMap,
			consts::le_controller::READ_REMOTE_USED_FEATURES => LeControllerOpcode::ReadRemoteUsedFeatures,
			consts::le_controller::ENCRYPT => LeControllerOpcode::Encrypt,
			consts::le_controller::RAND => LeControllerOpcode::Rand,
			consts::le_controller::START_ENCRYPTION => LeControllerOpcode::StartEncryption,
			consts::le_controller::LONG_TERM_KEY_REQUEST_REPLY => LeControllerOpcode::LongTermKeyRequestReply,
			consts::le_controller::LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY => LeControllerOpcode::LongTermKeyRequestNegativeReply,
			consts::le_controller::READ_SUPPORTED_STATES => LeControllerOpcode::ReadSupportedStates,
			consts::le_controller::RECEIVER_TEST => LeControllerOpcode::ReceiverTest,
			consts::le_controller::TRANSMITTER_TEST => LeControllerOpcode::TransmitterTest,
			consts::le_controller::TEST_END => LeControllerOpcode::TestEnd,
			value => LeControllerOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for LeControllerOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			LeControllerOpcode::SetEventMask => consts::le_controller::SET_EVENT_MASK,
			LeControllerOpcode::ReadBufferSize => consts::le_controller::READ_BUFFER_SIZE,
			LeControllerOpcode::ReadLocalSupportedFeatures => consts::le_controller::READ_LOCAL_SUPPORTED_FEATURES,
			LeControllerOpcode::SetRandomAddress => consts::le_controller::SET_RANDOM_ADDRESS,
			LeControllerOpcode::SetAdvertisingParameters => consts::le_controller::SET_ADVERTISING_PARAMETERS,
			LeControllerOpcode::ReadAdvertisingChannelTxPower => consts::le_controller::READ_ADVERTISING_CHANNEL_TX_POWER,
			LeControllerOpcode::SetAdvertisingData => consts::le_controller::SET_ADVERTISING_DATA,
			LeControllerOpcode::SetScanResponseData => consts::le_controller::SET_SCAN_RESPONSE_DATA,
			LeControllerOpcode::SetAdvertiseEnable => consts::le_controller::SET_ADVERTISE_ENABLE,
			LeControllerOpcode::SetScanParameters => consts::le_controller::SET_SCAN_PARAMETERS,
			LeControllerOpcode::SetScanEnable => consts::le_controller::SET_SCAN_ENABLE,
			LeControllerOpcode::CreateConnection => consts::le_controller::CREATE_CONNECTION,
			LeControllerOpcode::CreateConnectionCancel => consts::le_controller::CREATE_CONNECTION_CANCEL,
			LeControllerOpcode::ReadWhitelistSize => consts::le_controller::READ_WHITELIST_SIZE,
			LeControllerOpcode::ClearWhitelist => consts::le_controller::CLEAR_WHITELIST,
			LeControllerOpcode::AddDeviceToWhitelist => consts::le_controller::ADD_DEVICE_TO_WHITELIST,
			LeControllerOpcode::RemoveDeviceFromWhitelist => consts::le_controller::REMOVE_DEVICE_FROM_WHITELIST,
			LeControllerOpcode::ConnectionUpdate => consts::le_controller::CONNECTION_UPDATE,
			LeControllerOpcode::SetHostChannelClassification => consts::le_controller::SET_HOST_CHANNEL_CLASSIFICATION,
			LeControllerOpcode::ReadChannelMap => consts::le_controller::READ_CHANNEL_MAP,
			LeControllerOpcode::ReadRemoteUsedFeatures => consts::le_controller::READ_REMOTE_USED_FEATURES,
			LeControllerOpcode::Encrypt => consts::le_controller::ENCRYPT,
			LeControllerOpcode::Rand => consts::le_controller::RAND,
			LeControllerOpcode::StartEncryption => consts::le_controller::START_ENCRYPTION,
			LeControllerOpcode::LongTermKeyRequestReply => consts::le_controller::LONG_TERM_KEY_REQUEST_REPLY,
			LeControllerOpcode::LongTermKeyRequestNegativeReply => consts::le_controller::LONG_TERM_KEY_REQUEST_NEGATIVE_REPLY,
			LeControllerOpcode::ReadSupportedStates => consts::le_controller::READ_SUPPORTED_STATES,
			LeControllerOpcode::ReceiverTest => consts::le_controller::RECEIVER_TEST,
			LeControllerOpcode::TransmitterTest => consts::le_controller::TRANSMITTER_TEST,
			LeControllerOpcode::TestEnd => consts::le_controller::TEST_END,
			LeControllerOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		((consts::le_controller::OGF as u16) << 10) | ocf
	}
}

impl Into<Opcode> for LeControllerOpcode {
	fn into(self) -> Opcode {
		Opcode::LeController(self)
	}
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum VendorOpcode {
	Unknown(u16),
}

impl VendorOpcode {
	fn from_trusted_u16(value: u16) -> VendorOpcode {
		match value & ((1 << 10) - 1) {
			value => VendorOpcode::Unknown(value),
		}
	}
}

impl Into<u16> for VendorOpcode {
	fn into(self) -> u16 {
		let ocf = match self {
			VendorOpcode::Unknown(ocf) => ocf,
		} & ((1 << 10) - 1);
		(consts::vendor::OGF << 10) | ocf
	}
}

impl Into<Opcode> for VendorOpcode {
	fn into(self) -> Opcode {
		Opcode::Vendor(self)
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// }
