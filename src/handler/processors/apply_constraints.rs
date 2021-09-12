use crate::{options::Color, Context};
use tide::Result;

pub async fn apply_constraints(mut context: Context) -> Result<Context> {
  let Context { options, .. } = &context;

  context.color = match options.color {
    None => {
      if context.size > options.softmax {
        Color::Yellow
      } else if context.size > options.max {
        Color::Red
      } else {
        context.color
      }
    }
    Some(_) => context.color,
  };

  Ok(context)
}
