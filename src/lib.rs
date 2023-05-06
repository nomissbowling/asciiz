/// asciiz create buffer and copy bytes ends with 0u8
///
/// #Examples
/// Keep lifetime
/// ```
/// use asciiz::u8z::U8zBuf;
///
/// let b = b"bytesarray";
/// let mut m = U8zBuf::from_u8array(b);
/// assert_eq!(m.as_u8p() as usize, m.as_u8p_mut() as usize);
/// // let r = unsafe { c_function_requires_asciiz_const(m.as_i8p()) };
/// // let s = unsafe { c_function_requires_asciiz_not_const(m.as_i8p_mut()) };
///
/// ```

pub mod u8z;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_const_i8p_eq_u8p() {
    let b = b"ab";
    let c = u8z::U8zBuf::from_u8array(b);
    assert_eq!(c.as_i8p() as usize, c.as_u8p() as usize);
  }

  #[test]
  fn b_const_eq_mut() {
    let b = b"ab";
    let mut m = u8z::U8zBuf::from_u8array(b);
    assert_eq!(m.as_u8p() as usize, m.as_u8p_mut() as usize);
    assert_eq!(m.as_i8p() as usize, m.as_i8p_mut() as usize);
    assert_eq!(m.as_i8p() as usize, m.as_u8p() as usize);
  }

  #[test]
  fn c_check_array() {
    let b = b"x\0yz"; // includes 0u8
    let mut m = u8z::U8zBuf::from_u8array(b);
    let s: usize = b.len() + 1;
    assert_eq!(s, 5);
    let d: &mut Vec<u8> = &mut vec![255u8; s]; // fill not 0u8
    d[..].copy_from_slice(
      unsafe { std::slice::from_raw_parts(m.as_u8p_mut(), s) });
    assert_eq!(d[..(s - 1)], b.to_vec()); // check bytes
    assert_eq!(d[s - 1], 0u8); // end 0u8
  }

  #[test]
  fn d_check_u8zz() {
    let a: Vec<String> = vec!["Z".to_string(), "A".to_string()];
    let cab = u8z::u8zz::CArgsBuf::from(&a);
    assert_eq!(cab.as_argc(), 2);
    let s: usize = 5;
    let d: &mut Vec<u8> = &mut vec![255u8; s]; // fill not 0u8
    d[..].copy_from_slice(
      unsafe { std::slice::from_raw_parts(cab.as_flat_ptr() as *const u8, s) });
    assert_eq!(d[..(s - 1)], b"Z\0A\0".to_vec()); // check bytes
    assert_eq!(d[s - 1], 0u8); // end 0u8
  }
}
