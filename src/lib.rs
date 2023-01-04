#![deny(clippy::all)]

use std::ffi::c_void;

use windows::Win32::Foundation::{HANDLE, CloseHandle};
use windows::Win32::System::Threading::{OpenProcessToken, GetCurrentProcess};
use windows::Win32::Security::{TOKEN_QUERY, TOKEN_ELEVATION, TokenElevation, GetTokenInformation};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn is_elevated() -> bool {
  let mut is_elevated = false;
  unsafe {
    // Based on http://stackoverflow.com/a/8196291
    let mut token: HANDLE = HANDLE::default();
    OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token);
    let mut len = 0;
    let mut elevation = TOKEN_ELEVATION::default();
    let ptr = ((&mut elevation) as *mut TOKEN_ELEVATION) as *mut c_void;
    if GetTokenInformation(token, TokenElevation, Some(ptr), std::mem::size_of_val(&elevation) as u32, &mut len).as_bool()  {
      is_elevated = elevation.TokenIsElevated > 0;
    }

    if token.ne(&HANDLE::default()) {
      CloseHandle(token);
    }
  }

  is_elevated
}
