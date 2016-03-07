use alloc::heap::deallocate;
use std::mem::{ align_of_val, size_of_val };
use std::ptr::{ read, Shared };
use std::sync::atomic::{ AtomicUsize, Ordering };

pub struct DecayPtr<T> {
    _ptr: Shared<DecayInner<T>>,
    has_decayed: bool,
}

struct DecayInner<T> {
    decay_count: AtomicUsize,
    data: T,
}

unsafe impl<T: Send> Send for DecayPtr<T> { }
unsafe impl<T: Send> Send for DecayInner<T> { }

impl<T> DecayPtr<T> {
    pub fn new(data: T) -> DecayPtr<T> {
        let inner = Box::new(
            DecayInner {
                decay_count: AtomicUsize::new(1),
                data: data,
            }
        );
        DecayPtr {
            _ptr: unsafe {
                Shared::new(Box::into_raw(inner))
            },
            has_decayed: false,
        }
    }

    pub fn decay(mut self) -> Option<T> {
        assert!(!self.has_decayed, "A decay pointer cannot decay twice.");
        self.has_decayed = true;

        let should_release = self.decay_decrement() == 0;

        if should_release {
            Some(self.decay_release())
        }
        else {
            None
        }
    }

    fn decay_decrement(&self) -> usize {
        let inner = self.inner_ref();
        inner.decay_count.fetch_sub(1, Ordering::Acquire)
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

impl<T> DecayPtr<T> {
    fn inner_ref(&self) -> &DecayInner<T> {
        unsafe { &**self._ptr }
    }

    fn inner_raw(self) -> *mut DecayInner<T> {
        *self._ptr
    }

    pub fn clone(&self) ->DecayPtr<T> {
        assert!(!self.has_decayed, "A decay pointer cannot be cloned if it has decayed.");
        self.inner_ref().decay_count.fetch_add(1, Ordering::Relaxed);
        DecayPtr {
            _ptr: self._ptr,
            has_decayed: false,
        }
    }
}

impl<T> Drop for DecayPtr<T> {
    fn drop(&mut self) {
        assert!(self.has_decayed, "A decay pointer must always decay before it is dropped.");
    }
}
