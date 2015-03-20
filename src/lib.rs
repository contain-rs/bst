// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Maps are collections of unique keys with corresponding values, and sets are
//! just unique keys without a corresponding value.
//!
//! This crate defines the `TreeMap` and `TreeSet` types. Their keys must implement `Ord`.
//!
//! `TreeMap`s are ordered.
//!
//! # Examples
//!
//! ```
//! use bst::TreeSet;
//!
//! let mut tree_set = TreeSet::new();
//!
//! tree_set.insert(2);
//! tree_set.insert(1);
//! tree_set.insert(3);
//!
//! for i in tree_set.iter() {
//!    println!("{}", i) // prints 1, then 2, then 3
//! }
//! ```

#![feature(box_patterns, box_syntax)]
#![feature(core)]
#![feature(unboxed_closures)]

#![cfg_attr(test, feature(hash, test))]

extern crate compare;

#[cfg(feature = "ordered_iter")] extern crate ordered_iter;

#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate test;
#[cfg(test)] #[macro_use] mod bench;

pub use map::TreeMap;
pub use set::TreeSet;

pub mod map;
pub mod set;
