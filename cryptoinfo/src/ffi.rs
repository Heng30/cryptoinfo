use libc::c_int;
use std::ffi;
use cpp::cpp;

cpp!{{ #include "bar/bar.h" }}

#[link(name = "foo")]
extern {
    fn foo_say_hello(name: *const i8) -> c_int;
}

pub fn run_ffi() {
    {
        let name = ffi::CString::new("jack").unwrap();
        unsafe { foo_say_hello(name.as_ptr()); };
    }

    {

        let name = std::ffi::CString::new("mike").unwrap();
        let name_ptr = name.as_ptr();
        let r = unsafe {
            cpp!([name_ptr as "const char *"] -> i32 as "int" {
               return bar_say_hello(name_ptr);
            })
        };

        println!("{}", r);
    }
}
