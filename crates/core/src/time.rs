//! The clock, in one place: `std::time` where the system has one, and in the browser `web-time`, whose
//! `Instant::now` and `SystemTime::now` read `Performance.now()` and `Date.now()` — `std`'s own panic on
//! wasm32-unknown-unknown ("time not implemented on this platform"). `web-time` re-exports `std::time`
//! outside the browser, so both worlds see the same types.

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Nanoseconds since the epoch, for ids and stamps. Where the clock is not callable, 0: the other halves
/// of the mix (a count, the process) still tell the values apart.
pub fn epoch_nanos() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos() as u64)
}

/// Seconds since the epoch: the timestamp an IFC header carries.
pub fn epoch_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}
