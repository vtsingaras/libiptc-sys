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
    pub fn iptc_first_rule(chain: *const c_char, handle: *mut xtc_handle) -> *mut ipt_entry;
/*
const struct ipt_entry *iptc_next_rule(const struct ipt_entry *prev,
				       struct xtc_handle *handle);

const char *iptc_get_target(const struct ipt_entry *e,
			    struct xtc_handle *handle);

int iptc_builtin(const char *chain, struct xtc_handle *const handle);

const char *iptc_get_policy(const char *chain,
			    struct xt_counters *counter,
			    struct xtc_handle *handle);



int iptc_insert_entry(const xt_chainlabel chain,
		      const struct ipt_entry *e,
		      unsigned int rulenum,
		      struct xtc_handle *handle);

int iptc_replace_entry(const xt_chainlabel chain,
		       const struct ipt_entry *e,
		       unsigned int rulenum,
		       struct xtc_handle *handle);


int iptc_append_entry(const xt_chainlabel chain,
		      const struct ipt_entry *e,
		      struct xtc_handle *handle);

int iptc_check_entry(const xt_chainlabel chain,
		      const struct ipt_entry *origfw,
		      unsigned char *matchmask,
		      struct xtc_handle *handle);


int iptc_delete_entry(const xt_chainlabel chain,
		      const struct ipt_entry *origfw,
		      unsigned char *matchmask,
		      struct xtc_handle *handle);

int iptc_delete_num_entry(const xt_chainlabel chain,
			  unsigned int rulenum,
			  struct xtc_handle *handle);


const char *iptc_check_packet(const xt_chainlabel chain,
			      struct ipt_entry *entry,
			      struct xtc_handle *handle);

int iptc_flush_entries(const xt_chainlabel chain,
		       struct xtc_handle *handle);

int iptc_zero_entries(const xt_chainlabel chain,
		      struct xtc_handle *handle);

int iptc_create_chain(const xt_chainlabel chain,
		      struct xtc_handle *handle);

int iptc_delete_chain(const xt_chainlabel chain,
		      struct xtc_handle *handle);

int iptc_rename_chain(const xt_chainlabel oldname,
		      const xt_chainlabel newname,
		      struct xtc_handle *handle);

int iptc_set_policy(const xt_chainlabel chain,
		    const xt_chainlabel policy,
		    struct xt_counters *counters,
		    struct xtc_handle *handle);

int iptc_get_references(unsigned int *ref,
			const xt_chainlabel chain,
			struct xtc_handle *handle);

struct xt_counters *iptc_read_counter(const xt_chainlabel chain,
				       unsigned int rulenum,
				       struct xtc_handle *handle);

int iptc_zero_counter(const xt_chainlabel chain,
		      unsigned int rulenum,
		      struct xtc_handle *handle);

int iptc_set_counter(const xt_chainlabel chain,
		     unsigned int rulenum,
		     struct xt_counters *counters,
		     struct xtc_handle *handle);

int iptc_commit(struct xtc_handle *handle);

int iptc_get_raw_socket(void);

const char *iptc_strerror(int err);

extern void dump_entries(struct xtc_handle *const);

extern const struct xtc_ops iptc_ops;
*/
}