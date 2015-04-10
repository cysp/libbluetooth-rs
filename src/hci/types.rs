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
// 0x0000000000000080
// Encryption Change Event
// 0x0000000000000100
// Change Connection Link Key Complete Event
// 0x0000000000000200
// Master Link Key Complete Event
// 0x0000000000000400
// Read Remote Supported Features Complete Event
// 0x0000000000000800
// Read Remote Version Information Complete Event
// 0x0000000000001000
// QoS Setup Complete Event
// 0x0000000000008000
// Hardware Error Event
// 0x0000000000010000
// Flush Occurred Event
// 0x0000000000020000
// Role Change Event
// 0x0000000000080000
// Mode Change Event
// 0x0000000000100000
// Return Link Keys Event
// 0x0000000000200000
// PIN Code Request Event
// 0x0000000000400000
// Link Key Request Event
// 0x0000000000800000
// Link Key Notification Event
// 0x0000000001000000
// Loopback Command Event
// 0x0000000002000000
// Data Buffer Overflow Event
// 0x0000000004000000
// Max Slots Change Event
// 0x0000000008000000
// Read Clock Offset Complete Event
// 0x0000000010000000
// Connection Packet Type Changed Event
// 0x0000000020000000
// QoS Violation Event
// 0x0000000040000000
// Page Scan Mode Change Event [deprecated]
// 0x0000000080000000
// Page Scan Repetition Mode Change Event
// 0x0000000100000000
// Flow Specification Complete Event
// 0x0000000200000000
// Inquiry Result with RSSI Event
// 0x0000000400000000
// Read Remote Extended Features Complete Event
// 0x0000080000000000
// Synchronous Connection Complete Event
// 0x0000100000000000
// Synchronous Connection Changed Event
// 0x0000200000000000
// Sniff Subrating Event
// 0x0000400000000000
// Extended Inquiry Result Event
// 0x0000800000000000
// Encryption Key Refresh Complete Event
// 0x0001000000000000
// IO Capability Request Event
// 0x0002000000000000
// IO Capability Request Reply Event
// 0x0004000000000000
// User Confirmation Request Event
// 0x0008000000000000
// User Passkey Request Event
// 0x0010000000000000
// Remote OOB Data Request Event
// 0x0020000000000000
// Simple Pairing Complete Event
// 0x0080000000000000
// Link Supervision Timeout Changed Event
// 0x0100000000000000
// Enhanced Flush Complete Event
// 0x0400000000000000
// User Passkey Notification Event
// 0x0800000000000000
// Keypress Notification Event
// 0x1000000000000000
// Remote Host Supported Features Notification Event
// 0x2000000000000000
// LE Meta-Event
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
