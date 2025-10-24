use core::task::{RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
unsafe fn clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}
unsafe fn wake(_: *const ()) {}
unsafe fn wake_by_ref(_: *const ()) {}
unsafe fn drop(_: *const ()) {}

pub fn make(data: *const ()) -> Waker {
    unsafe { Waker::new(data, &VTABLE) }
}
