use std;


mod consts {
	pub mod event_mask {
		pub const INQUIRY_COMPLETE: u64 = 0x0000000000000001;
		pub const INQUIRY_RESULT: u64 = 0x0000000000000002;
		pub const CONNECTION_COMPLETE: u64 = 0x0000000000000004;
		pub const CONNECTION_REQUEST: u64 = 0x0000000000000008;
		pub const DISCONNECTION_COMPLETE: u64 = 0x0000000000000010;
		pub const AUTHENTICATION_COMPLETE: u64 = 0x0000000000000020;
		pub const REMOTE_NAME_REQUEST_COMPLETE: u64 = 0x0000000000000040;
		pub const ENCRYPTION_CHANGE: u64 = 0x0000000000000080;
		pub const CHANGE_CONNECTION_LINK_KEY_COMPLETE: u64 = 0x0000000000000100;
		pub const MASTER_LINK_KEY_COMPLETE: u64 = 0x0000000000000200;
		pub const READ_REMOTE_SUPPORTED_FEATURES_COMPLETE: u64 = 0x0000000000000400;
		pub const READ_REMOTE_VERSION_INFORMATION_COMPLETE: u64 = 0x0000000000000800;
		pub const QOS_SETUP_COMPLETE: u64 = 0x0000000000001000;
		// pub const COMMAND_COMPLETE: u64 = 0x0000000000002000;
		// pub const COMMAND_STATUS: u64 = 0x0000000000004000;
		pub const HARDWARE_ERROR: u64 = 0x0000000000008000;
		pub const FLUSH_OCCURRED: u64 = 0x0000000000010000;
		pub const ROLE_CHANGE: u64 = 0x0000000000020000;
		pub const MODE_CHANGE: u64 = 0x0000000000080000;
		pub const RETURN_LINK_KEYS: u64 = 0x0000000000100000;
		pub const PIN_CODE_REQUEST: u64 = 0x0000000000200000;
		pub const LINK_KEY_REQUEST: u64 = 0x0000000000400000;
		pub const LINK_KEY_NOTIFICATION: u64 = 0x0000000000800000;
		pub const LOOPBACK_COMMAND: u64 = 0x0000000001000000;
		pub const DATA_BUFFER_OVERFLOW: u64 = 0x0000000002000000;
		pub const MAX_SLOTS_CHANGE: u64 = 0x0000000004000000;
		pub const READ_CLOCK_OFFSET_COMPLETE: u64 = 0x0000000008000000;
		pub const CONNECTION_PACKET_TYPE_CHANGED: u64 = 0x0000000010000000;
		pub const QOS_VIOLATION: u64 = 0x0000000020000000;
		pub const PAGE_SCAN_MODE_CHANGE: u64 = 0x0000000040000000; // deprecated
		pub const PAGE_SCAN_REPETITION_MODE_CHANGE: u64 = 0x0000000080000000;
		pub const FLOW_SPECIFICATION_COMPLETE: u64 = 0x0000000100000000;
		pub const INQUIRY_RESULT_WITH_RSSI: u64 = 0x0000000200000000;
		pub const READ_REMOTE_EXTENDED_FEATURES_COMPLETE: u64 = 0x0000000400000000;
		pub const SYNCHRONOUS_CONNECTION_COMPLETE: u64 = 0x0000080000000000;
		pub const SYNCHRONOUS_CONNECTION_CHANGED: u64 = 0x0000100000000000;
		pub const SNIFF_SUBRATING: u64 = 0x0000200000000000;
		pub const EXTENDED_INQUIRY_RESULT: u64 = 0x0000400000000000;
		pub const ENCRYPTION_KEY_REFRESH_COMPLETE: u64 = 0x0000800000000000;
		pub const IO_CAPABILITY_REQUEST: u64 = 0x0001000000000000;
		pub const IO_CAPABILITY_REQUEST_REPLY: u64 = 0x0002000000000000;
		pub const USER_CONFIRMATION_REQUEST: u64 = 0x0004000000000000;
		pub const USER_PASSKEY_REQUEST: u64 = 0x0008000000000000;
		pub const REMOTE_OOB_DATA_REQUEST: u64 = 0x0010000000000000;
		pub const SIMPLE_PAIRING_COMPLETE: u64 = 0x0020000000000000;
		pub const LINK_SUPERVISION_TIMEOUT_CHANGED: u64 = 0x0080000000000000;
		pub const ENHANCED_FLUSH_COMPLETE: u64 = 0x0100000000000000;
		pub const USER_PASSKEY_NOTIFICATION: u64 = 0x0400000000000000;
		pub const KEYPRESS_NOTIFICATION: u64 = 0x0800000000000000;
		pub const REMOTE_HOST_SUPPORTED_FEATURES_NOTIFICATION: u64 = 0x1000000000000000;
		pub const LE_META: u64 = 0x2000000000000000;
	}

