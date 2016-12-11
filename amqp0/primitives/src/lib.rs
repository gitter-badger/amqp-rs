// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]

#[cfg(not(feature = "amqp0-build-primitives"))]
include!(concat!("../pregen/mod.rs"));
#[cfg(feature = "amqp0-build-primitives")]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub mod field;

use std::io;

pub trait Spec {

}

pub trait Payload {
    fn class_id(&self) -> u16;
    fn method_id(&self) -> u16;
    fn len(&self) -> usize;
    fn write_to<W: io::Write>(&self, &mut W) -> io::Result<()>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

