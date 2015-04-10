use hci::types::*;
use hci::opcode::ControllerOpcode;


define_command!(ControllerOpcode::SetEventMask, mask: u64 as EventMask);
define_command!(ControllerOpcode::Reset);
define_command!(ControllerOpcode::WriteInquiryMode, mode: u8 as InquiryMode);
