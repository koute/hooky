#[doc(hidden)]
#[macro_export]
macro_rules! __define_hook {
    (
        @define_hook $initializer:block
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block
    ) => {
        impl real {
            #[inline]
            #[allow(dead_code)]
            #[allow(non_snake_case)]
            pub unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type {
                use std::{mem, ptr};

                #[cfg(not(feature = "use_parking_lot"))]
                use std::sync::{Once, ONCE_INIT};

                #[cfg(feature = "use_parking_lot")]
                use $crate::parking_lot::{Once, ONCE_INIT};

                static INIT: Once = ONCE_INIT;
                INIT.call_once( || {
                    *raw::$name() = $initializer;
                });

                let pointer: *mut $crate::private::c_void = *raw::$name();
                assert!( pointer != ptr::null_mut() );
                assert!( pointer != $name as *mut $crate::private::c_void );
                let ptr: extern "C" fn( $( $arg_type ),* ) -> $return_type = mem::transmute( pointer );
                ( ptr )( $( $arg_name ),* )
            }
        }

        impl raw {
            #[inline(always)]
            #[allow(dead_code)]
            #[allow(non_snake_case)]
            pub unsafe fn $name() -> *mut *mut $crate::private::c_void {
                static mut POINTER: *mut $crate::private::c_void = 0 as *mut $crate::private::c_void;
                (&mut POINTER) as *mut *mut $crate::private::c_void
            }
        }

        #[no_mangle]
        #[deny(private_no_mangle_fns)]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $name( $( $arg_name : $arg_type ),* ) -> $return_type {
            #[inline(always)]
            #[warn(dead_code)]
            fn internal( $( $arg_name : $arg_type ),* ) -> $return_type {
                $body
            }

            let name = stringify!( $name );
            if $crate::are_hooks_disabled() && name != "dlsym" && name != "dlvsym" && name != "__libc_dlsym" {
                return real::$name( $( $arg_name ),* );
            }

            internal( $( $arg_name ),* )
        }
    };

    (
        @name_list [$($name_list:ident)*]
        #[initializer($initializer:path)]
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block $($rest:tt)*
    ) => {
        __define_hook! {
            @define_hook {
                $initializer()
            }
            unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type $body
        }

        __define_hook! {
            @name_list [$name $($name_list)*]
            $($rest)*
        }
    };

    (
        @name_list [$($name_list:ident)*]
        #[source($so_name:expr)]
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block $($rest:tt)*
    ) => {
        __define_hook! {
            @define_hook {
                use std::ffi::CStr;

                let symbol_cstr = CStr::from_bytes_with_nul_unchecked( concat!( stringify!( $name ), "\0" ).as_bytes() );
                let so_cstr = CStr::from_bytes_with_nul_unchecked( concat!( $so_name, "\0" ).as_bytes() );
                let mut so_handle = libc::dlopen( so_cstr.as_ptr(), libc::RTLD_GLOBAL | libc::RTLD_NOW | libc::RTLD_NOLOAD );
                if so_handle.is_null() {
                    so_handle = libc::dlopen( so_cstr.as_ptr(), libc::RTLD_GLOBAL | libc::RTLD_NOW );
                }
                if so_handle.is_null() {
                    panic!( concat!( "Cannot open ", $so_name ) );
                }
                let handle = real::dlsym( so_handle, symbol_cstr.as_ptr() );
                if handle.is_null() {
                    panic!( concat!( "Symbol ", stringify!( $name ), " not found" ) );
                }
                handle
            }
            unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type $body
        }

        __define_hook! {
            @name_list [$name $($name_list)*]
            $($rest)*
        }
    };

    (
        @name_list [$($name_list:ident)*]
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block $($rest:tt)*
    ) => {
        __define_hook! {
            @define_hook {
                use std::ffi::CStr;

                let symbol_cstr = CStr::from_bytes_with_nul_unchecked( concat!( stringify!( $name ), "\0" ).as_bytes() );
                let handle = real::dlsym( libc::RTLD_NEXT, symbol_cstr.as_ptr() );
                if handle.is_null() {
                    panic!( concat!( "Symbol ", stringify!( $name ), " not found" ) );
                }
                handle
            }
            unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type $body
        }

        __define_hook! {
            @name_list [$name $($name_list)*]
            $($rest)*
        }
    };

    (
        @name_list [$($name:tt)*]
    ) => {
        #[allow(non_camel_case_types)]
        enum real {}

        #[allow(non_camel_case_types)]
        enum raw {}

        #[allow(non_camel_case_types)]
        mod hooky_private {
            pub unsafe fn dlsym_wrapper(
                _handle: *mut $crate::private::c_void,
                symbol: *const $crate::private::c_char,
                _version: *const $crate::private::c_char
            ) -> Option< *mut $crate::private::c_void > {
                use std::ffi::CStr;

                if symbol.is_null() {
                    return None;
                }

                let symbol = CStr::from_ptr( symbol );
                let symbol = symbol.to_bytes();
                $(
                    if symbol == stringify!( $name ).as_bytes() {
                        return Some( super::$name as *mut $crate::private::c_void );
                    }
                )*

                None
            }
        }

        #[allow(dead_code)]
        #[inline]
        fn hooky_get_symbol( symbol: &::std::ffi::CStr ) -> Option< *mut $crate::private::c_void > {
            use std::ptr;
            unsafe {
                hooky_private::dlsym_wrapper( ptr::null_mut(), symbol.as_ptr(), ptr::null() )
            }
        }

        // We don't use any symbols from libdl.so directly
        // so there is no guarantee that the dynamic linker
        // will actually load it if the application we're
        // attaching to doesn't use libdl.so itself.
        //
        // This will force the dynamic linker to load libdl.so
        // even if it'll seem like nothing uses it.
        //
        // Unfortunately this works only in the crate which will
        // get compiled into a .so file, so we need to put this
        // here in this macro.
        #[link_args = "-Wl,--no-as-needed -ldl -Wl,--as-needed"]
        extern {}
    };
}

