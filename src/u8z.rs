/*
  u8z::{u8zz, ...}
*/

pub mod u8zz;

pub struct U8zBuf {
  buf: Vec<u8>
}

impl U8zBuf {

pub fn from_u8array(a: &[u8]) -> U8zBuf {
  let mut ub = U8zBuf{buf: vec![0u8; a.len() + 1]}; // must end 0
  // ub.buf.resize_with(a.len() + 1, || { 0u8 }); // Default::default needless
  ub.buf[..a.len()].copy_from_slice(&a[..]);
  ub
}

pub fn as_u8p_mut(&mut self) -> *mut u8 {
  &mut self.buf[0] as *mut u8
}

pub fn as_i8p_mut(&mut self) -> *mut i8 {
  self.as_u8p_mut() as *mut i8
}

pub fn as_u8p(&self) -> *const u8 {
  &self.buf[0] as *const u8
}

pub fn as_i8p(&self) -> *const i8 {
  self.as_u8p() as *const i8
}

}
