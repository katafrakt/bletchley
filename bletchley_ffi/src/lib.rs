extern crate libc;
extern crate bletchley_lib;

use std::ffi::{CStr};
use std::os::raw::c_char;
use bletchley_lib::{crypto,ble};

#[no_mangle]
pub extern "C" fn encrypt(key: *const c_char, input: *const c_char) -> i32 {
    unsafe {
        let c_key = CStr::from_ptr(key).to_str().unwrap();
        let c_input = CStr::from_ptr(input).to_str().unwrap();

        let crypt_result = crypto::encryptor::with_file_key(c_key, c_input);
        ble::format::create_file(crypt_result, None);
    }
    return 0;
}
