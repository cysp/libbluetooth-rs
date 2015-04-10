use hci::types::*;
use hci::opcode::LeControllerOpcode;


define_command!(LeControllerOpcode::SetEventMask, mask: u64 as LeEventMask);
