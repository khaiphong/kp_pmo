#[derive(Debug)]
pub struct HuiNeng {}

#[derive(Debug)]
pub struct Gotama {}

// this is custom type KpF from [-8 to +8] to enforce its validity and behaviours in using
// Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpF {
  value: i32,
}
impl KpF {
  pub fn new(value: i32) -> KpF {
    if value < -8 || value > 8 {
            panic!("KpF value must be between -8 to +8, got {}.", value);
    }

    KpF { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

