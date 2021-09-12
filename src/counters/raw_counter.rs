use std::io::{Result, Write};

use super::{CountWrite, Counter};

pub struct RawCounter {
  inner: CountWrite,
}

impl RawCounter {
  pub fn new() -> Self {
    Self {
      inner: CountWrite::new(),
    }
  }
}

impl Counter for RawCounter {
  fn count(&self) -> usize {
    self.inner.count()
  }
}

impl Write for RawCounter {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.inner.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.inner.flush()
  }
}
