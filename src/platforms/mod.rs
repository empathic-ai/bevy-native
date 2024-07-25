#[cfg(all(target_os = "android"))]
mod android;
#[cfg(all(target_os = "android"))]
pub use android::*;

#[cfg(all(target_os = "linux"))]
mod linux;
#[cfg(all(target_os = "linux"))]
pub use linux::*;

#[cfg(all(target_os = "ios"))]
mod ios;
#[cfg(all(target_os = "ios"))]
pub use ios::*;

#[cfg(all(target_os = "macos"))]
mod macos;
#[cfg(all(target_os = "macos"))]
pub use macos::*;

#[cfg(all(target_os = "windows"))]
mod windows;
#[cfg(all(target_os = "windows"))]
pub use windows::*;

#[cfg(all(target_arch = "wasm32"))]
mod web;
#[cfg(all(target_arch = "wasm32"))]
pub use web::*;