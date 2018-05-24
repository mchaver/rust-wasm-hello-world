#![feature(custom_attribute)]
#![feature(repr_transparent)]
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn add_one(a: u32) -> u32 {
    a + 1
}

// static SOME_STR: &'static str = "A static string";
static mut HELLO: &'static str = "Hello";

#[no_mangle]
pub unsafe fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[no_mangle]
pub unsafe fn get_hello_len() -> usize {
    HELLO.len()
}

#[no_mangle]
pub unsafe fn set_goodbye() {
    HELLO = "Goodbye";
}


// Very important to use `transparent` to prevent ABI issues 
#[repr(transparent)]
pub struct JsInteropString(*mut String);

impl JsInteropString {
    // Unsafe because we create a string and say it's full of valid
    // UTF-8 data, but it isn't!
    unsafe fn with_capacity(cap: usize) -> Self {
        let mut d = Vec::with_capacity(cap);
        d.set_len(cap);
        let s = Box::new(String::from_utf8_unchecked(d));
        JsInteropString(Box::into_raw(s))
    }

    unsafe fn as_string(&self) -> &String {
        &*self.0
    }

    unsafe fn as_mut_string(&mut self) -> &mut String {
        &mut *self.0
    }

    unsafe fn into_boxed_string(self) -> Box<String> {
        Box::from_raw(self.0)
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_string().as_mut_vec().as_mut_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn stringPrepare(cap: usize) -> JsInteropString {
    JsInteropString::with_capacity(cap)
}

#[no_mangle]
pub unsafe extern "C" fn stringData(mut s: JsInteropString) -> *mut u8 {
    s.as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn stringLen(s: JsInteropString) -> usize {
    s.as_string().len()
}

#[no_mangle]
pub unsafe extern "C" fn compute(s: JsInteropString, n1: i32, n2: i32) -> i32 {
    let s = s.into_boxed_string();
    real_code::compute(&s, n1, n2)
}

mod real_code {
    pub fn compute(operator: &str, n1: i32, n2: i32) -> i32 {
        match operator {
            "SUM"  => n1 + n2,
            "DIFF" => n1 - n2,
            "MULT" => n1 * n2,
            "DIV"  => n1 / n2,
            _ => 0,
        }
    }
}
/*
// A struct with a known memory layout that we can pass string information in
#[repr(C)]
pub struct JsInteropString {
    data: *const u8,
    len: usize,
}

// Our FFI shim function    
#[no_mangle]
pub unsafe extern "C" fn compute(s: *const JsInteropString, n1: i32, n2: i32) -> i32 {
    print!("compute is called");
    // Check for NULL (see corresponding comment in JS)
    let s = match s.as_ref() {
        Some(s) => s,
        None => return -1,
    };

    // Convert the pointer and length to a `&[u8]`.
    let data = std::slice::from_raw_parts(s.data, s.len);

    // Convert the `&[u8]` to a `&str`    
    match std::str::from_utf8(data) {
        Ok(s) => real_code::compute(s, n1, n2),
        Err(_) => -2,
    }
}

// I advocate that you keep your interesting code in a different
// crate for easy development and testing. Have a separate crate
// with the FFI shims.
mod real_code {
    pub fn compute(operator: &str, n1: i32, n2: i32) -> i32 {
        println!("{}", operator);
        match operator {
            "SUM"  => n1 + n2,
            "DIFF" => n1 - n2,
            "MULT" => n1 * n2,
            "DIV"  => n1 / n2,
            _ => 0,
        }
    }
}
*/
