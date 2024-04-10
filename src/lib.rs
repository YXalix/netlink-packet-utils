// SPDX-License-Identifier: MIT
#![no_std]

#![feature(ip_in_core)]
pub extern crate byteorder;
pub extern crate paste;
extern crate alloc;

#[macro_use]
mod macros;

pub mod errors;
pub use self::errors::{DecodeError, EncodeError};

pub mod parsers;

pub mod traits;
pub use self::traits::*;

pub mod nla;
