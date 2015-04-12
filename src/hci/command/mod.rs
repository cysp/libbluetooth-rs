use std;
use std::io::Write;

use byteorder::{WriteBytesExt,LittleEndian};

use super::opcode;


macro_rules! define_command {
	( $command_group:ident :: $command_name:ident ) => {
		pub struct $command_name;

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				use $crate::hci::command::CommandBuilder;

				let b = CommandBuilder::new($command_group::$command_name)
					.build();
				b.to_bytes()
			}
		}
	};
	( $command_group:ident :: $command_name:ident, $( $param_name:ident: $param_ty:ty ),+ ) => {
		pub struct $command_name {
			$( pub $param_name: $param_ty, )+
		}

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				use $crate::hci::command::{CommandBuilder, CommandBuilding};

				let b = CommandBuilder::new($command_group::$command_name)
				$(  .parameter(self.$param_name) )*
					.build();
				b.to_bytes()
			}
		}
	};
	( $command_group:ident :: $command_name:ident, $( $param_name:ident: $param_ty:ty as $param_as_ty:ty ),+ ) => {
		pub struct $command_name {
			$( pub $param_name: $param_as_ty, )+
		}

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				use $crate::hci::command::{CommandBuilder, CommandBuilding};

				$(
				let $param_name: $param_ty = self.$param_name.into();
				)*

				let b = CommandBuilder::new($command_group::$command_name)
				$(  .parameter($param_name) )*
					.build();
				b.to_bytes()
			}
		}
	};
}

pub mod controller;
pub mod informational;
pub mod le_controller;


pub struct Command {
	opcode: opcode::Opcode,
	parameter_data: Vec<u8>,
}

impl Command {
	pub fn write_to(&self, w: &mut std::io::Write) -> std::io::Result<usize> {
		let parameter_data_length = self.parameter_data.len();
		if parameter_data_length > 255 {
			return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "parameter_data too large"));
		}
		let buf: Vec<u8> = self.to_bytes();
		w.write(&*buf)
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let opcode: u16 = self.opcode.into();
		let parameter_data_length = self.parameter_data.len();
		let buf: Vec<u8> = Vec::with_capacity(2 + 1 + parameter_data_length);
		let mut c = std::io::Cursor::new(buf);
		c.write_u16::<LittleEndian>(opcode).unwrap();
		c.write_u8(parameter_data_length as u8).unwrap();
		c.write(&*self.parameter_data).unwrap();
		c.into_inner()
	}
}

impl Into<Vec<u8>> for Command {
	fn into(self) -> Vec<u8> {
		let opcode: u16 = self.opcode.into();
		let parameter_data_length = self.parameter_data.len();
		let buf: Vec<u8> = Vec::with_capacity(2 + 1 + parameter_data_length);
		let mut c = std::io::Cursor::new(buf);
		c.write_u16::<LittleEndian>(opcode).unwrap();
		c.write_u8(parameter_data_length as u8).unwrap();
		c.write(&*self.parameter_data).unwrap();
		c.into_inner()
	}
}


trait CommandBuilding<T> {
	fn parameter(mut self, v: T) -> Self;
}

struct CommandBuilder {
	opcode: opcode::Opcode,
	parameter_data: Vec<u8>,
}

impl CommandBuilder {
	pub fn new<T: Into<opcode::Opcode> >(opcode: T) -> CommandBuilder {
		CommandBuilder {
			opcode: opcode.into(),
			parameter_data: Vec::new(),
		}
	}

	pub fn build(self) -> Command {
		Command {
			opcode: self.opcode,
			parameter_data: self.parameter_data,
		}
	}
}

impl CommandBuilding<u8> for CommandBuilder {
	fn parameter(mut self, v: u8) -> CommandBuilder {
		self.parameter_data.write_u8(v).unwrap();
		self
	}
}

impl CommandBuilding<u16> for CommandBuilder {
	fn parameter(mut self, v: u16) -> CommandBuilder {
		self.parameter_data.write_u16::<LittleEndian>(v).unwrap();
		self
	}
}

impl CommandBuilding<u32> for CommandBuilder {
	fn parameter(mut self, v: u32) -> CommandBuilder {
		self.parameter_data.write_u32::<LittleEndian>(v).unwrap();
		self
	}
}

impl CommandBuilding<u64> for CommandBuilder {
	fn parameter(mut self, v: u64) -> CommandBuilder {
		self.parameter_data.write_u64::<LittleEndian>(v).unwrap();
		self
	}
}

impl CommandBuilding<[u8; 8]> for CommandBuilder {
	fn parameter(mut self, v: [u8; 8]) -> CommandBuilder {
		self.parameter_data.extend(v.iter().cloned());
		self
	}
}
