// evaluate each p8_PrajnaTIP1 ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP8 {
  value: i32,
}
impl KpP8 {
  pub fn new(value: i32) -> KpP8 {
    if value < 0 || value > 5 {
            panic!("KpP8 value must be between 0 to 5, got {}.", value);
    }

    KpP8 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

