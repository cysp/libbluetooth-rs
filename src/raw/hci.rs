extern crate libc;
extern crate std;

use std::ffi;
use std::str;

pub static SOL_HCI: libc::c_int = 0;
pub static SOL_L2CAP: libc::c_int = 6;
pub static SOL_SCO: libc::c_int = 17;
pub static SOL_RFCOMM: libc::c_int = 18;
pub static SOL_BLUETOOTH: libc::c_int = 274;

// HCI Socket options
pub static HCI_DATA_DIR: libc::c_int = 1;
pub static HCI_FILTER: libc::c_int = 2;
pub static HCI_TIME_STAMP: libc::c_int = 3;


#[repr(C,packed)]
#[derive(Debug)]
pub struct hci_filter {
	pub type_mask: libc::uint32_t,
	pub event_mask: libc::uint64_t,
	pub opcode: libc::uint16_t,
}


#[repr(C)]
pub enum CommandStatusCode {
	Success = 0x00,
UnknownHCICommand = 0x01,
UnknownConnectionIdentifier = 0x02,
0x03
Hardware Failure
0x04
Page Timeout
0x05
Authentication Failure
0x06
PIN or Key Missing
0x07
Memory Capacity Exceeded
0x08
Connection Timeout
0x09
Connection Limit Exceeded
0x0A
Synchronous Connection Limit To A Device Exceeded
0x0B
ACL Connection Already Exists
0x0C
Command Disallowed
0x0D
Connection Rejected due to Limited Resources
0x0E
Connection Rejected Due To Security Reasons
0x0F
Connection Rejected due to Unacceptable BD_ADDR
0x10
Connection Accept Timeout Exceeded
0x11
Unsupported Feature or Parameter Value
0x12
Invalid HCI Command Parameters
0x13
Remote User Terminated Connection
0x14
Remote Device Terminated Connection due to Low Resources
0x15
Remote Device Terminated Connection due to Power Off
0x16
Connection Terminated By Local Host
0x17
Repeated Attempts
0x18
Pairing Not Allowed
0x19
Unknown LMP PDU
0x1A
Unsupported Remote Feature / Unsupported LMP Feature
0x1B
SCO Offset Rejected
0x1C
SCO Interval Rejected
0x1D
SCO Air Mode Rejected
0x1E
Invalid LMP Parameters
0x1F
Unspecified Error
0x20
Unsupported LMP Parameter Value
0x21
Role Change Not Allowed
0x22
LMP Response Timeout / LL Response Timeout
0x23
LMP Error Transaction Collision
0x24
LMP PDU Not Allowed
0x25
Encryption Mode Not Acceptable
0x26
Link Key cannot be Changed
0x27
Requested QoS Not Supported
0x28
Instant Passed
0x29
Pairing With Unit Key Not Supported
0x2A
Different Transaction Collision
0x2B
Reserved
0x2C
QoS Unacceptable Parameter
0x2D
QoS Rejected
0x2E
Channel Classification Not Supported
0x2F
Insufficient Security
0x30
Parameter Out Of Mandatory Range
0x31
Reserved
0x32
Role Switch Pending
0x33
Reserved
0x34
Reserved Slot Violation
0x35
Role Switch Failed
0x36
Extended Inquiry Response Too Large
0x37
Secure Simple Pairing Not Supported By Host.
0x38
Host Busy - Pairing
0x39
Connection Rejected due to No Suitable Channel Found
0x3A
Controller Busy
0x3B
Unacceptable Connection Interval
0x3C
Directed Advertising Timeout
0x3D
Connection Terminated due to MIC Failure
0x3E
Connection Failed to be Established
0x3F
MAC Connection Failed
}


// #[link(name = "bluetooth")]
// extern {
// 	pub fn hci_get_route(bdaddr: * const bdaddr_t) -> libc::c_int;

// 	pub fn hci_open_dev(dev_id: libc::c_int) -> libc::c_int;
// 	pub fn hci_close_dev(dd: libc::c_int) -> libc::c_int;

// 	pub fn hci_vertostr(ver: libc::c_uint) -> *const libc::c_char;
// 	pub fn lmp_vertostr(ver: libc::c_uint) -> *const libc::c_char;
// 	pub fn hci_cmdtostr(ver: libc::c_uint) -> *const libc::c_char;
// 	pub fn bt_compidtostr(compid: libc::c_int) -> *const libc::c_char;

// 	pub fn hci_read_local_name(dd: libc::c_int, len: libc::c_int, name: *mut libc::c_char, to: libc::c_int) -> libc::c_int;
// 	pub fn hci_write_local_name(dd: libc::c_int, name: *const libc::c_char, to: libc::c_int) -> libc::c_int;
// 	pub fn hci_read_local_version(dd: libc::c_int, ver: *mut hci_version, to: libc::c_int) -> libc::c_int;
// 	pub fn hci_read_local_commands(dd: libc::c_int, commands: *mut hci_commands, to: libc::c_int) -> libc::c_int;

// 	pub fn hci_read_remote_name(dd: libc::c_int, bdaddr: *const bdaddr_t, len: libc::c_int, name: *mut libc::c_char, to: libc::c_int) -> libc::c_int;
// 	// int hci_read_remote_name_with_clock_offset(int dd, const bdaddr_t *bdaddr, uint8_t pscan_rep_mode, uint16_t clkoffset, int len, char *name, int to);
// 	// int hci_read_remote_name_cancel(int dd, const bdaddr_t *bdaddr, int to);
// 	// pub fn hci_read_remote_version(dd: libc::c_int, handle: libc::uint16_t, ver: *mut hci_version, to: libc::c_int) -> libc::c_int;
// 	// int hci_read_clock_offset(int dd, uint16_t handle, uint16_t *clkoffset, int to);

// 	pub fn hci_le_set_scan_enable(dd: libc::c_int, enable: libc::uint8_t, filter_dup: libc::uint8_t, to: libc::c_int) -> libc::c_int;
// 	pub fn hci_le_set_scan_parameters(dd: libc::c_int, scan_type: libc::uint8_t, interval: libc::uint16_t, window: libc::uint16_t, own_type: libc::uint8_t, filter: libc::uint8_t, to: libc::c_int) -> libc::c_int;
// 	pub fn hci_le_set_advertise_enable(dd: libc::c_int, enable: libc::uint8_t, to: libc::c_int) -> libc::c_int;
// }


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn smoke() {
		let r = unsafe { hci_get_route(&BDADDR_ANY) };
		let _ = unsafe { hci_get_route(&BDADDR_ALL) };
		let _ = unsafe { hci_get_route(&BDADDR_LOCAL) };

		if r > 0 {
			let d = unsafe { hci_open_dev(r) };
			if d > 0 {
				unsafe { hci_close_dev(d) };
			}
		}
	}
}
