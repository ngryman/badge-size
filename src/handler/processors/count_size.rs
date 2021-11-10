use async_std::io::ReadExt;
use std::io::Write;
use tide::Result;

use crate::{
  counters::{BrotliCounter, Counter, GzipCounter, RawCounter},
  options::Compression,
  Context,
};

pub async fn count_size(mut context: Context) -> Result<Context> {
  let mut raw_counter = RawCounter::new();
  let mut compression_counter: Option<Box<dyn Counter>> =
    match context.options.compression {
      Compression::Brotli => Some(Box::new(BrotliCounter::new())),
      Compression::Gzip => Some(Box::new(GzipCounter::new())),
      Compression::None => None,
    };

  let reader = context.source_reader.as_mut().unwrap();
  let mut buf = [0; 4096];

  loop {
    let bytes_read = reader.read(&mut buf).await?;
    let buf_slice = &buf[0..bytes_read];

    if bytes_read > 0 {
      raw_counter.write_all(buf_slice)?;
      if let Some(ref mut counter) = compression_counter {
        counter.write_all(buf_slice)?;
        counter.flush()?;
      }
    } else {
      break;
    };
  }

  context.original_size = raw_counter.count();
  context.size = match compression_counter {
    Some(counter) => counter.count(),
    None => context.original_size,
  };
  context.pretty_size = bytesize::to_string(context.size as u64, false);

  Ok(context)
}
