extern crate libiptc_sys;
extern crate libc;
use std::ffi::{CString, CStr};
use libiptc_sys::{xtc_handle, xt_chainlabel};

#[test]
fn can_init() {
    unsafe{
        let h: *mut xtc_handle;
        h = libiptc_sys::iptc_init(CString::new("filter").unwrap().as_ptr());
    }
}

#[test]
fn can_get_chainname() {
    unsafe {
        let h: *mut xtc_handle;
        h = libiptc_sys::iptc_init(CString::new("filter").unwrap().as_ptr());
        let chain: xt_chainlabel = libiptc_sys::iptc_first_chain(h);
        let rust_chain = CStr::from_ptr(chain).to_string_lossy();
        println!("First chain is: {}", rust_chain);
    }
}