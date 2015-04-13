use types::*;
use hci::types::*;
use hci::opcode::LeControllerOpcode;


define_command!(LeControllerOpcode::SetEventMask, mask: u64 as LeEventMask);
define_command!(LeControllerOpcode::ReadBufferSize);
define_command!(LeControllerOpcode::ReadLocalSupportedFeatures);
define_command!(LeControllerOpcode::SetRandomAddress, address: [u8; 6] as BdAddr);
define_command!(LeControllerOpcode::SetAdvertisingParameters,
	interval_min: u16 as u16, interval_max: u16 as u16,
	advertising_type: u8 as u8,
	own_address_type: u8 as u8, direct_address_type: u8 as u8, direct_address: [u8; 6] as BdAddr,
	channel_map: u8 as u8, filter_policy: u8 as u8);
define_command!(LeControllerOpcode::ReadAdvertisingChannelTxPower);
define_command!(LeControllerOpcode::SetAdvertisingData, data_len: u8, data: [u8; 31]);
define_command!(LeControllerOpcode::SetScanResponseData, data_len: u8, data: [u8; 31]);
define_command!(LeControllerOpcode::SetAdvertiseEnable, enable: u8);
define_command!(LeControllerOpcode::SetScanParameters, scan_type: u8,
	interval: u16, window: u16,
	own_address_type: u8, filter_policy: u8);
define_command!(LeControllerOpcode::SetScanEnable, enable: u8, filter_duplicates: u8);
	// CreateConnection,
define_command!(LeControllerOpcode::CreateConnectionCancel);
define_command!(LeControllerOpcode::ReadWhitelistSize);
define_command!(LeControllerOpcode::ClearWhitelist);
define_command!(LeControllerOpcode::AddDeviceToWhitelist, address_type: u8 as u8, address: [u8; 6] as BdAddr);
define_command!(LeControllerOpcode::RemoveDeviceFromWhitelist, address_type: u8 as u8, address: [u8; 6] as BdAddr);
	// ConnectionUpdate,
	// SetHostChannelClassification,
	// ReadChannelMap,
	// ReadRemoteUsedFeatures,
	// Encrypt,
	// Rand,
	// StartEncryption,
	// LongTermKeyRequestReply,
	// LongTermKeyRequestNegativeReply,
	// ReadSupportedStates,
	// ReceiverTest,
	// TransmitterTest,
	// TestEnd,
