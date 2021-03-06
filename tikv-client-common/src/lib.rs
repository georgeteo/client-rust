#[macro_use]
pub mod util;
pub mod compat;
pub mod config;
pub mod errors;
pub mod kv;
pub mod security;
pub mod stats;
mod timestamp;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prometheus;

#[doc(inline)]
pub use crate::config::Config;
#[doc(inline)]
pub use crate::errors::Error;
#[doc(inline)]
pub use crate::errors::ErrorKind;
#[doc(inline)]
pub use crate::errors::Result;
#[doc(inline)]
pub use crate::kv::{BoundRange, Key, KvPair, ToOwnedRange, Value};
#[doc(inline)]
pub use crate::timestamp::{Timestamp, TimestampExt};
