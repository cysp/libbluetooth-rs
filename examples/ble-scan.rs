#![feature(libc)]
#![feature(std_misc)]

// extern crate libc;
extern crate serialize;

extern crate bytes;
extern crate byteorder;
extern crate mio;
extern crate nix;

extern crate bluetooth;

use serialize::hex::ToHex;

use std::os::unix::AsRawFd;

use bytes::{Buf,MutByteBuf};
use byteorder::{ReadBytesExt,WriteBytesExt,BigEndian,LittleEndian};

// use bluetooth::{HciDeviceHandle, BDADDR_ANY};
use mio::{FromFd, TryRead};
use mio::buf::{MutBuf};


const HCISocketToken: mio::Token = mio::Token(0);


fn main() {
	let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK | nix::sys::socket::SOCK_CLOEXEC, 1) {
	// let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_CLOEXEC, 1) {
		Ok(s) => s,
		Err(e) => panic!("err: {:?}", e),
	};

	match nix::sys::ioctl::ioctl(s, nix::sys::ioctl::HCIDEVUP(0)) {
		Ok(()) => (),
		Err(nix::NixError::Sys(nix::errno::EALREADY)) => println!("hci0 already up"),
		Err(e) => panic!("err: {:?}", e),
	}

	match nix::sys::ioctl::ioctl(s, nix::sys::ioctl::HCIDEVRESET(0)) {
		Ok(()) => (),
		Err(e) => panic!("err: {:?}", e),
	}

	match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(0), nix::sys::socket::HciChannel::Raw))) {
		Ok(()) => (),
		Err(e) => panic!("err: {:?}", e),		
	}

	// let msg = [0x01u8, 0x00, 0xff, 0xff, 0x00, 0x00];
	// match nix::unistd::write(s, &msg) {
	// 	Ok(size) => {
	// 		println!("wrote {} bytes:", size);
	// 		println!("    {}", msg.to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let mut buf = [0u8; 256];
	// match nix::unistd::read(s, &mut buf) {
	// 	Ok(size) => {
	// 		println!("read {} bytes:", size);
	// 		println!("    {}", buf[0..size].to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let msg = [0x2cu8, 0x00, 0x00, 0x00, 0x04, 0x00, 0x60, 0x00, 0x30, 0x00];
	// match nix::unistd::write(s, &msg) {
	// 	Ok(size) => {
	// 		println!("wrote {} bytes:", size);
	// 		println!("    {}", msg.to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let mut buf = [0u8; 256];
	// match nix::unistd::read(s, &mut buf) {
	// 	Ok(size) => {
	// 		println!("read {} bytes:", size);
	// 		println!("    {}", buf[0..size].to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// loop {
	// 	let mut buf = [0u8; 256];
	// 	match nix::unistd::read(s, &mut buf) {
	// 		Ok(size) => {
	// 			println!("read {} bytes:", size);
	// 			println!("    {}", buf[0..size].to_hex())
	// 		},
	// 		Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
	// 		Err(e) => panic!("err: {:?}", e),
	// 	}
	// }


	// let d = match bluetooth::HciDeviceHandle::new(&bluetooth::BDADDR_ANY) {
	// 	Ok(d) => d,
	// 	Err(e) => panic!("err: {}", e),
	// };

	// match d.le_set_scan_enable(false, false) {
	// 	Ok(()) => (),
	// 	Err(e) => panic!("err: {}", e),
	// }

	// match d.le_set_scan_parameters(bluetooth::HciLeScanType::Passive, 0x10, 0x10, bluetooth::HciLeScanAddressType::Public, bluetooth::HciLeScanFilter::Default) {
	// 	Ok(()) => (),
	// 	Err(e) => panic!("err: {}", e),
	// }

	// match d.le_set_scan_enable(true, false) {
	// 	Ok(()) => (),
	// 	Err(e) => panic!("err: {}", e),
	// }


	// let fd = d.as_raw_fd();

	// let filter = bluetooth::raw::hci_filter {
	// 	type_mask: 1 << 4,
	// 	event_mask: 0xff,
	// 	opcode: 0,
	// };
	// let rv = unsafe { libc::funcs::bsd43::setsockopt(s, bluetooth::raw::SOL_HCI, bluetooth::raw::HCI_FILTER, (&filter as *const _ as *const libc::c_void), std::mem::size_of_val(&filter) as libc::uint32_t) };
	// if rv < 0 {
	// 	panic!("hmmm");
	// }

	// let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK | nix::sys::socket::SOCK_CLOEXEC, 1) {
	// 	Ok(s) => s,
	// 	Err(e) => panic!("err: {:?}", e),
	// };

	// match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::Dev(0), nix::sys::socket::HciChannel::Raw))) {
	// 	Ok(()) => (),
	// 	Err(e) => panic!("err: {:?}", e),		
	// }

	// loop {
	// 	let mut buf = [0u8; 256];
	// 	match nix::unistd::read(fd, &mut buf) {
	// 		Ok(size) => {
	// 			println!("read {} bytes:", size);
	// 			println!("    {}", buf[0..size].to_hex())
	// 		},
	// 		Err(nix::NixError::Sys(nix::errno::EAGAIN)) => (),
	// 		Err(e) => panic!("err: {:?}", e),
	// 	}
	// }

	// let mut buf = [0u8; 256];
	// match nix::unistd::read(s, &mut buf) {
	// 	Ok(size) => {
	// 		println!("read {} bytes:", size);
	// 		println!("    {}", buf[0..size].to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let mut buf = [0u8; 256];
	// match nix::unistd::read(s, &mut buf) {
	// 	Ok(size) => {
	// 		println!("read {} bytes:", size);
	// 		println!("    {}", buf[0..size].to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let mut buf = [0u8; 256];
	// match nix::unistd::read(s, &mut buf) {
	// 	Ok(size) => {
	// 		println!("read {} bytes:", size);
	// 		println!("    {}", buf[0..size].to_hex())
	// 	},
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// match nix::unistd::write(s, &[0x00u8, 0x01, 0xff, 0xff, 0x00, 0x00]) {
	// 	Ok(size) => println!("wrote {} bytes", size),
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// let opcode: u16 = 0x03 << 10 | 0x03;
	// let command = [0x01u8, (opcode >> 8) as u8, (opcode & 0xff) as u8, 0x00];
	// println!("write: {:?}", command.to_hex());
	// match nix::unistd::write(s, &command) {
	// 	Ok(size) => println!("wrote {} bytes", size),
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// // nop
	// let opcode: u16 = 0;
	// let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	// WriteBytesExt::write_u8(&mut command, 0x01);
	// WriteBytesExt::write_u16::<BigEndian>(&mut command, opcode);
	// WriteBytesExt::write_u8(&mut command, 0x00);

	// println!("write: {:?}", &*command.to_hex());
	// match nix::unistd::write(s, &*command) {
	// 	Ok(size) => println!("wrote {} bytes", size),
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// // reset
	// let opcode: u16 = 0x03 << 10 | 0x03;
	// let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	// WriteBytesExt::write_u8(&mut command, 0x01);
	// WriteBytesExt::write_u16::<LittleEndian>(&mut command, opcode);
	// WriteBytesExt::write_u8(&mut command, 0x00);

	// println!("write: {:?}", &*command.to_hex());
	// match nix::unistd::write(s, &*command) {
	// 	Ok(size) => println!("wrote {} bytes", size),
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	// match nix::sys::socket::getsockopt(s, nix::sys::socket::SockLevel::Ip, nix::sys::socket::sockopt::HciFilter) {
	// 	Ok(f) => println!("filter: {:?}", f),
	// 	Err(e) => panic!("err: {:?}", e),
	// }

	let f = nix::sys::socket::sockopt::HciFilterValue {
		type_mask: 0xffffffff,
		event_mask: [0xffffffff, 0xffffffff],
		opcode: 0,
	};
	match nix::sys::socket::setsockopt(s, nix::sys::socket::SockLevel::Ip, nix::sys::socket::sockopt::HciFilter, &f) {
		Ok(()) => (),
		Err(e) => panic!("err: {:?}", e),
	}


	// event mask
	// let opcode: u16 = 0x03 << 10 | 0x0001;
	let opcode = bluetooth::HciOpcode::Controller(bluetooth::HciControllerOpcode::SetEventMask).to_u16();
	let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, opcode);
	WriteBytesExt::write_u8(&mut command, 0x08);
	WriteBytesExt::write_u64::<LittleEndian>(&mut command, 0x3dbff807fffbffff as u64);

	println!("write: {:?}", bluetooth::HciPacket::from_bytes(&*command));
	match nix::unistd::write(s, &*command) {
		Ok(size) => println!("wrote {} bytes", size),
		Err(e) => panic!("err: {:?}", e),
	}

	// le event mask
	let opcode: u16 = 0x08 << 10 | 0x0001;
	let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, opcode);
	WriteBytesExt::write_u8(&mut command, 0x08);
	WriteBytesExt::write_u64::<LittleEndian>(&mut command, 0x1f as u64);

	println!("write: {:?}", bluetooth::HciPacket::from_bytes(&*command));
	match nix::unistd::write(s, &*command) {
		Ok(size) => println!("wrote {} bytes", size),
		Err(e) => panic!("err: {:?}", e),
	}

	// le scan parameters
	let opcode: u16 = 0x08 << 10 | 0x000b;
	let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, opcode);
	WriteBytesExt::write_u8(&mut command, 0x07);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, 0x0190);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, 0x0010);
	WriteBytesExt::write_u8(&mut command, 0x00);
	WriteBytesExt::write_u8(&mut command, 0x00);

	println!("write: {:?}", bluetooth::HciPacket::from_bytes(&*command));
	match nix::unistd::write(s, &*command) {
		Ok(size) => println!("wrote {} bytes", size),
		Err(e) => panic!("err: {:?}", e),
	}

	// le scan enable
	let opcode: u16 = 0x08 << 10 | 0x000c;
	let mut command: std::vec::Vec<u8> = std::vec::Vec::with_capacity(4);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u16::<LittleEndian>(&mut command, opcode);
	WriteBytesExt::write_u8(&mut command, 0x02);
	WriteBytesExt::write_u8(&mut command, 0x01);
	WriteBytesExt::write_u8(&mut command, 0x00);

	println!("write: {:?}", bluetooth::HciPacket::from_bytes(&*command));
	match nix::unistd::write(s, &*command) {
		Ok(size) => println!("wrote {} bytes", size),
		Err(e) => panic!("err: {:?}", e),
	}


	let sio = mio::Io::new(s);
	let mut scanner = BleScanner::new(sio);

	match scanner.run() {
		Ok(_) => (),
		Err(e) => panic!("err: {}", e),
	}

	match nix::sys::ioctl::ioctl(s, nix::sys::ioctl::HCIDEVDOWN(0)) {
		Ok(()) => (),
		Err(nix::NixError::Sys(nix::errno::EALREADY)) => println!("hci0 already down"),
		Err(e) => panic!("err: {:?}", e),
	}
}


