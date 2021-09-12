use std::io::Write;

pub trait Counter: Write + Send {
  fn count(&self) -> usize;
}
