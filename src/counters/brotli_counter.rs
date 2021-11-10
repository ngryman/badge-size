use brotli::CompressorWriter;
use std::io::{Result, Write};

use super::{CountWrite, Counter};

/// Compression quality [0-11]
///
/// [Apache] defaults: `5`
/// [Nginx] defaults: `6`
///
///
/// [apache]: https://httpd.apache.org/docs/2.4/mod/mod_brotli.html
/// [nginx]: https://github.com/google/ngx_brotli
const BROTLI_QUALITY: u32 = 6;

/// Window size
///
/// [Apache] defaults: `18`
/// [Nginx] defaults: `19 (512k)`
///
/// [apache]: https://httpd.apache.org/docs/2.4/mod/mod_brotli.html
/// [nginx]: https://github.com/google/ngx_brotli
const BROTLI_LGWIN: u32 = 19;

pub struct BrotliCounter {
  inner: CompressorWriter<CountWrite>,
}

impl BrotliCounter {
  pub fn new() -> Self {
    Self {
      inner: CompressorWriter::new(
        CountWrite::new(),
        4096,
        BROTLI_QUALITY,
        BROTLI_LGWIN,
      ),
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
