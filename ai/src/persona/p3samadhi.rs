// evaluate each p1_EmptyTheContent ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP3 {
  value: i32,
}
impl KpP3 {
  pub fn new(value: i32) -> KpP3 {
    if value < 0 || value > 5 {
            panic!("KpP3 value must be between 0 to 5, got {}.", value);
    }

    KpP3 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

