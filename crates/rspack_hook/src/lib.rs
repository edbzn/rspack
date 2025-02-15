#![feature(slice_group_by)]

mod async_parallel;
mod async_series;
mod async_series_bail;
mod interceptor;
mod sync_series;

pub use async_parallel::{AsyncParallel, AsyncParallel3, AsyncParallel3Hook, AsyncParallelHook};
pub use async_series::{
  AsyncSeries, AsyncSeries2, AsyncSeries2Hook, AsyncSeries3, AsyncSeries3Hook, AsyncSeriesHook,
};
pub use async_series_bail::{
  AsyncSeriesBail, AsyncSeriesBail2, AsyncSeriesBail2Hook, AsyncSeriesBail3, AsyncSeriesBail3Hook,
  AsyncSeriesBail4, AsyncSeriesBail4Hook, AsyncSeriesBailHook,
};
pub use interceptor::{Hook, Interceptor};
pub use rspack_macros::{define_hook, plugin, plugin_hook};
pub use sync_series::{
  SyncSeries, SyncSeries3, SyncSeries3Hook, SyncSeries4, SyncSeries4Hook, SyncSeriesHook,
};

// pub trait Plugin<HookContainer> {
//   fn apply(&self, hook_container: &mut HookContainer);
// }

#[doc(hidden)]
pub mod __macro_helper {
  pub use async_trait::async_trait;
  pub use futures_concurrency;
  pub use rspack_error::Result;
  pub use rustc_hash::FxHashSet;
}
