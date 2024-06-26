use std::future::Future;
use tokio::task::JoinSet;

pub trait IntoJoinSet: Iterator {
  fn into_join_set<T>(self) -> JoinSet<T>
  where
    Self: Sized,
    Self::Item: Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    self.collect()
  }

  fn into_join_set_by<T, F, M>(self, f: M) -> JoinSet<T>
  where
    Self: Sized,
    Self::Item: Send + 'static,
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
    M: FnMut(Self::Item) -> F,
  {
    self.map(f).into_join_set()
  }
}

impl<T> IntoJoinSet for T where T: Iterator + ?Sized {}

#[cfg(test)]
mod tests {
  use super::*;
  use std::future;

  #[tokio::test]
  async fn into_join_set() {
    let mut set = (0..10)
      .into_iter()
      .map(future::ready)
      .into_join_set();

    assert!(set.len() == 10);

    while let Some(result) = set.join_next().await {
      result.unwrap();
    }
  }

  #[tokio::test]
  async fn into_join_set_by() {
    let mut set = (0..10)
      .into_iter()
      .into_join_set_by(future::ready);

    assert!(set.len() == 10);

    while let Some(result) = set.join_next().await {
      result.unwrap();
    }
  }
}