	pub mod le_event_mask {
		pub const CONNECTION_COMPLETE: u64 = 0x0000000000000001;
		pub const ADVERTISING_REPORT: u64 = 0x0000000000000002;
		pub const CONNECTION_UPDATE_COMPLETE: u64 = 0x0000000000000004;
		pub const READ_REMOTE_USED_FEATURES_COMPLETE: u64 = 0x0000000000000008;
		pub const LONG_TERM_KEY_REQUEST: u64 = 0x0000000000000010;
	}
}


macro_rules! event_mask_builder_method {
	( $method_name:ident => $const_name:ident ) => {
		pub fn $method_name (mut self) -> EventMaskBuilder {
			self.mask |= consts::event_mask::$const_name;
			self
		}
	}
}

pub struct EventMask(pub u64);

impl EventMask {
	pub fn builder() -> EventMaskBuilder {
		EventMaskBuilder::new()
	}
	pub fn default() -> EventMask {
		std::default::Default::default()
	}
	pub fn known() -> EventMask {
		let mut mask: u64 = 0x0000000000006000;
		mask |= consts::event_mask::INQUIRY_COMPLETE;
		mask |= consts::event_mask::INQUIRY_RESULT;
		mask |= consts::event_mask::CONNECTION_COMPLETE;
		mask |= consts::event_mask::CONNECTION_REQUEST;
		mask |= consts::event_mask::DISCONNECTION_COMPLETE;
		mask |= consts::event_mask::AUTHENTICATION_COMPLETE;
		mask |= consts::event_mask::REMOTE_NAME_REQUEST_COMPLETE;
		mask |= consts::event_mask::ENCRYPTION_CHANGE;
		mask |= consts::event_mask::CHANGE_CONNECTION_LINK_KEY_COMPLETE;
		mask |= consts::event_mask::MASTER_LINK_KEY_COMPLETE;
		mask |= consts::event_mask::READ_REMOTE_SUPPORTED_FEATURES_COMPLETE;
		mask |= consts::event_mask::READ_REMOTE_VERSION_INFORMATION_COMPLETE;
		mask |= consts::event_mask::QOS_SETUP_COMPLETE;
		mask |= consts::event_mask::HARDWARE_ERROR;
		mask |= consts::event_mask::FLUSH_OCCURRED;
		mask |= consts::event_mask::ROLE_CHANGE;
		mask |= consts::event_mask::MODE_CHANGE;
		mask |= consts::event_mask::RETURN_LINK_KEYS;
		mask |= consts::event_mask::PIN_CODE_REQUEST;
		mask |= consts::event_mask::LINK_KEY_REQUEST;
		mask |= consts::event_mask::LINK_KEY_NOTIFICATION;
		mask |= consts::event_mask::LOOPBACK_COMMAND;
		mask |= consts::event_mask::DATA_BUFFER_OVERFLOW;
		mask |= consts::event_mask::MAX_SLOTS_CHANGE;
		mask |= consts::event_mask::READ_CLOCK_OFFSET_COMPLETE;
		mask |= consts::event_mask::CONNECTION_PACKET_TYPE_CHANGED;
		mask |= consts::event_mask::QOS_VIOLATION;
		mask |= consts::event_mask::PAGE_SCAN_MODE_CHANGE;
		mask |= consts::event_mask::PAGE_SCAN_REPETITION_MODE_CHANGE;
		mask |= consts::event_mask::FLOW_SPECIFICATION_COMPLETE;
		mask |= consts::event_mask::INQUIRY_RESULT_WITH_RSSI;
		mask |= consts::event_mask::READ_REMOTE_EXTENDED_FEATURES_COMPLETE;
		mask |= consts::event_mask::SYNCHRONOUS_CONNECTION_COMPLETE;
		mask |= consts::event_mask::SYNCHRONOUS_CONNECTION_CHANGED;
		mask |= consts::event_mask::SNIFF_SUBRATING;
		mask |= consts::event_mask::EXTENDED_INQUIRY_RESULT;
		mask |= consts::event_mask::ENCRYPTION_KEY_REFRESH_COMPLETE;
		mask |= consts::event_mask::IO_CAPABILITY_REQUEST;
		mask |= consts::event_mask::IO_CAPABILITY_REQUEST_REPLY;
		mask |= consts::event_mask::USER_CONFIRMATION_REQUEST;
		mask |= consts::event_mask::USER_PASSKEY_REQUEST;
		mask |= consts::event_mask::REMOTE_OOB_DATA_REQUEST;
		mask |= consts::event_mask::SIMPLE_PAIRING_COMPLETE;
		mask |= consts::event_mask::LINK_SUPERVISION_TIMEOUT_CHANGED;
		mask |= consts::event_mask::ENHANCED_FLUSH_COMPLETE;
		mask |= consts::event_mask::USER_PASSKEY_NOTIFICATION;
		mask |= consts::event_mask::KEYPRESS_NOTIFICATION;
		mask |= consts::event_mask::REMOTE_HOST_SUPPORTED_FEATURES_NOTIFICATION;
		mask |= consts::event_mask::LE_META;
		EventMask(mask)
	}
}

