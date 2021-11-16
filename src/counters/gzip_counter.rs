use flate2::{write::GzEncoder, Compression};
use std::io::{Result, Write};

use super::{CountWrite, Counter};

/// Compression level [1-9]
///
/// [Apache] defaults: `6`
/// [Nginx] defaults: `1`
///
/// [apache]: https://httpd.apache.org/docs/2.4/mod/mod_deflate.html#deflatecompressionlevel
/// [nginx]: http://nginx.org/en/docs/http/ngx_http_gzip_module.html#gzip_comp_level
const GZIP_COMPRESSION: u32 = 6;

pub struct GzipCounter {
  inner: GzEncoder<CountWrite>,
}

impl GzipCounter {
  pub fn new() -> Self {
    Self {
      inner: GzEncoder::new(
        CountWrite::new(),
        Compression::new(GZIP_COMPRESSION),
      ),
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
