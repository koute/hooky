extern crate libc;

#[macro_use]
extern crate hooky;

#[cfg(not(test))]
define_hook! {
    unsafe fn clock_gettime( id: libc::clockid_t, ts: *mut libc::timespec ) -> libc::c_int {
        unsafe {
            let result = real::clock_gettime( id, ts );
            if id == libc::CLOCK_REALTIME {
                (*ts).tv_sec -= 24 * 60 * 60;
            }

            result
        }
    }

    unsafe fn gettimeofday( tv: *mut libc::timeval, tz: *mut libc::c_void ) -> libc::c_int {
        unsafe {
            let result = real::gettimeofday( tv, tz );
            (*tv).tv_sec -= 24 * 60 * 60;

            result
        }
    }
}
