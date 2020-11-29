use core::ptr::NonNull;
use core::marker::PhantomData;
use core::slice;
use core::mem::MaybeUninit;

/// When working with raw pointers, one must be careful to avoid mutable aliasing. Bind is a zero-sized structure that
/// makes it easier by ensuring only one pointer can be dereferenced at a time in the same scope.
/// # Examples
/// ```
/// # use unsafer::pointers::Bind;
/// unsafe fn swap_aliasing(a : *mut i32, b : *mut i32) {
///     let mut bind = Bind::new();
///     unsafe {
///         let r = bind.get(a);
///         let temp = *r;
///         *bind.get_mut(a) = *bind.get(b);
///         *bind.get_mut(b) = temp;
///     }
/// }
/// ```
///
/// ```
/// # use unsafer::pointers::Bind;
/// // Bind only protects from aliasing specifically and does not make the function safe by itself
/// fn wrong_usage(a : *mut i32, b : *mut i32) {
///     let mut bind1 = Bind::new();
///     let mut bind2 = Bind::new();
///     unsafe {
///         let r1 = bind1.get_mut(a); 
///         let r2 = bind2.get_mut(b); // This is still a UB when a == b
///     }
/// }
/// ```
pub struct Bind<T> {
    _ph: PhantomData<*mut T>
}

pub trait Pointer<T> {
    /// Casts a generic pointer into a mutable pointer.
    fn as_mut_ptr(&mut self) -> *mut T;
    /// Casts a generic pointer into a const pointer.
    fn as_ptr(&self) -> *const T;

    /// Runs `f` and writes the result into the location.
    /// # Safety
    /// Every invariant of write applies to this. Memory behind the pointer must be treated as undefined if
    /// `f` panics.
    unsafe fn write_with(&mut self, f : impl FnOnce() -> T) {
        self.as_mut_ptr().write(f());
    }
}

impl <T> Pointer<T> for *mut T {
    fn as_mut_ptr(&mut self) -> *mut T {
        *self
    }

    fn as_ptr(&self) -> *const T {
        *self as *const T
    }
}

impl <T> Pointer<T> for *const T {
    fn as_mut_ptr(&mut self) -> *mut T {
        *self as *mut T
    }

    fn as_ptr(&self) -> *const T {
        *self
    }
}

impl <T> Pointer<T> for NonNull<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_ptr() as *mut T
    }

    fn as_ptr(&self) -> *const T {
        (*self).as_ptr()
    }
}

impl <T> Pointer<T> for &MaybeUninit<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        panic!("Mutably accessing an immutable MaybeUninit would be unsound");
    }

    fn as_ptr(&self) -> *const T {
        (**self).as_ptr()
    }
}


impl <T> Pointer<T> for &mut MaybeUninit<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        (**self).as_mut_ptr()
    }

    fn as_ptr(&self) -> *const T {
        (**self).as_ptr()
    }
}

impl <T> Bind<T> {
    pub fn new() -> Self {
        Bind { _ph : PhantomData }
    }

    /// Dereferences `ptr` and binds the reference to `self`.
    /// # Safety
    /// Every invariant for dereferencing a raw pointer applies to Bind.
    pub unsafe fn get(&self, ptr : impl Pointer<T>) -> &T {
        unsafe {
            &*(ptr.as_ptr())
        }
    }

    /// Mutably dereferences `ptr` and binds the reference to `self`.
    /// # Safety
    /// Every invariant for dereferencing a raw pointer applies to Bind.
    pub unsafe fn get_mut(&mut self, mut ptr : impl Pointer<T>) -> &mut T {
        unsafe {
            &mut *(ptr.as_mut_ptr())
        }
    }

    /// Creates a slice and binds it to `self`
    /// # Safety
    /// Every invariant of slice::from_raw_parts applies to Bind
    pub unsafe fn slice(&self, ptr : impl Pointer<T>, len : usize) -> &[T] {
        unsafe {
            slice::from_raw_parts(ptr.as_ptr(), len)
        }
    }

    /// Creates a mutable slice and binds it to `self`
    /// # Safety
    /// Every invariant of slice::from_raw_parts_mut applies to Bind
    pub unsafe fn slice_mut(&mut self, mut ptr : impl Pointer<T>, len : usize) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(ptr.as_mut_ptr(), len)
        }
    }
}