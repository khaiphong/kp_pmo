// evaluate each y_pointers ranging from [-3 to +3] in observable enforcements of its behaviours
// in Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpP {
  value: i32,
}
impl KpP {
  pub fn new(value: i32) -> KpP {
    if value < 0 || value > 9 {
            panic!("KpP value must be between 0 to 9, got {}.", value);
    }

    KpP { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

