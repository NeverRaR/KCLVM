// Copyright 2021 The KCL Authors. All rights reserved.

extern crate chrono;

use chrono::prelude::Local;

use crate::*;

// today() -> str:

#[no_mangle]
#[runtime_fn]
pub extern "C" fn kclvm_datetime_today(
    _ctx: *mut kclvm_context_t,
    _args: *const kclvm_value_ref_t,
    _kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let s = Local::now().to_string();
    return ValueRef::str(s.as_ref()).into_raw();
}

// now() -> str:

#[no_mangle]
#[runtime_fn]
pub extern "C" fn kclvm_datetime_now(
    _ctx: *mut kclvm_context_t,
    _args: *const kclvm_value_ref_t,
    _kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let s = Local::now().to_string();
    return ValueRef::str(s.as_ref()).into_raw();
}

// ticks() -> float:

#[no_mangle]
#[runtime_fn]
pub extern "C" fn kclvm_datetime_ticks(
    _ctx: *mut kclvm_context_t,
    _args: *const kclvm_value_ref_t,
    _kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let x = Local::now().timestamp();
    ValueRef::float(x as f64).into_raw()
}

// date() -> str:

#[no_mangle]
#[runtime_fn]
pub extern "C" fn kclvm_datetime_date(
    _ctx: *mut kclvm_context_t,
    _args: *const kclvm_value_ref_t,
    _kwargs: *const kclvm_value_ref_t,
) -> *const kclvm_value_ref_t {
    let s = Local::now().to_string();
    return ValueRef::str(s.as_ref()).into_raw();
}
