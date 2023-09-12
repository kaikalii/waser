#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(not(any(feature = "server", feature = "client", debug_assertions)))]
compile_error!("Either the 'server' or 'client' feature must be enabled");
