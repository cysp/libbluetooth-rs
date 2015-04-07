use std;
use std::io::Write;

use byteorder::{WriteBytesExt,LittleEndian};

use super::opcode;


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
	pub fn new(opcode: opcode::Opcode) -> HciCommandBuilder {
		HciCommandBuilder {
			opcode: opcode,
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
