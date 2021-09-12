use flate2::{write::GzEncoder, Compression};
use std::io::{Result, Write};

use super::{CountWrite, Counter};

pub struct GzipCounter {
  inner: GzEncoder<CountWrite>,
}

impl GzipCounter {
  pub fn new() -> Self {
    Self {
      inner: GzEncoder::new(CountWrite::new(), Compression::default()),
    }
  }
}

impl Counter for GzipCounter {
  fn count(&self) -> usize {
    self.inner.get_ref().count()
  }
}

impl Write for GzipCounter {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.inner.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.inner.flush()
  }
}
