use std;
use std::io::Write;

use byteorder::{WriteBytesExt,LittleEndian};

use super::opcode;


macro_rules! define_command {
	( $command_group:ident :: $command_name:ident ) => {
		pub struct $command_name;

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				let b = HciCommandBuilder::new($command_group::$command_name)
					.build();
				b.to_bytes()
			}
		}
	};
	( $command_group:ident :: $command_name:ident, $( $param_name:ident: $param_ty:ty ),+ ) => {
		pub struct $command_name {
			$( pub $param_name: $param_ty ),+
		}

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				let b = HciCommandBuilder::new($command_group::$command_name)
				$(  .parameter(self.$param_name) )*
					.build();
				b.to_bytes()
			}
		}
	};
	( $command_group:ident :: $command_name:ident, $( $param_name:ident: $param_ty:ty as $param_as_ty:ty ),+ ) => {
		pub struct $command_name {
			$( pub $param_name: $param_ty ),+
		}

		impl Into<Vec<u8>> for $command_name {
			fn into(self) -> Vec<u8> {
				$(
				let $param_name: $param_as_ty = self.$param_name.into();
				)*
				let b = HciCommandBuilder::new($command_group::$command_name)
				$(  .parameter($param_name) )*
					.build();
				b.to_bytes()
			}
		}
	};
}

pub mod controller;
pub mod informational;


pub struct HciCommand {
	opcode: opcode::Opcode,
	parameter_data: Vec<u8>,
}

impl HciCommand {
	pub fn write_to(&self, w: &mut std::io::Write) -> std::io::Result<usize> {
		let opcode: u16 = self.opcode.into();
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


pub trait HciCommandBuilding<T> {
	fn parameter(mut self, v: T) -> Self;
}

pub struct HciCommandBuilder {
	opcode: opcode::Opcode,
	parameter_data: Vec<u8>,
}

impl HciCommandBuilder {
	pub fn new<T: Into<opcode::Opcode> >(opcode: T) -> HciCommandBuilder {
		HciCommandBuilder {
			opcode: opcode.into(),
			parameter_data: Vec::new(),
		}
	}

	pub fn build(self) -> HciCommand {
		HciCommand {
			opcode: self.opcode,
			parameter_data: self.parameter_data,
		}
	}
}

impl HciCommandBuilding<u8> for HciCommandBuilder {
	fn parameter(mut self, v: u8) -> HciCommandBuilder {
		self.parameter_data.write_u8(v).unwrap();
		self
	}
}

impl HciCommandBuilding<u16> for HciCommandBuilder {
	fn parameter(mut self, v: u16) -> HciCommandBuilder {
		self.parameter_data.write_u16::<LittleEndian>(v).unwrap();
		self
	}
}

impl<'a> HciCommandBuilding<&'a [u8]> for HciCommandBuilder {
	fn parameter(mut self, v: &'a [u8]) -> HciCommandBuilder {
		self.parameter_data.push_all(v);
		self
	}
}
