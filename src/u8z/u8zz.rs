/*
  module u8z::u8zz
*/

// pub mod u8zz {

pub struct CArgsBuf {
  pub argc: usize,
  pub argv: Vec<*mut i8>,
  pub u8zz: Vec<u8>
}

impl CArgsBuf {
  pub fn from(args: &Vec<String>) -> Self {
    let argc: usize = args.len();
    let u8zz_len: usize = (0..argc).map(|i| args[i].len()).sum();
    let mut s = Self{
      argc: argc,
      argv: vec![0 as *mut i8; argc],
      u8zz: vec![0u8; u8zz_len + argc + 1]};
    let mut p: usize = 0;
    for i in 0..argc {
      s.argv[i] = ((&s.u8zz[0] as *const u8) as usize + p) as *mut i8;
      for j in 0..args[i].len() {
        s.u8zz[p] = args[i].as_bytes()[j];
        p += 1;
      }
      s.u8zz[p] = b'\0';
      p += 1;
    }
    s.u8zz[p] = b'\0';
/*
    println!("{:?}, {}", s.argv, u8zz_len);
    for i in 0..argc {
      let mut j: usize = 0;
      loop {
        let c: u8 = unsafe { *((s.argv[i] as usize + j) as *const u8) };
        if c == b'\0' { print!(" {:02x}", c); break; }
        else{ print!("{}", c as char); }
        j += 1;
      }
      println!("");
    }
*/
    s
  }

  pub fn as_argc(&self) -> i32 {
    self.argc as i32
  }

  pub fn as_argv_ptr_mut(&mut self) -> *mut *mut i8 {
    let av: &mut [*mut i8] = &mut self.argv;
    av as *mut [*mut i8] as *mut *mut i8
  }

  pub fn as_argv_ptr(&self) -> *const *mut i8 {
    let av: &[*mut i8] = &self.argv;
    av as *const [*mut i8] as *const *mut i8
  }

  pub fn as_flat_u8ptr_mut(&mut self) -> *mut u8 {
    let p: &mut [u8] = &mut self.u8zz;
    p as *mut [u8] as *mut u8
  }

  pub fn as_flat_u8ptr(&self) -> *const u8 {
    let p: &[u8] = &self.u8zz;
    p as *const [u8] as *const u8
  }

  pub fn as_flat_ptr_mut(&mut self) -> *mut i8 {
    self.as_flat_u8ptr_mut() as *mut i8
  }

  pub fn as_flat_ptr(&self) -> *const i8 {
    self.as_flat_u8ptr() as *const i8
  }
}

// }
