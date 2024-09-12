#![no_main]
use libfuzzer_sys::fuzz_target;

#[macro_use]
extern crate litcrypt;

use_litcrypt!();

fuzz_target!(|value: (&[u8], &[u8])| {
    let enc = litcrypt_internal::xor(value.0, value.1);
    let dec = litcrypt_internal::xor(&enc, value.1);
    assert_eq!(dec, value.0);
});
