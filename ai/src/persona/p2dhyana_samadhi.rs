// evaluate each p1_EmptyTheContent ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP2 custom gdb
#[derive(Debug)]
pub struct KpP2 {
  value: i32,
}
impl KpP2 {
  pub fn new(value: i32) -> KpP2 {
    if value < 0 || value > 5 {
            panic!("KpP2 value must be between 0 to 5, got {}.", value);
    }

    KpP2 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

