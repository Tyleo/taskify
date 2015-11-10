use alloc::heap::deallocate;
use std::marker::Unsize;
use std::mem::{ align_of_val, size_of_val };
use std::ops::CoerceUnsized;
use std::ptr::{ read, Shared };
use std::sync::atomic::{ AtomicUsize, Ordering };

pub struct DecayPtr<T: ?Sized> {
    _ptr: Shared<DecayInner<T>>,
}

struct DecayInner<T: ?Sized> {
    decay_count: AtomicUsize,
    data: T,
}

unsafe impl<T: ?Sized + Sync + Send> Send for DecayPtr<T> { }
unsafe impl<T: ?Sized + Sync + Send> Sync for DecayPtr<T> { }
impl <T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<DecayPtr<U>> for DecayPtr<T> { }

unsafe impl<T: ?Sized + Sync + Send> Send for DecayInner<T> { }
unsafe impl<T: ?Sized + Sync + Send> Sync for DecayInner<T> { }

impl<T> DecayPtr<T> {
    pub unsafe fn new(data: T) -> DecayPtr<T> {
        let inner = Box::new(DecayInner {
            decay_count: AtomicUsize::new(0),
            data: data,
        });
        DecayPtr { _ptr: unsafe { Shared::new(Box::into_raw(inner)) } }
    }

    pub unsafe fn decay(self) -> Option<T> {
        let should_release = self.decay_decrement();

        if should_release {
            Some(self.decay_release())
        }
        else {
            None
        }
    }

    fn decay_decrement(&self) -> bool {
        let inner = self.inner_ref();
        inner.decay_count.fetch_sub(1, Ordering::SeqCst) == 1
    }

    fn decay_release(self) -> T {
        let inner = self.inner_raw();
        let result = unsafe { read(inner) };
        unsafe { deallocate(inner as *mut u8,
                            size_of_val(&*inner),
                            align_of_val(&*inner)) };

        result.data
    }
}

impl<T: ?Sized> DecayPtr<T> {
    fn inner_ref(&self) -> &DecayInner<T> {
        unsafe { &**self._ptr }
    }

    fn inner_raw(self) -> *mut DecayInner<T> {
        unsafe { *self._ptr }
    }

    pub unsafe fn clone(&self) ->DecayPtr<T> {
        self.inner_ref().decay_count.fetch_add(1, Ordering::Relaxed);
        DecayPtr { _ptr: self._ptr }
    }
}
