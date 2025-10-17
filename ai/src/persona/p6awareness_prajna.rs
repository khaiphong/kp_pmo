// evaluate each p6_AwarenessPrajna ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP6 {
  value: i32,
}
impl KpP6 {
  pub fn new(value: i32) -> KpP6 {
    if value < 0 || value > 5 {
            panic!("KpP6 value must be between 0 to 5, got {}.", value);
    }

    KpP6 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