#[macro_export]
macro_rules! define_hook {
    (
        $($rest:tt)*
    ) => {
        __define_hook! {
            @name_list []

            #[initializer($crate::private::initialize_dlsym)]
            unsafe fn dlsym( handle: *mut $crate::private::c_void, symbol: *const $crate::private::c_char ) -> *mut $crate::private::c_void {
                use std::ptr;
                unsafe {
                    hooky_private::dlsym_wrapper( handle, symbol, ptr::null() ).unwrap_or_else( || real::dlsym( handle, symbol ) )
                }
            }

            #[initializer($crate::private::initialize_dlvsym)]
            unsafe fn dlvsym( handle: *mut $crate::private::c_void, symbol: *const $crate::private::c_char, version: *const $crate::private::c_char ) -> *mut $crate::private::c_void {
                unsafe {
                    hooky_private::dlsym_wrapper( handle, symbol, version ).unwrap_or_else( || real::dlvsym( handle, symbol, version ) )
                }
            }

            #[initializer($crate::private::initialize_libc_dlsym)]
            unsafe fn __libc_dlsym( handle: *mut $crate::private::c_void, symbol: *const $crate::private::c_char ) -> *mut $crate::private::c_void {
                use std::ptr;
                unsafe {
                    hooky_private::dlsym_wrapper( handle, symbol, ptr::null() ).unwrap_or_else( || real::__libc_dlsym( handle, symbol ) )
                }
            }

            $($rest)*
        }
    }
}

#[macro_export]
macro_rules! define_initializer {
    (fn $name:ident() $body:block) => (
        extern "C" fn $name() $body

        #[link_section=".init_array"]
        pub static INITIALIZER: extern fn() = $name;
    )
}
