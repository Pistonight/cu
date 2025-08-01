//! Batteries-included common utils
//!
//! # Basic Guide
//! ```rust,ignore
//! // prelude import should be used to automatically bring traits into scope
//! use cu::prelude::*;
//!
//! // types, macros, and most functions should just use `cu::` prefix
//! // to avoid bloating the imports
//! fn main() -> std::process::ExitCode {
//!     // cli_wrapper will set up everything for you.
//!     // All you need is a type that implements clap::Parser
//!     cu::cli_wrapper(main_internal)
//! }
//!
//! #[derive(clap::Parser)]
//! struct MyArgs {
//!     #[clap(flatten)]
//!     common: cu::CliFlags
//! }
//! impl AsRef<cu::CliFlags> for MyArgs {
//!     fn as_ref(&self) -> cu::CliFlags {
//!         &self.common
//!     }
//! }
//!
//! fn main_internal(args: MyArgs) -> cu::Result<()> {
//!     cu::info!("hello");
//!     // some functions are bundled in modules, use `cu::<module>::` prefix
//!     cu::fs::read("x/y/z").context("failed to read that file")?;
//!     Ok(())
//! }
//! ```
//!
//! # Printing
//! `cu::CliFlags` contains the following options for print level control:
//! - `--verbose/-v` to increase verbose level.
//! - `--quiet/-q` to decrease verbose level.
//! - `--color` to set color mode
//!
//! `-v` and `-q` can be specified multiple times, `-vv` is the max verbose level, `-qq` is the max
//! quietness level. They also cancel each other out `-vvvq` is the same as `-vv`.
//!
//! The following table shows what are printed for each level
//! |         | `-qq` | `-q` | Normal | `-v` | `-vv` |
//! |-|-      |-     |-       |-     |-      |
//! | [`error`] | ❌ | ✅ | ✅ | ✅ | ✅ |
//! | [`hint`]  | ❌ | ✅ | ✅ | ✅ | ✅ |
//! | [`print`] | ❌ | ✅ | ✅ | ✅ | ✅ |
//! | [`warn`]  | ❌ | ❌ | ✅ | ✅ | ✅ |
//! | [`info`]  | ❌ | ❌ | ✅ | ✅ | ✅ |
//! | [`debug`] | ❌ | ❌ | ❌ | ✅ | ✅ |
//! | [`trace`] | ❌ | ❌ | ❌ | ❌ | ✅ |
//!
//! [`set_thread_print_name`] can be used to add a prefix to all messages printed
//! by that thread.
//!
//! Messages that are too long and multi-line messages are automatically wrapped.
//!
//! # Progress Bar
//! Animated progress bars are displayed at the bottom of the terminal.
//! While progress bars are visible, printing still works and will be put
//! above the bars. However, prints will be buffered and refreshed
//! and the same frame rate as the bars.
//!
//! [`progress_bar`] and [`progress_bar_lowp`] are used to create a bar.
//! The only difference is that `lowp` doesn't print a message when the progress
//! is done (as if the bar was never there). The bar takes a message to indicate
//! the current action, and each update call can accept a message to indicate
//! the current step. When `bar` is dropped, it will print a done message.
//!
//! ```rust,no_run
//! {
//!    let bar = cu::progress_bar(10, "This takes 2.5 seconds");
//!    for i in 0..10 {
//!        cu::progress!(bar, i, "step {i}");
//!        cu::debug!("this is debug message\n");
//!        std::thread::sleep(Duration::from_millis(250));
//!    }
//! }
//! ```
//!
//! # Prompting
//! With the `prompt` feature, additional CLI args are available:
//! - `--yes/-y`: Automatically answer `y` to all yes/no prompts (regular prompts are still shown)
//! - `--non-interactive`: Disallow prompts, prompts will fail with an error instead
//! - `--interactive`: This is the default, and cancels the effect of one `--non-interactive`
//!
//! Prompts are always shown regardless of verbosity. But when stdout is redirected,
//! they will not render in terminal.
//! Use [`prompt`] and [`yesno`] to show prompts. The prompts are thread-safe. Meaning
//! You can call them from multiple threads, and they will be queued to prompt the user one after
//! the other.

// anyhow re-exports
pub use anyhow::{Context, Result, bail, ensure};

// mod env_var;
// pub use env_var::*;
// mod parse;
// pub use parse::*;

mod clap;
pub use clap::{CliFlags, cli_wrapper};

pub use log::{debug, error, info, trace, warn};

/// File System utils
pub mod fs;

/// Path utils
mod path;
pub use path::PathExtension;

/// Printing utils
mod print;

pub use print::{
    ColorLevel, PrintLevel, ProgressBarHandle, PromptLevel, color_enabled, init_print_options,
    progress_bar, progress_bar_lowp, set_thread_print_name,
};

#[doc(hidden)]
pub mod __priv {
    pub use super::print::{__PrintType, __print_with_type, __prompt, __prompt_yesno};
}

/// Prelude imports
pub mod prelude {
    pub use crate::Context as _;
    pub use crate::PathExtension as _;
}
