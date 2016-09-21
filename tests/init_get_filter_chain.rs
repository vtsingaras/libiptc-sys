extern crate libiptc_sys;
extern crate libc;
use std::ffi::{CString, CStr};
use libiptc_sys::{xtc_handle, xt_chainlabel};

//Skip tests if CAP_NET_ADMIN missing
extern crate capabilities;
use capabilities::{Capability, Capabilities, Flag};

fn has_priv() -> bool {
    let caps = Capabilities::from_current_proc().unwrap();
    caps.check(Capability::CAP_NET_ADMIN, Flag::Effective)
}

#[test]
fn can_init() {
    if! has_priv() {
        println!("Test skipped due to missing privileges.");
        return;
    }
    unsafe{
        let h: *mut xtc_handle;
        h = libiptc_sys::iptc_init(CString::new("filter").unwrap().as_ptr());
        assert!(!(h.is_null()));
        libiptc_sys::iptc_free(h);
    }
}

#[test]
fn can_get_chainname() {
    if! has_priv() {
        println!("Test skipped due to missing privileges.");
        return;
    }
    unsafe {
        let h: *mut xtc_handle;
        h = libiptc_sys::iptc_init(CString::new("filter").unwrap().as_ptr());
        let chain: xt_chainlabel = libiptc_sys::iptc_first_chain(h);
        assert!(!(chain.is_null()));
        let rust_chain = CStr::from_ptr(chain).to_string_lossy();
        libiptc_sys::iptc_free(h);
        println!("First chain is: {}", rust_chain);
    }
}