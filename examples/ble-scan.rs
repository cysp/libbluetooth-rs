#![feature(libc)]
#![feature(std_misc)]

// extern crate libc;
extern crate serialize;

extern crate bluetooth;
// extern crate mio;
extern crate nix;

use serialize::hex::ToHex;

use std::os::unix::AsRawFd;

// use bluetooth::{HciDeviceHandle, BDADDR_ANY};
// use mio::{FromFd, TryRead};
// use mio::buf::{MutBuf};


// const HCISCAN: mio::Token = mio::Token(0);


fn main() {
	// let s = match nix::sys::socket::socket(nix::sys::socket::AddressFamily::Bluetooth, nix::sys::socket::SockType::Raw, nix::sys::socket::SOCK_NONBLOCK | nix::sys::socket::SOCK_CLOEXEC, 1) {
	// 	Ok(s) => s,
	// 	Err(e) => panic!("err: {:?}", e),
	// };

	// match nix::sys::socket::bind(s, &nix::sys::socket::SockAddr::new_hci(nix::sys::socket::HciAddr::new_bluetooth(nix::sys::socket::HciDev::None, nix::sys::socket::HciChannel::Control))) {
	// 	Ok(()) => (),
	// 	Err(e) => panic!("err: {:?}", e),		
	// }

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
	// let rv = unsafe { libc::funcs::bsd43::setsockopt(fd, bluetooth::raw::SOL_HCI, bluetooth::raw::HCI_FILTER, (&filter as *const _ as *const libc::c_void), std::mem::size_of_val(&filter) as libc::uint32_t) };
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

	// let mut event_loop = match mio::EventLoop::new() {
 //    	Ok(e) => e,
 //    	Err(e) => panic!("err: {}", e),
	// };

	// let sock = mio::Io::new(fd);

 //    match event_loop.register_opt(&sock, HCISCAN, mio::Interest::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()) {
 //    	Ok(_) => (),
 //    	Err(e) => panic!("err: {}", e),
 //    }
    
 //    match event_loop.run(&mut BleScanner::new(sock)) {
 //    	Ok(_) => (),
 //    	Err(e) => panic!("err: {}", e),
 //    }
}


// struct BleScanner {
// 	sock: mio::Io,
// }

// impl BleScanner {
// 	fn new(sock: mio::Io) -> BleScanner {
// 		BleScanner { sock: sock }
// 	}
// }

// impl mio::Handler for BleScanner {
//     type Timeout = usize;
//     type Message = ();

//     fn readable(&mut self, event_loop: &mut mio::EventLoop<BleScanner>, token: mio::Token, hint: mio::ReadHint) {
//     	let mut buf = mio::buf::ByteBuf::mut_with_capacity(140);
//     	match self.sock.read(&mut buf) {
//             Ok(None) => {
//                 panic!("We just got readable, but were unable to read from the socket?");
//             }
//             Ok(Some(r)) => {
//                 println!("CLIENT : We read {} bytes!", r);
//                 println!("data: {:?}", buf.mut_bytes());
//             }
//             Err(e) => {
//                 panic!("not implemented; client err={:?}", e);
//             }
//     	}
//     }
// }