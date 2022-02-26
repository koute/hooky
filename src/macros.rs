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
                const UNINITIALIZED: u8 = 0;
                const INITIALIZING: u8 = 1;
                const PANICKED: u8 = 2;
                const DONE: u8 = 3;

                static INIT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new( UNINITIALIZED );

                #[inline(never)]
                #[cold]
                fn init() {
                    if INIT.compare_exchange( UNINITIALIZED, INITIALIZING, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Acquire ).is_ok() {
                        unsafe {
                            let result = std::panic::catch_unwind(|| {
                                let pointer = $initializer;
                                assert!( pointer != std::ptr::null_mut() );
                                assert!( pointer != $name as *mut $crate::private::c_void );

                                *raw::$name() = pointer;
                            });
                            match result {
                                Ok(()) => {
                                    INIT.store(DONE, std::sync::atomic::Ordering::Release);
                                },
                                Err( error ) => {
                                    INIT.store(PANICKED, std::sync::atomic::Ordering::Release);
                                    std::panic::resume_unwind( error );
                                }
                            }
                        }
                    }

                    loop {
                        let status = INIT.load( std::sync::atomic::Ordering::Acquire );
                        match status {
                            INITIALIZING => std::thread::yield_now(),
                            PANICKED => panic!(),
                            DONE => break,
                            _ => unsafe { std::hint::unreachable_unchecked() }
                        }
                    }
                }

                if INIT.load( std::sync::atomic::Ordering::Acquire ) != DONE {
                    init();
                }

                let pointer: *mut $crate::private::c_void = *raw::$name();
                let ptr: extern "C" fn( $( $arg_type ),* ) -> $return_type = std::mem::transmute( pointer );
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
        $crate::__define_hook! {
            @define_hook {
                $initializer()
            }
            unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type $body
        }

        $crate::__define_hook! {
            @name_list [$name $($name_list)*]
            $($rest)*
        }
    };

    (
        @name_list [$($name_list:ident)*]
        #[source($so_name:expr)]
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block $($rest:tt)*
    ) => {
        $crate::__define_hook! {
            @define_hook {
                let symbol_cstr = std::ffi::CStr::from_bytes_with_nul_unchecked( concat!( stringify!( $name ), "\0" ).as_bytes() );
                let so_cstr = std::ffi::CStr::from_bytes_with_nul_unchecked( concat!( $so_name, "\0" ).as_bytes() );
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

        $crate::__define_hook! {
            @name_list [$name $($name_list)*]
            $($rest)*
        }
    };

    (
        @name_list [$($name_list:ident)*]
        unsafe fn $name:ident( $( $arg_name:ident : $arg_type:ty ),* ) -> $return_type:ty $body:block $($rest:tt)*
    ) => {
        $crate::__define_hook! {
            @define_hook {
                let symbol_cstr = std::ffi::CStr::from_bytes_with_nul_unchecked( concat!( stringify!( $name ), "\0" ).as_bytes() );
                let handle = real::dlsym( libc::RTLD_NEXT, symbol_cstr.as_ptr() );
                if handle.is_null() {
                    panic!( concat!( "Symbol ", stringify!( $name ), " not found" ) );
                }
                handle
            }
            unsafe fn $name( $( $arg_name : $arg_type ),* ) -> $return_type $body
        }

        $crate::__define_hook! {
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
                if symbol.is_null() {
                    return None;
                }

                let symbol = std::ffi::CStr::from_ptr( symbol );
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
            unsafe {
                hooky_private::dlsym_wrapper( std::ptr::null_mut(), symbol.as_ptr(), std::ptr::null() )
            }
        }
    }
}

#[macro_export]
macro_rules! define_hook {
    (
        $($rest:tt)*
    ) => {
        $crate::__define_hook! {
            @name_list []

            #[initializer($crate::private::initialize_dlsym)]
            unsafe fn dlsym( handle: *mut $crate::private::c_void, symbol: *const $crate::private::c_char ) -> *mut $crate::private::c_void {
                unsafe {
                    hooky_private::dlsym_wrapper( handle, symbol, std::ptr::null() ).unwrap_or_else( || real::dlsym( handle, symbol ) )
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
                unsafe {
                    hooky_private::dlsym_wrapper( handle, symbol, std::ptr::null() ).unwrap_or_else( || real::__libc_dlsym( handle, symbol ) )
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
