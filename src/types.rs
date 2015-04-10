extern crate std;


#[derive(Clone,Copy)]
pub struct BdAddr(pub [u8; 6]);

impl BdAddr {
	pub fn to_raw(&self) -> &[u8; 6] {
		&self.0
	}
}


impl From<[u8; 6]> for BdAddr {
	fn from(value: [u8; 6]) -> BdAddr {
		BdAddr(value)
	}
}

impl<'a> From<&'a [u8; 6]> for BdAddr {
		fn from(value: &'a [u8; 6]) -> BdAddr {
		BdAddr(*value)
	}
}

impl AsRef<[u8; 6]> for BdAddr {
	fn as_ref(&self) -> &[u8; 6] {
		&self.0
	}
}

impl std::ops::Deref for BdAddr {
	type Target = [u8; 6];
	fn deref<'a>(&'a self) -> &'a [u8; 6] {
		&self.0
	}
}
