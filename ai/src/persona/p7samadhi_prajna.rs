// evaluate each p7_SamadhiPrajna ranging from [0 to +5] from qualified to NoThingness
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP7 {
  value: i32,
}
impl KpP7 {
  pub fn new(value: i32) -> KpP7 {
    if value < 0 || value > 5 {
            panic!("KpP7 value must be between 0 to 5, got {}.", value);
    }

    KpP7 { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

