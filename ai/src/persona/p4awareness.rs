// evaluate each p4_Awareness ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP4 {
  value: i32,
}
impl KpP4 {
  pub fn new(value: i32) -> KpP4 {
    if value < 0 || value > 5 {
            panic!("KpP4 value must be between 0 to 5, got {}.", value);
    }

    KpP4 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

