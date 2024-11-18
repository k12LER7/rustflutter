#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.80.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_get_a_number_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, _, i8>(
        WrapInfo {
            debug_name: "get_a_number",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_a_number()),
    )
}
fn wire_get_string_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, _, String>(
        WrapInfo {
            debug_name: "get_string",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_string()),
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
