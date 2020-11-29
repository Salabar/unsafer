use unsafer::shared_box::SharedBox;
use unsafer::pointers::*;
use unsafer::assume::*;

struct Increment {
    ctr : *mut i32
}

impl Increment {
    fn inc(&mut self) {
        unsafe {
            *self.ctr += 1;
        }
    }
}

#[test]
fn shared_test() {
    let s = Box::new(0);
    let mut s = SharedBox::from(s);
    {
        let mut a = Increment { ctr : s.as_ptr() as *mut i32 };
        let mut b = Increment { ctr : s.as_ptr() as *mut i32 };
        a.inc();
        b.inc();
    }
    let s = unsafe { s.into_box() };

    assert_eq!(*s, 2);
}

#[allow(unused_unsafe)]
unsafe fn swap_ptr(a : *mut i32, b : *mut i32) {
    let mut bind = Bind::new();
    unsafe {
        let temp = *bind.get(a);
        *bind.get_mut(a) = *bind.get(b);
        *bind.get_mut(b) = temp;
    }
}

#[test]
fn bind_test() {
    let mut a = 0;
    let mut b = 10;
    unsafe {
        swap_ptr(&mut a, &mut b);
    }
    assert_eq!(a, 10);

    let p = &mut a as *mut i32;
    unsafe {
        swap_ptr(p, p);
    }
    assert_eq!(a, 10);
}

use std::collections::HashMap;

#[test]
fn assume_test() {
    let v = vec![1, 4, 7, 9];

    unsafe { assume(|| v.len() == 4); }
    let second = v[2];

    assert!(second == 7);

    let mut dict = HashMap::new();
    dict.insert("first", 64);
    dict.insert("second", 93);
    dict.insert("third", 1256);
    dict.insert("fourth", 5483);

    let a = unsafe { *dict.get("third").assume_some() };
    assert!(a == 1256);
}


