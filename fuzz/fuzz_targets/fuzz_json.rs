// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
#![no_main]
use libfuzzer_sys::fuzz_target;
use rlottie_core::loader::json;

fuzz_target!(|data: &[u8]| {
    let _ = json::from_slice(data);
});
