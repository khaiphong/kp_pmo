// evaluate each p9_PrajnaTIP2 ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP9 {
  value: i32,
}
impl KpP9 {
  pub fn new(value: i32) -> KpP9 {
    if value < 0 || value > 5 {
            panic!("KpP9 value must be between 0 to 5, got {}.", value);
    }

    KpP9 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

