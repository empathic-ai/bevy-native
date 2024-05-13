mod android;

#[cfg(all(target_os = "linux"))]
mod linux;
#[cfg(all(target_os = "linux"))]
pub use linux::*;

#[cfg(all(target_os = "macos"))]
mod ios;
#[cfg(all(target_os = "macos"))]
pub use ios::*;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(all(target_arch = "wasm32"))]
mod web;
#[cfg(all(target_arch = "wasm32"))]
pub use web::*;