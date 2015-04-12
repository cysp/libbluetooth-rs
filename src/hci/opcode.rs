// extern crate libc;

use std;
use byteorder::{ReadBytesExt,LittleEndian};

mod consts {
	pub use hci::consts::opcodes::*;
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum OpcodeGroup {
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
			Err(_) => return None,
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
