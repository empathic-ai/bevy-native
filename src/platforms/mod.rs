mod android;
mod ios;
mod linux;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(all(target_arch = "wasm32"))]
mod web;
#[cfg(all(target_arch = "wasm32"))]
pub use web::*;