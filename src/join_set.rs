use std::future::Future;
use tokio::task::JoinSet;

pub mod prelude {
  pub use super::{IntoJoinSet as _, JoinSetFromIter as _};
}

pub trait JoinSetFromIter: Iterator {
  fn join_set<T>(self) -> JoinSet<T>
  where
    Self: Sized,
    Self::Item: Future<Output = T> + Send + 'static,
    T: Send + 'static,
  {
    self.collect()
  }

  fn join_set_by<T, F, M>(self, f: M) -> JoinSet<T>
  where
    Self: Sized,
    Self::Item: Send + 'static,
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
    M: FnMut(Self::Item) -> F,
  {
    self.map(f).join_set()
  }
}

impl<T> JoinSetFromIter for T where T: Iterator + ?Sized {}

pub trait IntoJoinSet<F, T>: IntoIterator
where
  Self: Sized,
  <Self as IntoIterator>::Item: Future<Output = T> + Send + 'static,
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  fn into_join_set(self) -> JoinSet<T> {
    self.into_iter().join_set()
  }

  fn into_join_set_by<M>(self, f: M) -> JoinSet<T>
  where
    M: FnMut(Self::Item) -> F,
  {
    self.into_iter().join_set_by(f)
  }
}

impl<F, T> IntoJoinSet<F, T> for Vec<F>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::future;

  #[tokio::test]
  async fn join_set_by() {
    let mut set = (0..10).into_iter().join_set_by(future::ready);

    assert!(set.len() == 10);

    while let Some(result) = set.join_next().await {
      result.unwrap();
    }
  }
}
