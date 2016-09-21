# libiptc-sys [![Build Status](https://travis-ci.org/vtsingaras/libiptc-sys.svg)](https://travis-ci.org/vtsingaras/libiptc-sys)
Rust [ffi](https://doc.rust-lang.org/book/ffi.html) bindings to libiptc.

### Developer dependencies
Requires the libiptc development package and pkg-config on the system.  
For the tests it requires the libcap development package as it skips some integration tests if the necessary privileges are not present.  
#### Fedora/CentOS/etc.
```bash
sudo dnf install iptables-devel libcap-devel
```
#### Ubuntu/Debian/etc.
```bash
sudo aptitude install iptables-dev libcap-dev
```
