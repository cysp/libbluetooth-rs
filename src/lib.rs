#![feature(core)]
#![feature(libc)]
#![feature(collections)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate libc;
extern crate rustc_serialize as serialize;

extern crate byteorder;

mod types;
pub use types::*;

pub mod hci;

// pub use hci::HciHandle;
// use hci::opcode::Opcode;
// use hci::opcode::NopOpcode;
// use hci::opcode::LinkControlOpcode;
// use hci::opcode::LinkPolicyOpcode;
// use hci::opcode::ControllerOpcode;
// use hci::opcode::InformationalOpcode;
// use hci::opcode::StatusParametersOpcode;
// use hci::opcode::TestingOpcode;
// use hci::opcode::LeControllerOpcode;
// use hci::opcode::VendorOpcode;
// pub use hci::HciPacket;
// pub use hci::HciCommandPacket;
// pub use hci::HciEventPacket;

// pub use hci::HciCommand;
// use hci::command::{self,HciCommand,HciCommandBuilder,HciCommandBuilding};


pub static BDADDR_ANY: BdAddr = BdAddr([0, 0, 0, 0, 0, 0]);
pub static BDADDR_ALL: BdAddr = BdAddr([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
pub static BDADDR_LOCAL: BdAddr = BdAddr([0, 0, 0, 0xff, 0xff, 0xff]);


#[cfg(test)]
mod tests {
	use std;
	use super::*;
	use super::hci::*;
	// use super::hci::opcode::*;
	// use super::hci::types;
	// use super::hci::command::{self, HciCommand, HciCommandBuilder, HciCommandBuilding};

	#[test]
	fn smoke() {
		let _: BdAddr = [0, 0, 0, 0, 0, 0].into();
		let _: BdAddr = (&[0, 0, 0, 0, 0, 0]).into();
		let a: BdAddr = [0, 0, 0, 0, 0, 0].into();
		assert_eq!(*a, [0, 0, 0, 0, 0, 0]);

		// < 01 03 0C 00
		{
			let packet = [0x01, 0x03, 0x0C, 0x00];
			let b: Vec<u8> = command::controller::Reset.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 04 01 03 0C 00
		// < 01 03 10 00
		{
			let packet = [0x01, 0x03, 0x10, 0x00];
			let b: Vec<u8> = command::informational::ReadLocalSupportedFeatures.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 0C 01 03 10 00 FF FF 8F FE DB FF 5B 87
		// < 01 01 10 00
		{
			let packet = [0x01, 0x01, 0x10, 0x00];
			let b: Vec<u8> = command::informational::ReadLocalVersionInformation.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 0C 01 01 10 00 06 BB 22 06 0A 00 BB 22
		// < 01 09 10 00
		{
			let packet = [0x01, 0x09, 0x10, 0x00];
			let b: Vec<u8> = command::informational::ReadBdAddr.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 0A 01 09 10 00 13 71 DA 7D 1A 00
		// < 01 05 10 00
		{
			let packet = [0x01, 0x05, 0x10, 0x00];
			let b: Vec<u8> = command::informational::ReadBufferSize.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 0B 01 05 10 00 36 01 40 0A 00 08 00
		// < 01 23 0C 00
		// {
		// 	let packet = [0x01, 0x23, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadClassOfDevice);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 07 01 23 0C 00 00 00 00
		// < 01 14 0C 00
		// {
		// 	let packet = [0x01, 0x14, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadLocalName);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E FC 01 14 0C 00 43 53 52 38 35 31 30 20 41 31 30 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		// < 01 25 0C 00
		// {
		// 	let packet = [0x01, 0x25, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadVoiceSetting);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 06 01 25 0C 00 60 00
		// < 01 38 0C 00
		// {
		// 	let packet = [0x01, 0x38, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadNumberOfSupportedIac);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 05 01 38 0C 00 02
		// < 01 39 0C 00
		// {
		// 	let packet = [0x01, 0x39, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadCurrentIacLap);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 08 01 39 0C 00 01 33 8B 9E
		// < 01 05 0C 01 00
		// {
		// 	let packet = [0x01, 0x05, 0x0C, 0x01, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::SetEventFilter);
		// 	let b = HciCommandBuilder::new(oc).parameter(0u8).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 05 0C 00
		// < 01 16 0C 02 00 7D
		// {
		// 	let packet = [0x01, 0x16, 0x0C, 0x02, 0x00, 0x7D];
		// 	let oc = Opcode::Controller(ControllerOpcode::WriteConnectionAcceptTimeout);
		// 	let b = HciCommandBuilder::new(oc).parameter(32000u16).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 16 0C 00
		// < 01 1B 0C 00
		// {
		// 	let packet = [0x01, 0x1B, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadPageScanActivity);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 08 01 1B 0C 00 00 08 12 00
		// < 01 46 0C 00
		// {
		// 	let packet = [0x01, 0x46, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadPageScanType);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 05 01 46 0C 00 00
		// < 01 02 20 00
		// {
		// 	let packet = [0x01, 0x02, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ReadBufferSize);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 07 01 02 20 00 00 00 00
		// < 01 03 20 00
		// {
		// 	let packet = [0x01, 0x03, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ReadLocalSupportedFeatures);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 0C 01 03 20 00 01 00 00 00 00 00 00 00
		// < 01 1C 20 00
		// {
		// 	let packet = [0x01, 0x1C, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ReadSupportedStates);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 0C 01 1C 20 00 FF FF FF 1F 00 00 00 00
		// < 01 07 20 00
		// {
		// 	let packet = [0x01, 0x07, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ReadAdvertisingChannelTxPower);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 05 01 07 20 00 08
		// < 01 0F 20 00
		// {
		// 	let packet = [0x01, 0x0F, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ReadWhitelistSize);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 05 01 0F 20 00 19
		// < 01 10 20 00
		// {
		// 	let packet = [0x01, 0x10, 0x20, 0x00];
		// 	let oc = Opcode::LeController(LeControllerOpcode::ClearWhitelist);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 10 20 00
		// < 01 01 0C 08 FF FF FB FF 07 F8 BF 3D
		// {
		// 	let packet = [0x01, 0x01, 0x0C, 0x08, 0xFF, 0xFF, 0xFB, 0xFF, 0x07, 0xF8, 0xBF, 0x3D];
		// 	let oc = Opcode::Controller(ControllerOpcode::SetEventMask);
		// 	let b = HciCommandBuilder::new(oc).parameter([0xFF, 0xFF, 0xFB, 0xFF, 0x07, 0xF8, 0xBF, 0x3D].as_ref()).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 01 0C 00
		// < 01 01 20 08 1F 00 00 00 00 00 00 00
		{
			let packet = [0x01, 0x01, 0x20, 0x08, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
			let b: Vec<u8> = command::le_controller::SetEventMask{
				mask: LeEventMask(0x00000000000000001F),
			}.into();
			assert_eq!(b, &packet[1..]);
		}
		{
			let packet = [0x01, 0x01, 0x20, 0x08, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
			let b: Vec<u8> = command::le_controller::SetEventMask{
				mask: LeEventMask::default(),
			}.into();
			assert_eq!(b, &packet[1..]);
		}
		{
			let packet = [0x01, 0x01, 0x20, 0x08, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
			let b: Vec<u8> = command::le_controller::SetEventMask{
				mask: std::default::Default::default(),
			}.into();
			assert_eq!(b, &packet[1..]);
		}
		{
			let packet = [0x01, 0x01, 0x20, 0x08, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
			let b: Vec<u8> = command::le_controller::SetEventMask{
				mask: LeEventMask::builder()
					.connection_complete()
					.advertising_report()
					.connection_update_complete()
					.read_remote_used_features_complete()
					.long_term_key_request()
					.build(),
			}.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 04 01 01 20 00
		// < 01 02 10 00
		{
			let packet = [0x01, 0x02, 0x10, 0x00];
			let b: Vec<u8> = command::informational::ReadLocalSupportedCommands.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 44 01 02 10 00 FF FF FF 03 FE FF FF FF FF FF FF FF F3
		//   0F E8 FE 3F F7 83 FF 1C 00 00 00 61 F7 FF FF 7F 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00
		// < 01 52 0C F1 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
		//   00 00 00 00 00
		// {
		// 	let packet = [
		// 		0x01,
		// 		0x52, 0x0C,
		// 		0xF1,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		// 		0x00
		// 	];
		// 	let oc = Opcode::Controller(ControllerOpcode::WriteExtendedInquiryResponse);
		// 	let b = HciCommandBuilder::new(oc)
		// 		.parameter(0u8)
		// 		.parameter([0u8; 240].as_ref())
		// 		.build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 52 0C 00
		// < 01 45 0C 01 02
		{
			let packet = [0x01, 0x45, 0x0C, 0x01, 0x02];
			let c = command::controller::WriteInquiryMode {
				mode: InquiryMode::RSSIOrExtended,
			};
			let b: Vec<u8> = c.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 04 01 45 0C 00
		// < 01 58 0C 00
		// {
		// 	let packet = [0x01, 0x58, 0x0C, 0x00];
		// 	let oc = Opcode::Controller(ControllerOpcode::ReadInquiryResponseTransmitPowerLevel);
		// 	let b = HciCommandBuilder::new(oc).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 05 01 58 0C 00 04
		// < 01 04 10 01 01
		{
			let packet = [0x01, 0x04, 0x10, 0x01, 0x01];
			let b: Vec<u8> = command::informational::ReadLocalExtendedFeatures{ page: 1 }.into();
			assert_eq!(b, &packet[1..]);
		}

		// > 04 0E 0E 01 04 10 00 01 00 00 00 00 00 00 00 00 00
		// < 01 12 0C 07 00 00 00 00 00 00 01
		// {
		// 	let packet = [0x01, 0x12, 0x0C, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
		// 	let oc = Opcode::Controller(ControllerOpcode::DeleteStoredLinkKey);
		// 	let b = HciCommandBuilder::new(oc)
		// 		.parameter(BDADDR_ANY.as_ref().as_ref())
		// 		.parameter(1u8)
		// 		.build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 06 01 12 0C 00 00 00
		// < 01 0F 08 02 0F 00
		// {
		// 	let packet = [0x01, 0x0F, 0x08, 0x02, 0x0F, 0x00];
		// 	let oc = Opcode::LinkPolicy(LinkPolicyOpcode::WriteDefaultLinkPolicySettings);
		// 	let b = HciCommandBuilder::new(oc).parameter(0x0Fu16).build();
		// 	assert_eq!(b.to_bytes(), &packet[1..]);
		// }

		// > 04 0E 04 01 0F 08 00
	}
}