impl std::default::Default for EventMask {
	fn default() -> EventMask {
		EventMask(0x00001FFFFFFFFFFF)
	}
}

impl From<u64> for EventMask {
	fn from(value: u64) -> EventMask {
		EventMask(value)
	}
}

impl Into<u64> for EventMask {
	fn into(self) -> u64 {
		self.0
	}
}

pub struct EventMaskBuilder {
	mask: u64,
}

impl EventMaskBuilder {
	pub fn new() -> EventMaskBuilder {
		EventMaskBuilder {
			mask: 0x0000000000006000,
		}
	}
	pub fn known(mut self) -> EventMaskBuilder {
		self.mask = EventMask::known().into();
		self
	}
	event_mask_builder_method!(inquiry_complete => INQUIRY_COMPLETE);
	event_mask_builder_method!(inquiry_result => INQUIRY_RESULT);
	event_mask_builder_method!(connection_complete => CONNECTION_COMPLETE);
	event_mask_builder_method!(connection_request => CONNECTION_REQUEST);
	event_mask_builder_method!(disconnection_complete => DISCONNECTION_COMPLETE);
	event_mask_builder_method!(authentication_complete => AUTHENTICATION_COMPLETE);
	event_mask_builder_method!(remote_name_request_complete => REMOTE_NAME_REQUEST_COMPLETE);
	event_mask_builder_method!(encryption_change => ENCRYPTION_CHANGE);
	event_mask_builder_method!(change_connection_link_key_complete => CHANGE_CONNECTION_LINK_KEY_COMPLETE);
	event_mask_builder_method!(master_link_key_complete => MASTER_LINK_KEY_COMPLETE);
	event_mask_builder_method!(read_remote_supported_features_complete => READ_REMOTE_SUPPORTED_FEATURES_COMPLETE);
	event_mask_builder_method!(read_remote_version_information_complete => READ_REMOTE_VERSION_INFORMATION_COMPLETE);
	event_mask_builder_method!(qos_setup_complete => QOS_SETUP_COMPLETE);
	event_mask_builder_method!(hardware_error => HARDWARE_ERROR);
	event_mask_builder_method!(flush_occurred => FLUSH_OCCURRED);
	event_mask_builder_method!(role_change => ROLE_CHANGE);
	event_mask_builder_method!(mode_change => MODE_CHANGE);
	event_mask_builder_method!(return_link_keys => RETURN_LINK_KEYS);
	event_mask_builder_method!(pin_code_request => PIN_CODE_REQUEST);
	event_mask_builder_method!(link_key_request => LINK_KEY_REQUEST);
	event_mask_builder_method!(link_key_notification => LINK_KEY_NOTIFICATION);
	event_mask_builder_method!(loopback_command => LOOPBACK_COMMAND);
	event_mask_builder_method!(data_buffer_overflow => DATA_BUFFER_OVERFLOW);
	event_mask_builder_method!(max_slots_change => MAX_SLOTS_CHANGE);
	event_mask_builder_method!(read_clock_offset_complete => READ_CLOCK_OFFSET_COMPLETE);
	event_mask_builder_method!(connection_packet_type_changed => CONNECTION_PACKET_TYPE_CHANGED);
	event_mask_builder_method!(qos_violation => QOS_VIOLATION);
	event_mask_builder_method!(page_scan_mode_change => PAGE_SCAN_MODE_CHANGE); // deprecated
	event_mask_builder_method!(page_scan_repetition_mode_change => PAGE_SCAN_REPETITION_MODE_CHANGE);
	event_mask_builder_method!(flow_specification_complete => FLOW_SPECIFICATION_COMPLETE);
	event_mask_builder_method!(inquiry_result_with_rssi => INQUIRY_RESULT_WITH_RSSI);
	event_mask_builder_method!(read_remote_extended_features_complete => READ_REMOTE_EXTENDED_FEATURES_COMPLETE);
	event_mask_builder_method!(synchronous_connection_complete => SYNCHRONOUS_CONNECTION_COMPLETE);
	event_mask_builder_method!(synchronous_connection_changed => SYNCHRONOUS_CONNECTION_CHANGED);
	event_mask_builder_method!(sniff_subrating => SNIFF_SUBRATING);
	event_mask_builder_method!(extended_inquiry_result => EXTENDED_INQUIRY_RESULT);
	event_mask_builder_method!(encryption_key_refresh_complete => ENCRYPTION_KEY_REFRESH_COMPLETE);
	event_mask_builder_method!(io_capability_request => IO_CAPABILITY_REQUEST);
	event_mask_builder_method!(io_capability_request_reply => IO_CAPABILITY_REQUEST_REPLY);
	event_mask_builder_method!(user_confirmation_request => USER_CONFIRMATION_REQUEST);
	event_mask_builder_method!(user_passkey_request => USER_PASSKEY_REQUEST);
	event_mask_builder_method!(remote_oob_data_request => REMOTE_OOB_DATA_REQUEST);
	event_mask_builder_method!(simple_pairing_complete => SIMPLE_PAIRING_COMPLETE);
	event_mask_builder_method!(link_supervision_timeout_changed => LINK_SUPERVISION_TIMEOUT_CHANGED);
	event_mask_builder_method!(enhanced_flush_complete => ENHANCED_FLUSH_COMPLETE);
	event_mask_builder_method!(user_passkey_notification => USER_PASSKEY_NOTIFICATION);
	event_mask_builder_method!(keypress_notification => KEYPRESS_NOTIFICATION);
	event_mask_builder_method!(remote_host_supported_features_notification => REMOTE_HOST_SUPPORTED_FEATURES_NOTIFICATION);
	event_mask_builder_method!(le_meta => LE_META);
	pub fn unknown(mut self, value: u64) -> EventMaskBuilder {
		self.mask |= value;
		self
	}
	pub fn build(self) -> EventMask {
		EventMask(self.mask)
	}
}


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


