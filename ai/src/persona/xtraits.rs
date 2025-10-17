// evaluate each x_traits ranging [-3 to +3] in observable enforcements of its validity
// and behaviours in using Rust compiler, LLM models, and KP custom gdb
#[derive(Debug)]
pub struct KpT {
  value: i32,
}
impl KpT {
  pub fn new(value: i32) -> KpT {
    if value < -4 || value > 4 {
            panic!("KpT value must be between -4 to +4, got {}.", value);
    }

    KpT { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

