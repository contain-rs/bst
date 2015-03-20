// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An ordered map and set based on a binary search tree.

#![feature(box_patterns, box_syntax)]
#![feature(core)]
#![feature(unboxed_closures)]

#![cfg_attr(test, feature(hash, test))]

extern crate compare;

#[cfg(feature = "ordered_iter")] extern crate ordered_iter;

#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate test;
#[cfg(test)] #[macro_use] mod bench;

pub use map::Map;
pub use set::Set;

pub mod map;
pub mod set;