macro_rules! le_event_mask_builder_method {
	( $method_name:ident => $const_name:ident ) => {
		pub fn $method_name (mut self) -> LeEventMaskBuilder {
			self.mask |= consts::le_event_mask::$const_name;
			self
		}
	}
}

pub struct LeEventMask(pub u64);

impl LeEventMask {
	pub fn builder() -> LeEventMaskBuilder {
		LeEventMaskBuilder::new()
	}
	pub fn default() -> LeEventMask {
		std::default::Default::default()
	}
	pub fn known() -> LeEventMask {
		let mut mask: u64 = 0;
		mask |= consts::le_event_mask::CONNECTION_COMPLETE;
		mask |= consts::le_event_mask::ADVERTISING_REPORT;
		mask |= consts::le_event_mask::CONNECTION_UPDATE_COMPLETE;
		mask |= consts::le_event_mask::READ_REMOTE_USED_FEATURES_COMPLETE;
		mask |= consts::le_event_mask::LONG_TERM_KEY_REQUEST;
		LeEventMask(mask)
	}
}

impl std::default::Default for LeEventMask {
	fn default() -> LeEventMask {
		LeEventMask(0x000000000000001F)
	}
}

impl Into<u64> for LeEventMask {
	fn into(self) -> u64 {
		self.0
	}
}

pub struct LeEventMaskBuilder {
	mask: u64,
}

impl LeEventMaskBuilder {
	pub fn new() -> LeEventMaskBuilder {
		LeEventMaskBuilder {
			mask: 0,
		}
	}
	pub fn known(mut self) -> LeEventMaskBuilder {
		self.mask = LeEventMask::known().into();
		self
	}
	le_event_mask_builder_method!(connection_complete => CONNECTION_COMPLETE);
	le_event_mask_builder_method!(advertising_report => ADVERTISING_REPORT);
	le_event_mask_builder_method!(connection_update_complete => CONNECTION_UPDATE_COMPLETE);
	le_event_mask_builder_method!(read_remote_used_features_complete => READ_REMOTE_USED_FEATURES_COMPLETE);
	le_event_mask_builder_method!(long_term_key_request => LONG_TERM_KEY_REQUEST);
	pub fn unknown(mut self, value: u64) -> LeEventMaskBuilder {
		self.mask |= value;
		self
	}
	pub fn build(self) -> LeEventMask {
		LeEventMask(self.mask)
	}
}
