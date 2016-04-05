use std::cmp::Ordering;
use std::str;

#[inline]
pub unsafe fn real_memcmp(a: &[u8], b: &[u8], len: usize) -> i32 {
    // NOTE: In theory n should be libc::size_t and not usize, but libc is not available here
    #[allow(improper_ctypes)]
    extern { fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32; }
    memcmp(a.as_ptr() as *const i8, b.as_ptr() as *const i8, len)
}

#[inline]
pub unsafe fn my_memcmp(a: &[u8], b: &[u8], len: usize) -> i32 {
    let mut a_ptr = a.as_ptr() as *const i8;
    let mut b_ptr = b.as_ptr() as *const i8;

    let end = a_ptr.offset(len as isize);

    /*

    while a_ptr != end && a_ptr % mem::align_of::<u32>() != 0 {
        let value = *a_ptr - *b_ptr;
        if value != 0 {
            return value as i32;
        }
        a_ptr = a_ptr.offset(1);
        b_ptr = b_ptr.offset(1);
        len -= 1;
    }

    let a_ptr = a_ptr as *u32;
    let a_ptr = a_ptr as *u32;

    while len != 0 && len > mem::size_of::<u32> {
        let value = *a_ptr - *b_ptr;
        if value != 0 {
            return value as i32;
        }
        a_ptr = a_ptr.offset(1);
        b_ptr = b_ptr.offset(1);
        len 
    }
    */

    /*
    while a_ptr != end && *a_ptr % mem::align_of::<u32>() != 0 {
        let value = *a_ptr - *b_ptr;
        if value != 0 {
            println!("mismatch! {} {} {}", *a_ptr, *b_ptr, value);
            return value as i32;
        }
        a_ptr = a_ptr.offset(1);
        b_ptr = b_ptr.offset(1);
    }
    */

    while a_ptr != end {
        let value = *a_ptr - *b_ptr;
        if value != 0 {
            return value as i32;
        }
        a_ptr = a_ptr.offset(1);
        b_ptr = b_ptr.offset(1);
    }

    0
}

#[inline]
pub unsafe fn rust_cmp(a: &[u8], b: &[u8], _len: usize) -> i32 {
    let a = str::from_utf8_unchecked(a);
    let b = str::from_utf8_unchecked(b);

    match a.cmp(b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[inline]
pub unsafe fn my_memeq(a: &[u8], b: &[u8], len: usize) -> bool {
    let mut a_ptr = a.as_ptr() as *const i8;
    let mut b_ptr = b.as_ptr() as *const i8;

    let end = a_ptr.offset(len as isize);

    while a_ptr != end {
        if *a_ptr != *b_ptr {
            return false;
        }
        a_ptr = a_ptr.offset(1);
        b_ptr = b_ptr.offset(1);
    }

    true
}

#[inline]
pub unsafe fn rust_eq(a: &[u8], b: &[u8], _len: usize) -> bool {
    //let a = str::from_utf8_unchecked(a);
    //let b = str::from_utf8_unchecked(b);

    a == b
}
