#![allow(non_camel_case_types)]
extern crate libc;
use libc::{c_void, size_t, c_char, c_int, c_uint};


pub enum xtc_handle {}
pub type iptc_handle = xtc_handle;
pub type xt_chainlabel = *const i8;
pub type ipt_chainlabel = xt_chainlabel;

const IPTC_LABEL_ACCEPT: xt_chainlabel = b"ACCEPT\0" as *const u8 as *const i8;
const IPTC_LABEL_DROP: xt_chainlabel = b"DROP\0" as *const u8 as *const i8;
const IPTC_LABEL_QUEUE: xt_chainlabel = b"QUEUE\0" as *const u8 as *const i8;
const IPTC_LABEL_RETURN: xt_chainlabel = b"RETURN\0" as *const u8 as *const i8;

const IFNAMSIZ: usize = 16;
const RS_CHAINLABEL_MAX_SIZE: usize = 32;

pub type in_addr_t = u32;

#[repr(C)]
pub struct in_addr
{
    s_addr: in_addr_t,
}

#[repr(C)]
pub struct ipt_ip {
    src: in_addr,
    dst: in_addr,
    smsk: in_addr,
    dmsk: in_addr,
    iniface: [i8; IFNAMSIZ],
    outiface: [i8; IFNAMSIZ],
    iniface_mask: [u8; IFNAMSIZ],
    outiface_mask: [u8; IFNAMSIZ],
    proto: u16,
    flags: u8,
    invflags: u8,
}

#[repr(C)]
pub struct xt_counters {
    pcnt: u64,
    bcnt: u64,
}

#[repr(C)]
pub struct ipt_entry {
    ip: ipt_ip,
    nfcache: c_uint,
    target_offset: u16,
    next_offset: u16,
    comefrom: c_uint,
    counters: xt_counters,
    elems: [u8; 0],
}

extern "C" {
    pub fn iptc_is_chain(chain: *const c_char, handle: *mut xtc_handle) -> i32;
    pub fn iptc_init(tablename: *const c_char) -> *mut xtc_handle;
    pub fn iptc_free(handle: *mut xtc_handle);
    pub fn iptc_first_chain(handle: *mut xtc_handle) -> *const c_char;
    pub fn iptc_next_chain(handle: *mut xtc_handle) -> *const c_char;
    pub fn iptc_first_rule(chain: *const c_char, handle: *mut xtc_handle) -> *const ipt_entry;
    pub fn iptc_next_rule(prev: *const ipt_entry, handle: *mut xtc_handle ) -> *const ipt_entry;
    pub fn iptc_get_target(e: *const ipt_entry, handle: *mut xtc_handle) -> *const c_char;
    pub fn iptc_builtin(chain: *const c_char, handle: *const xtc_handle) -> c_int;
    pub fn iptc_get_policy(chain: *const c_char, counter: *mut xt_counters, handle: *mut xtc_handle) -> *const c_char;
    pub fn iptc_insert_entry(chain: xt_chainlabel, e: *const ipt_entry, rulenum: c_uint, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_replace_entry(chain: xt_chainlabel, e: *const ipt_entry, rulenum: c_uint, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_append_entry(chain: xt_chainlabel, e: *const ipt_entry, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_check_entry(chain: xt_chainlabel, origfw: *const ipt_entry, matchmask: *mut u8, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_delete_entry(chain: xt_chainlabel, origfw: *const ipt_entry, matchmask: *mut u8, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_delete_num_entry(chain: xt_chainlabel, rulenum: c_uint, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_check_packet(chain: xt_chainlabel, entry: *mut ipt_entry, handle: *mut xtc_handle) -> *const c_char;
    pub fn iptc_flush_entries(chain: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_zero_entries(chain: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_create_chain(chain: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_delete_chain(chain: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_rename_chain(oldname: xt_chainlabel, newname: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_set_policy(chain: xt_chainlabel, policy: xt_chainlabel, counters: *mut xt_counters, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_get_references(references: *mut c_uint, chain: xt_chainlabel, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_read_counter(chain: xt_chainlabel, rulenum: c_uint, handle: *mut xtc_handle) -> *mut xt_counters;
    pub fn iptc_zero_counter(chain: xt_chainlabel, rulenum: c_uint, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_set_counter(chain: xt_chainlabel, rulenum: c_uint, counters: *mut xt_counters, handle: *mut xtc_handle) -> c_int;
    pub fn iptc_commit(handle: *mut xtc_handle) -> c_int;
    pub fn iptc_get_raw_socket() -> c_int;
    pub fn iptc_strerror(err: c_int) -> *const c_char;
    pub fn dump_entries(handle: *mut xtc_handle);
}

