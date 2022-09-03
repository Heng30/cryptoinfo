use cpp::{cpp, cpp_class};
use libc::c_int;
use std::ffi;

cpp! {{ #include "bar/bar.h" }}

cpp_class!(unsafe struct Bar as "Bar");
impl Bar {
    fn new(base: i32) -> Self {
        unsafe {
            return cpp!([base as "int"] -> Bar as "Bar" {
                Bar bar(base);
                return bar;
            });
        }
    }

    fn add(&mut self, a: i32, b: i32) -> i64 {
        unsafe {
            return cpp!([self as "Bar*", a as "int", b as "int"] -> i64 as "int64_t" {
                return self->add(a, b);
            });
        }
    }
}

#[link(name = "foo")]
extern "C" {
    fn foo_say_hello(name: *const i8) -> c_int;
}

pub fn run_ffi() {
    {
        let name = ffi::CString::new("jack").unwrap();
        unsafe {
            foo_say_hello(name.as_ptr());
        };
    }

    {
        let name = ffi::CString::new("mike").unwrap();
        let name_ptr = name.as_ptr();
        let r = unsafe {
            cpp!([name_ptr as "const char *"] -> i32 as "int" {
               return bar_say_hello(name_ptr);
            })
        };

        println!("{}", r);
    }

    {
        let mut bar = Bar::new(10);
        println!("{}", bar.add(1, 2));
    }
}
