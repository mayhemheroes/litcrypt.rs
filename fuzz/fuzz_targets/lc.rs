#![no_main]
use libfuzzer_sys::fuzz_target;

#[macro_use]
extern crate litcrypt;

use_litcrypt!();

fuzz_target!(|value: (&[u8], &[u8])| {
    litcrypt_internal::xor(value.0, value.1);
});
