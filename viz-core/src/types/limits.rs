use std::{convert::Infallible, sync::Arc};

use crate::{async_trait, FromRequest, Request, RequestExt};

#[cfg(feature = "form")]
use super::Form;

#[cfg(feature = "json")]
use super::Json;

#[cfg(any(feature = "form", feature = "json"))]
use super::Payload;

/// Extracts the limits settings.
#[derive(Debug, Clone)]
pub struct Limits {
    inner: Arc<Vec<(&'static str, u64)>>,
}

impl Default for Limits {
    fn default() -> Self {
        let limits = Limits::new()
            .set("bytes", Limits::NORMAL)
            .set("payload", Limits::NORMAL)
            .set("text", Limits::NORMAL);

        #[cfg(feature = "json")]
        let limits = limits.set(<Json as Payload>::NAME, <Json as Payload>::LIMIT);

        #[cfg(feature = "form")]
        let limits = limits.set(<Form as Payload>::NAME, <Form as Payload>::LIMIT);

        limits.sort()
    }
}

impl Limits {
    /// By default 1024 * 8 = 8KB.
    pub const NORMAL: u64 = 1024 * 8;

    /// Creates a new Limits.
    #[must_use]
    pub fn new() -> Self {
        Limits {
            inner: Arc::new(Vec::new()),
        }
    }

    /// Sorts the limits for binary search.
    #[must_use]
    pub fn sort(mut self) -> Self {
        Arc::make_mut(&mut self.inner).sort_by_key(|a| a.0);
        self
    }

    /// Sets a name-limit pair into the Limits.
    #[must_use]
    pub fn set(mut self, name: &'static str, limit: u64) -> Self {
        if let Some(val) = self
            .inner
            .binary_search_by_key(&name, |&(a, _)| a)
            .ok()
            .and_then(|i| Arc::make_mut(&mut self.inner).get_mut(i))
        {
            val.1 = limit;
        } else {
            Arc::make_mut(&mut self.inner).push((name, limit));
        }
        self
    }

    /// Returns a limit value by the name.
    pub fn get<S>(&self, name: S) -> Option<u64>
    where
        S: AsRef<str>,
    {
        self.inner
            .binary_search_by_key(&name.as_ref(), |&(a, _)| a)
            .map(|i| self.inner[i].1)
            .ok()
    }
}

#[async_trait]
impl FromRequest for Limits {
    type Error = Infallible;

    async fn extract(req: &mut Request) -> Result<Self, Self::Error> {
        Ok(req.limits().clone())
    }
}
