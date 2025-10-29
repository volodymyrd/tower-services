//! Limit the max number of requests being concurrently processed.

pub mod future;
mod layer;
mod service;

#[cfg(test)]
mod test;

#[cfg(test)]
#[path = "../../../test-utils/src/support.rs"]
pub(crate) mod support;
