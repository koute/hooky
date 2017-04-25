#![feature(link_args)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate libc;

#[cfg(feature = "use_parking_lot")]
extern crate parking_lot;

#[macro_use]
mod macros;

mod common;
mod dl;

pub use common::{
    are_hooks_enabled,
    are_hooks_disabled,
    disable_hooks,
    enable_hooks
};

#[doc(hidden)]
pub mod private {
    pub use libc::{
        c_void,
        c_char
    };

    pub use dl::{
        initialize_dlsym,
        initialize_dlvsym,
        initialize_libc_dlsym
    };
}
