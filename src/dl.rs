use libc::{
    c_void,
    c_char
};

extern {
    fn hooky_load_symbol( library_name: *const c_char, symbol: *const c_char ) -> *mut c_void;
}

pub unsafe fn initialize_dlsym() -> *mut c_void {
    hooky_load_symbol( b"*libdl.so*\0".as_ptr() as *const c_char, b"dlsym\0".as_ptr() as *const c_char )
}

pub unsafe fn initialize_dlvsym() -> *mut c_void {
    hooky_load_symbol( b"*libdl.so*\0".as_ptr() as *const c_char, b"dlvsym\0".as_ptr() as *const c_char )
}

pub unsafe fn initialize_libc_dlsym() -> *mut c_void {
    hooky_load_symbol( b"*libc.so*\0".as_ptr() as *const c_char, b"__libc_dlsym\0".as_ptr() as *const c_char )
}
