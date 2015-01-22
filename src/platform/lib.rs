#![crate_name = "platform"]
#![crate_type = "rlib"]
#![no_std]

#![allow(unstable)]
#![feature(plugin)]

extern crate core;
extern crate hil;
#[plugin] #[no_link] extern crate plugins;

macro_rules! volatile {
    ($item:expr) => ({
        use core::intrinsics::volatile_load;
        unsafe { volatile_load(&$item) }
    });

    ($item:ident = $value:expr) => ({
        use core::intrinsics::volatile_store;
        unsafe { volatile_store(&mut $item, $value); }
    });
}


mod std {
    pub use core::*;
}

pub mod sam4l;
