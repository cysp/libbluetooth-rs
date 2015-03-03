// use super::raw;


pub enum BdAddr {
	Any,
	All,
	Local,
	Addr(u8, u8, u8, u8, u8, u8),
}
