use std::cell::Cell;
use std::mem;

thread_local!( static HOOK_LOCK: Cell< i32 > = Cell::new( 0 ) );

struct HookLockGuard( i32 );

impl HookLockGuard {
    #[inline]
    fn increment() -> HookLockGuard {
        HookLockGuard( HOOK_LOCK.with( |cell| {
            let previous = cell.get();
            cell.set( previous + 1 );
            previous
        }))
    }

    #[inline]
    fn zero() -> HookLockGuard {
        HookLockGuard( HOOK_LOCK.with( |cell| {
            let previous = cell.get();
            cell.set( 0 );
            previous
        }))
    }
}

impl Drop for HookLockGuard {
    #[inline]
    fn drop( &mut self ) {
        HOOK_LOCK.with( |cell| cell.set( self.0 ) );
    }
}

#[inline]
pub fn disable_hooks< R, F: FnOnce() -> R >( callback: F ) -> R {
    let lock = HookLockGuard::increment();
    let result = callback();
    mem::drop( lock );

    result
}

#[inline]
pub fn enable_hooks< R, F: FnOnce() -> R >( callback: F ) -> R {
    let lock = HookLockGuard::zero();
    let result = callback();
    mem::drop( lock );

    result
}

#[inline]
pub fn are_hooks_enabled() -> bool {
    HOOK_LOCK.with( |cell| cell.get() <= 0 )
}

#[inline]
pub fn are_hooks_disabled() -> bool {
    !are_hooks_enabled()
}
