use core::hint::unreachable_unchecked;

/// This has no effect if `predicate` returns true and invokes undefined behavior or panics in debug otherwise. This provides compiler with
/// additional opportunities for optimization when used in the critical paths of your application. Actual effect must
/// always be measured with a profiler and through ASM / IR inspection.
/// # Safety
/// `predicate` must not create any side effects or panic. It's better to avoid potentially panicking functions altogether, including memory allocation
/// and []-indexing as compiler may not be able to remove the code. 
/// You should stick with `assert` macro unless it is possible to formally verify `predicate`.
/// # Examples
/// ```
/// # use unsafer::assume::assume;
/// unsafe fn push_unchecked(v: &mut Vec<i32>, val: i32) {
///     unsafe {
///         assume(|| v.len() < v.capacity());
///         // Compiler is allowed to remove capacity check, reallocation logic and panic handler.
///         v.push(val);
///     }
/// }
/// ```
#[inline(always)]
pub unsafe fn assume(predicate: impl Fn() -> bool)
{
    let f = predicate();
    debug_assert!(f);
    if !f {
        unsafe {
            unreachable_unchecked();
        }
    }
}

pub trait OptionAssume<T> {
    /// Unwraps `self` in it contains Some and invokes undefined behavior or panics in debug otherwise.
    /// # Examples

    /// ```# use unsafer::assume::OptionAssume;
    /// let mut dict = HashMap::new();
    /// dict.insert("first", 64);
    /// dict.insert("second", 93);
    /// dict.insert("third", 1256);
    /// dict.insert("fourth", 5483);
    ///
    /// let a = unsafe { *dict.get("third").assume_some() };
    /// assert!(a == 1256);
    /// // let a = unsafe { *dict.get("thidr").assume_some() }; // UB: get hangs in an infinite loop
    /// // assert!(a == 1256);

    unsafe fn assume_some(self) -> T;
    /// Unwraps `self` in it contains None and invokes undefined behavior or panics in debug otherwise.
    unsafe fn assume_none(self);
}

impl <T> OptionAssume<T> for Option<T> {
    unsafe fn assume_some(self) -> T {
        debug_assert!(self.is_some());
        match self {
            Some(this) => this,
            None => unreachable_unchecked()
        }
    }

    unsafe fn assume_none(self) {
        debug_assert!(self.is_none());
        match self {
            Some(_) => unreachable_unchecked(),
            None => ()
        }
    }
}