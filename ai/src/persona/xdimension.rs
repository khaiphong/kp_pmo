#[derive(Debug)]
pub struct WuNien {}

// this is custom type KpX from [-7 to +7] to enforce its validity and behaviours in using
// Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpX {
  value: i32,
}
impl KpX {
  pub fn new(value: i32) -> KpX {
    if value < -7 || value > 7 {
            panic!("KpX value must be between -7 to +7, got {}.", value);
    }

    KpX { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

