use hci::types::*;
use hci::opcode::ControllerOpcode;


define_command!(ControllerOpcode::SetEventMask, mask: u64 as EventMask);
define_command!(ControllerOpcode::Reset);
define_command!(ControllerOpcode::SetEventFilter, filter: u64);
define_command!(ControllerOpcode::Flush);
define_command!(ControllerOpcode::WriteInquiryMode, mode: u8 as InquiryMode);
define_command!(ControllerOpcode::ReadPINType);
define_command!(ControllerOpcode::WritePINType, pin_type: u8);
	// CreateNewUnitKey,
	// ReadStoredLinkKey,
	// WriteStoredLinkKey,
	// DeleteStoredLinkKey,
	// WriteLocalName,
	// ReadLocalName,
	// ReadConnectionAcceptTimeout,
	// WriteConnectionAcceptTimeout,
	// ReadScanEnable,
	// WriteScanEnable,
	// ReadPageScanActivity,
	// WritePageScanActivity,
	// ReadInquiryScanActivity,
	// WriteInquiryScanActivity,
	// ReadClassOfDevice,
	// WriteClassOfDevice,
	// ReadVoiceSetting,
	// WriteVoiceSetting,
	// ReadNumberOfSupportedIac,
	// ReadCurrentIacLap,
	// ReadInquiryScanType,
	// WriteInquiryScanType,
	// ReadInquiryMode,
	// WriteInquiryMode,
	// ReadPageScanType,
	// WritePageScanType,
	// ReadAfhChannelAssessmentMode,
	// WriteAfhChannelAssessmentMode,
	// ReadExtendedInquiryResponse,
	// WriteExtendedInquiryResponse,
	// RefreshEncryptionKey,
	// ReadSimplePairingMode,
	// WriteSimplePairingMode,
	// ReadLocalOobData,
	// ReadInquiryResponseTransmitPowerLevel,
	// WriteInquiryResponseTransmitPowerLevel,
