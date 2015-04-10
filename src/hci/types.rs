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
		pub const ENCRYPTION_CHANGE: u64 = 0X0000000000000080;
		pub const CHANGE_CONNECTION_LINK_KEY_COMPLETE: u64 = 0X0000000000000100;
		pub const MASTER_LINK_KEY_COMPLETE: u64 = 0X0000000000000200;
		pub const READ_REMOTE_SUPPORTED_FEATURES_COMPLETE: u64 = 0X0000000000000400;
		pub const READ_REMOTE_VERSION_INFORMATION_COMPLETE: u64 = 0X0000000000000800;
		pub const QOS_SETUP_COMPLETE: u64 = 0X0000000000001000;
		pub const HARDWARE_ERROR: u64 = 0X0000000000008000;
		pub const FLUSH_OCCURRED: u64 = 0X0000000000010000;
		pub const ROLE_CHANGE: u64 = 0X0000000000020000;
		pub const MODE_CHANGE: u64 = 0X0000000000080000;
		pub const RETURN_LINK_KEYS: u64 = 0X0000000000100000;
		pub const PIN_CODE_REQUEST: u64 = 0X0000000000200000;
		pub const LINK_KEY_REQUEST: u64 = 0X0000000000400000;
		pub const LINK_KEY_NOTIFICATION: u64 = 0X0000000000800000;
		pub const LOOPBACK_COMMAND: u64 = 0X0000000001000000;
		pub const DATA_BUFFER_OVERFLOW: u64 = 0X0000000002000000;
		pub const MAX_SLOTS_CHANGE: u64 = 0X0000000004000000;
		pub const READ_CLOCK_OFFSET_COMPLETE: u64 = 0X0000000008000000;
		pub const CONNECTION_PACKET_TYPE_CHANGED: u64 = 0X0000000010000000;
		pub const QOS_VIOLATION: u64 = 0X0000000020000000;
		pub const PAGE_SCAN_MODE_CHANGE_EVENT_[DEPRECATED]: u64 = 0X0000000040000000;
		pub const PAGE_SCAN_REPETITION_MODE_CHANGE: u64 = 0X0000000080000000;
		pub const FLOW_SPECIFICATION_COMPLETE: u64 = 0X0000000100000000;
		pub const INQUIRY_RESULT_WITH_RSSI: u64 = 0X0000000200000000;
		pub const READ_REMOTE_EXTENDED_FEATURES_COMPLETE: u64 = 0X0000000400000000;
		pub const SYNCHRONOUS_CONNECTION_COMPLETE: u64 = 0X0000080000000000;
		pub const SYNCHRONOUS_CONNECTION_CHANGED: u64 = 0X0000100000000000;
		pub const SNIFF_SUBRATING: u64 = 0X0000200000000000;
		pub const EXTENDED_INQUIRY_RESULT: u64 = 0X0000400000000000;
		pub const ENCRYPTION_KEY_REFRESH_COMPLETE: u64 = 0X0000800000000000;
		pub const IO_CAPABILITY_REQUEST: u64 = 0X0001000000000000;
		pub const IO_CAPABILITY_REQUEST_REPLY: u64 = 0X0002000000000000;
		pub const USER_CONFIRMATION_REQUEST: u64 = 0X0004000000000000;
		pub const USER_PASSKEY_REQUEST: u64 = 0X0008000000000000;
		pub const REMOTE_OOB_DATA_REQUEST: u64 = 0X0010000000000000;
		pub const SIMPLE_PAIRING_COMPLETE: u64 = 0X0020000000000000;
		pub const LINK_SUPERVISION_TIMEOUT_CHANGED: u64 = 0X0080000000000000;
		pub const ENHANCED_FLUSH_COMPLETE: u64 = 0X0100000000000000;
		pub const USER_PASSKEY_NOTIFICATION: u64 = 0X0400000000000000;
		pub const KEYPRESS_NOTIFICATION: u64 = 0X0800000000000000;
		pub const REMOTE_HOST_SUPPORTED_FEATURES_NOTIFICATION: u64 = 0X1000000000000000;
		pub const LE_META: u64 = 0X2000000000000000;
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
			mask: 0,
		}
	}
	// pub fn connection_complete(mut self) -> LeEventMaskBuilder {
	// 	self.mask[0] |= 1 << consts::le_event_mask::CONNECTION_COMPLETE_SHIFT;
	// 	self
	// }
	// pub fn advertising_report(mut self) -> LeEventMaskBuilder {
	// 	self.mask[0] |= 1 << consts::le_event_mask::ADVERTISING_REPORT_SHIFT;
	// 	self
	// }
	// pub fn connection_update_complete(mut self) -> LeEventMaskBuilder {
	// 	self.mask[0] |= 1 << consts::le_event_mask::CONNECTION_UPDATE_COMPLETE_SHIFT;
	// 	self
	// }
	// pub fn read_remote_used_features_complete(mut self) -> LeEventMaskBuilder {
	// 	self.mask[0] |= 1 << consts::le_event_mask::READ_REMOTE_USED_FEATURES_COMPLETE_SHIFT;
	// 	self
	// }
	// pub fn long_term_key_request(mut self) -> LeEventMaskBuilder {
	// 	self.mask[0] |= 1 << consts::le_event_mask::LONG_TERM_KEY_REQUEST_SHIFT;
	// 	self
	// }
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
	le_event_mask_builder_method!(connection_complete => CONNECTION_COMPLETE);
	le_event_mask_builder_method!(advertising_report => ADVERTISING_REPORT);
	le_event_mask_builder_method!(connection_update_complete => CONNECTION_UPDATE_COMPLETE);
	le_event_mask_builder_method!(read_remote_used_features_complete => READ_REMOTE_USED_FEATURES_COMPLETE);
	le_event_mask_builder_method!(long_term_key_request => LONG_TERM_KEY_REQUEST);
	pub fn build(self) -> LeEventMask {
		LeEventMask(self.mask)
	}
}
