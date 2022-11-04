use std::fmt::Display;

pub trait Join {
  type Item;

  /// Returns a string that contains the items separated by the separator string.
  /// ```rust
  /// use stringx::Join;
  ///
  /// assert_eq!((1..=5).join(", "), "1, 2, 3, 4, 5")
  /// ```
  fn join(&mut self, sep: &str) -> String
  where
    Self::Item: Display;

  /// Works like [`Join::join`] but takes an additional formatting closure as argument.
  /// ```rust
  /// use stringx::Join;
  ///
  /// let string = [Some(8379), Some(990), None, Some(974385)]
  /// 	.iter()
  /// 	.join_format(" | ", |item| match item {
  /// 		Some(number) => number.to_string(),
  /// 		None => "_".into(),
  /// 	});
  /// assert_eq!(string, "8379 | 990 | _ | 974385");
  /// ```
  fn join_format<F>(&mut self, sep: &str, f: F) -> String
  where
    F: Fn(&Self::Item) -> String;
}

impl<I: Iterator> Join for I {
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
