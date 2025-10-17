// evaluate each p5_Prajna ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP5 {
  value: i32,
}
impl KpP5 {
  pub fn new(value: i32) -> KpP5 {
    if value < 0 || value > 5 {
            panic!("KpP5 value must be between 0 to 5, got {}.", value);
    }

    KpP5 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

