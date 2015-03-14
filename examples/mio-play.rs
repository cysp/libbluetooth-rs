#![feature(libc)]
#![feature(std_misc)]

extern crate libc;

extern crate mio;

use std::os::unix::AsRawFd;

use mio::{FromFd, TryRead};
use mio::buf::{MutBuf};


const HCISCAN: mio::Token = mio::Token(0);


fn main() {
	let f = match std::fs::File::open("/dev/random") {
		Ok(f) => f,
    	Err(e) => panic!("err: {:?}", e),
	};

	let fd = f.as_raw_fd();

	let mut event_loop = match mio::EventLoop::new() {
    	Ok(e) => e,
    	Err(e) => panic!("err: {:?}", e),
	};

	let sock = mio::Io::new(fd);

    match event_loop.register_opt(&sock, HCISCAN, mio::Interest::readable(), mio::PollOpt::edge() | mio::PollOpt::oneshot()) {
    	Ok(_) => (),
    	Err(e) => panic!("err: {}", e),
    }
    
    match event_loop.run(&mut BleScanner::new(sock)) {
    	Ok(_) => (),
    	Err(e) => panic!("err: {:?}", e),
    }
}


struct BleScanner {
	sock: mio::Io,
}

impl BleScanner {
	fn new(sock: mio::Io) -> BleScanner {
		BleScanner { sock: sock }
	}
}

impl mio::Handler for BleScanner {
    type Timeout = usize;
    type Message = ();

    fn readable(&mut self, event_loop: &mut mio::EventLoop<BleScanner>, token: mio::Token, hint: mio::ReadHint) {
    	let mut buf = mio::buf::ByteBuf::mut_with_capacity(20);
    	match self.sock.read(&mut buf) {
            Ok(None) => {
                panic!("We just got readable, but were unable to read from the socket?");
            }
            Ok(Some(r)) => {
                println!("CLIENT : We read {} bytes!", r);
                println!("data: {:?}", buf.mut_bytes());
                panic!("");
            }
            Err(e) => {
                panic!("not implemented; client err={:?}", e);
            }
    	}
    }

}