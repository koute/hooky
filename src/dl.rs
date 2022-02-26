use crate::elf::ObjectInfo;
use std::ops::ControlFlow;
use libc::c_void;

pub unsafe fn initialize_dlsym() -> *mut c_void {
    ObjectInfo::each( |info| {
        if info.name_contains( "libdl.so" ) || info.name_contains( "libc.so" ) {
            if let Some( pointer ) = info.dlsym( "dlsym" ) {
                return ControlFlow::Break( pointer )
            }
        }

        ControlFlow::Continue(())
    }).unwrap_or( std::ptr::null_mut() )
}

pub unsafe fn initialize_dlvsym() -> *mut c_void {
    ObjectInfo::each( |info| {
        if info.name_contains( "libdl.so" ) || info.name_contains( "libc.so" ) {
            if let Some( pointer ) = info.dlsym( "dlvsym" ) {
                return ControlFlow::Break( pointer )
            }
        }

        ControlFlow::Continue(())
    }).unwrap_or( std::ptr::null_mut() )
}

pub unsafe fn initialize_libc_dlsym() -> *mut c_void {
    ObjectInfo::each( |info| {
        if info.name_contains( "libc.so" ) {
            if let Some( pointer ) = info.dlsym( "__libc_dlsym" ) {
                return ControlFlow::Break( pointer )
            }
        }

        ControlFlow::Continue(())
    }).unwrap_or( std::ptr::null_mut() )
}