struct BleScanner {
	hcisock: mio::Io,
}

impl BleScanner {
	fn new(s: mio::Io) -> BleScanner {
		BleScanner {
			hcisock: s,
		}
	}

	pub fn run(&mut self) -> std::io::Result<()> {
		let mut event_loop = match mio::EventLoop::new() {
			Ok(e) => e,
			Err(e) => panic!("err: {}", e),
		};

		match event_loop.register_opt(&self.hcisock, HCISocketToken, mio::Interest::readable(), mio::PollOpt::edge()) {
			Ok(_) => (),
			Err(e) => panic!("err: {}", e),
		}

		event_loop.run(self)
	}
}

impl mio::Handler for BleScanner {
	type Timeout = usize;
	type Message = ();

	fn readable(&mut self, event_loop: &mut mio::EventLoop<BleScanner>, token: mio::Token, hint: mio::ReadHint) {
		match token {
			HCISocketToken => {
				let mut buf = [0; 260];
				match self.hcisock.read(&mut mio::buf::MutSliceBuf::wrap(&mut buf)) {
					Ok(None) => {
						panic!("We just got readable, but were unable to read from the socket?");
					}
					Ok(Some(r)) => {
						println!("data:   {:?}", buf[0..r].to_hex());
						if let Some(p) = bluetooth::HciPacket::from_bytes(&buf[0..r]) {
							println!("packet: {:?}", p);
							match p {
								bluetooth::HciPacket::Event(p) => {
									if let Some(e) = p.to_event() {
										println!("event:  {:?}", e);
									}
								},
								_ => (),
							}
						}
					}
					Err(e) => {
						match e.kind() {
							std::io::ErrorKind::BrokenPipe => {
								println!("broken pipe");
								event_loop.shutdown();
							},
							_ => panic!("not implemented; client err={:?}", e),
						}
					}
				}
			},
			mio::Token(t) => {
				panic!("unknown token: {:?}", t);
			},
		}
	}
}