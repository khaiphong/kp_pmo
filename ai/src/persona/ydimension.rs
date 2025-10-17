#[derive(Debug)]
pub struct TranscendentalMeditation {}

#[derive(Debug)]
pub struct Vipassana {}

#[derive(Debug)]
pub struct Prana {}

#[derive(Debug)]
pub struct Tantra {}

// this is custom type KpY from [-6 to +6] to enforce its validity and behaviours in using
// Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpY {
  value: i32,
}
impl KpY {
  pub fn new(value: i32) -> KpY {
    if value < -6 || value > 6 {
            panic!("KpY value must be between -6 to +6, got {}.", value);
    }

    KpY { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

