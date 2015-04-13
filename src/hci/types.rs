use std;

mod consts {
	pub use hci::consts::*;
}

macro_rules! event_mask_builder_method {
	( $method_name:ident => $const_name:ident ) => {
		pub fn $method_name (mut self) -> EventMaskBuilder {
			self.mask |= consts::event::masks::$const_name;
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
		let mut mask: u64 = 0;
		mask |= consts::event::masks::COMMAND_COMPLETE;
		mask |= consts::event::masks::COMMAND_STATUS;
		mask |= consts::event::masks::INQUIRY_COMPLETE;
		mask |= consts::event::masks::INQUIRY_RESULT;
		mask |= consts::event::masks::CONNECTION_COMPLETE;
		mask |= consts::event::masks::CONNECTION_REQUEST;
		mask |= consts::event::masks::DISCONNECTION_COMPLETE;
		mask |= consts::event::masks::AUTHENTICATION_COMPLETE;
		mask |= consts::event::masks::REMOTE_NAME_REQUEST_COMPLETE;
		mask |= consts::event::masks::ENCRYPTION_CHANGE;
		mask |= consts::event::masks::CHANGE_CONNECTION_LINK_KEY_COMPLETE;
		mask |= consts::event::masks::MASTER_LINK_KEY_COMPLETE;
		mask |= consts::event::masks::READ_REMOTE_SUPPORTED_FEATURES_COMPLETE;
		mask |= consts::event::masks::READ_REMOTE_VERSION_INFORMATION_COMPLETE;
		mask |= consts::event::masks::QOS_SETUP_COMPLETE;
		mask |= consts::event::masks::HARDWARE_ERROR;
		mask |= consts::event::masks::FLUSH_OCCURRED;
		mask |= consts::event::masks::ROLE_CHANGE;
		mask |= consts::event::masks::MODE_CHANGE;
		mask |= consts::event::masks::RETURN_LINK_KEYS;
		mask |= consts::event::masks::PIN_CODE_REQUEST;
		mask |= consts::event::masks::LINK_KEY_REQUEST;
		mask |= consts::event::masks::LINK_KEY_NOTIFICATION;
		mask |= consts::event::masks::LOOPBACK_COMMAND;
		mask |= consts::event::masks::DATA_BUFFER_OVERFLOW;
		mask |= consts::event::masks::MAX_SLOTS_CHANGE;
		mask |= consts::event::masks::READ_CLOCK_OFFSET_COMPLETE;
		mask |= consts::event::masks::CONNECTION_PACKET_TYPE_CHANGED;
		mask |= consts::event::masks::QOS_VIOLATION;
		mask |= consts::event::masks::PAGE_SCAN_MODE_CHANGE;
		mask |= consts::event::masks::PAGE_SCAN_REPETITION_MODE_CHANGE;
		mask |= consts::event::masks::FLOW_SPECIFICATION_COMPLETE;
		mask |= consts::event::masks::INQUIRY_RESULT_WITH_RSSI;
		mask |= consts::event::masks::READ_REMOTE_EXTENDED_FEATURES_COMPLETE;
		mask |= consts::event::masks::SYNCHRONOUS_CONNECTION_COMPLETE;
		mask |= consts::event::masks::SYNCHRONOUS_CONNECTION_CHANGED;
		mask |= consts::event::masks::SNIFF_SUBRATING;
		mask |= consts::event::masks::EXTENDED_INQUIRY_RESULT;
		mask |= consts::event::masks::ENCRYPTION_KEY_REFRESH_COMPLETE;
		mask |= consts::event::masks::IO_CAPABILITY_REQUEST;
		mask |= consts::event::masks::IO_CAPABILITY_REQUEST_REPLY;
		mask |= consts::event::masks::USER_CONFIRMATION_REQUEST;
		mask |= consts::event::masks::USER_PASSKEY_REQUEST;
		mask |= consts::event::masks::REMOTE_OOB_DATA_REQUEST;
		mask |= consts::event::masks::SIMPLE_PAIRING_COMPLETE;
		mask |= consts::event::masks::LINK_SUPERVISION_TIMEOUT_CHANGED;
		mask |= consts::event::masks::ENHANCED_FLUSH_COMPLETE;
		mask |= consts::event::masks::USER_PASSKEY_NOTIFICATION;
		mask |= consts::event::masks::KEYPRESS_NOTIFICATION;
		mask |= consts::event::masks::REMOTE_HOST_SUPPORTED_FEATURES_NOTIFICATION;
		mask |= consts::event::masks::LE_META;
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
		let mut mask: u64 = 0;
		mask |= consts::event::masks::COMMAND_COMPLETE;
		mask |= consts::event::masks::COMMAND_STATUS;
		EventMaskBuilder {
			mask: mask,
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
			self.mask |= consts::event::masks::le_subevent::$const_name;
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
		mask |= consts::event::masks::le_subevent::CONNECTION_COMPLETE;
		mask |= consts::event::masks::le_subevent::ADVERTISING_REPORT;
		mask |= consts::event::masks::le_subevent::CONNECTION_UPDATE_COMPLETE;
		mask |= consts::event::masks::le_subevent::READ_REMOTE_USED_FEATURES_COMPLETE;
		mask |= consts::event::masks::le_subevent::LONG_TERM_KEY_REQUEST;
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
