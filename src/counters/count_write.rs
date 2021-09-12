use std::io::{Result, Write};

use super::Counter;

pub struct CountWrite {
  count: usize,
}

impl CountWrite {
  pub fn new() -> Self {
    Self { count: 0 }
  }
}

impl Counter for CountWrite {
  fn count(&self) -> usize {
    self.count
  }
}

impl Write for CountWrite {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    // let written = match buf.iter().position(|c| *c == 0) {
    //   Some(written) => written,
    //   None => buf.len(),
    // };
    let written = buf.len();
    self.count += written;
    Ok(written)
  }

  fn flush(&mut self) -> Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use std::io::Write;

  use super::{CountWrite, Counter};

  #[test]
  fn count_size_of_a_slice() {
    // Arrange
    let buf: [u8; 1024] = [1; 1024];
    // Act
    let mut cw = CountWrite::new();
    cw.write(&buf).unwrap();
    // Assert
    assert_eq!(cw.count(), buf.len());
  }
}
