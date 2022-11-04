use std::fmt::Display;

pub trait Join {
  type Item;

  fn join(&mut self, sep: &str) -> String
  where
    Self::Item: Display;

  fn join_format<F>(&mut self, sep: &str, f: F) -> String
  where
    F: Fn(&Self::Item) -> String;
}

impl<I> Join for I
where
  I: Iterator,
{
  type Item = I::Item;

  fn join(&mut self, sep: &str) -> String
  where
    Self::Item: Display,
  {
    self.join_format(sep, Self::Item::to_string)
  }

  fn join_format<F>(&mut self, sep: &str, f: F) -> String
  where
    F: Fn(&Self::Item) -> String,
  {
    self
      .enumerate()
      .fold(String::new(), |mut accum, (index, item)| {
        if index != 0 {
          accum += sep
        }
        accum + &f(&item)
      })
  }
}

#[cfg(test)]
mod tests {
  use super::Join;

  #[test]
  fn join_iter() {
    assert_eq!(
      "392, 345, 4598645, 2134",
      [392, 345, 4598645, 2134].iter().join(", ")
    )
  }

  #[test]
  fn join_range() {
    assert_eq!(
      "1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10",
      (1..=10).join(" | ")
    )
  }
}
