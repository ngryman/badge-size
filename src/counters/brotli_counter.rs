use brotli::CompressorWriter;
use std::io::{Result, Write};

use super::{CountWrite, Counter};

pub struct BrotliCounter {
  inner: CompressorWriter<CountWrite>,
}

impl BrotliCounter {
  pub fn new() -> Self {
    Self {
      inner: CompressorWriter::new(CountWrite::new(), 4096, 6, 6),
    }
  }
}

impl Counter for BrotliCounter {
  fn count(&self) -> usize {
    self.inner.get_ref().count()
  }
}

impl Write for BrotliCounter {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.inner.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.inner.flush()
  }
}
