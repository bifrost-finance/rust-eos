use serde::{Deserialize, Serialize};


pub trait Client {
    fn node(&self) -> &str;

    fn fetch<T>(&self, path: impl AsRef<str>, params: impl Serialize) -> crate::Result<T>
        where T: 'static + for<'b> Deserialize<'b> + Send + Sync;
}
