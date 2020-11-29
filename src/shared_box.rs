use core::ptr::NonNull;
use core::marker::PhantomData;
use core::mem::forget;
use core::convert::From;
/// A Box-like container which manages the memory, but can be dereferenced from multiple locations without MIRI complaining.
/// # Examples
/// ```
/// # use unsafer::shared_box::SharedBox;
/// # struct Increment {
/// #   ctr : *mut i32
/// # }
/// #
/// # impl Increment {
/// #   fn inc(&mut self) {
/// #       unsafe {
/// #           *self.ctr += 1;
/// #       }
/// #   }
/// # }
/// let s = Box::new(0);
/// let mut s = SharedBox::from(s);
/// {
///     let mut a = Increment { ctr : s.as_ptr() as *mut i32 };
///     let mut b = Increment { ctr : s.as_ptr() as *mut i32 };
///     a.inc();
///     b.inc();
/// }
/// let s = unsafe { s.into_box() };
/// assert_eq!(*s, 2);
/// ```

#[repr(transparent)]
pub struct SharedBox<T> {
    data: NonNull<T>,
    _ph: PhantomData<T>        
}

unsafe impl<T: Send> Send for SharedBox<T> { }
unsafe impl<T: Sync> Sync for SharedBox<T> { }

impl <T> From<Box<T>> for SharedBox<T> {
    #[inline(always)]
    fn from(x: Box<T>) -> Self {
        let data = unsafe { NonNull::new_unchecked(Box::into_raw(x)) };
        SharedBox {
            data, _ph : PhantomData
        }
    }
}

impl <T> SharedBox<T> {
    /// Returns a pointer to the object. User must ensure any object which might utilize this pointer is dropped before
    /// the container.
    #[inline(always)]
    pub fn as_ptr(&mut self) -> *const T {
        self.data.as_ptr() as *const T
    }

    /// Consumes SharedBox and turns it into an exclusively owned Box.
    /// # Safety
    /// The user must ensure no active reference to an object in the container still exists.
    #[inline(always)]
    pub unsafe fn into_box(self) -> Box<T> 
    {
        unsafe {
            let r = Box::from_raw(self.data.as_ptr());
            forget(self);
            r
        }
    }
}

impl <T> Drop for SharedBox<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.data.as_ptr());
        }
    }
}