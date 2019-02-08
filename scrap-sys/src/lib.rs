extern crate libc;

use scrap;

#[no_mangle]
pub unsafe extern "C" fn error_free(err: *mut libc::c_char) {
    std::ffi::CString::from_raw(err);
}

#[repr(C)]
pub struct DisplayListOrErr {
    list: *mut *const scrap::Display,
    len: usize,
    err: *mut libc::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn display_list() -> DisplayListOrErr {
    let mut list = DisplayListOrErr {
        list: std::ptr::null_mut(),
        len: 0,
        err: std::ptr::null_mut(),
    };
    match scrap::Display::all() {
        Ok(displays) => {
            let mut ptrs: Vec<*const scrap::Display> = displays.iter().map(|d| {
                d as *const scrap::Display
            }).collect();
            ptrs.shrink_to_fit();
            list.list = ptrs.as_mut_ptr();
            list.len = ptrs.len();
            std::mem::forget(ptrs);
        }
        Err(err) => {
            list.err = std::ffi::CString::new(err.to_string()).unwrap().into_raw();
        }
    };
    list
}

#[repr(C)]
pub struct DisplayOrErr {
    display: *mut scrap::Display,
    err: *mut libc::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn display_primary() -> DisplayOrErr {
    let mut display = DisplayOrErr { display: std::ptr::null_mut(), err: std::ptr::null_mut() };
    match scrap::Display::primary() {
        Ok(primary) => {
            display.display = Box::into_raw(Box::new(primary))
        }
        Err(err) => {
            display.err = std::ffi::CString::new(err.to_string()).unwrap().into_raw();
        }
    };
    display
}

#[no_mangle]
pub unsafe extern "C" fn display_free(d: *mut scrap::Display) {
    Box::from_raw(d);
}

#[no_mangle]
pub unsafe extern "C" fn display_width(d: *mut scrap::Display) -> usize {
    (*d).width()
}

#[no_mangle]
pub unsafe extern "C" fn display_height(d: *mut scrap::Display) -> usize {
    (*d).height()
}

#[repr(C)]
pub struct CapturerOrErr {
    capturer: *mut scrap::Capturer,
    err: *mut libc::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn capturer_new(d: *mut scrap::Display) -> CapturerOrErr {
    let display = *Box::from_raw(d);
    let mut ret = CapturerOrErr { capturer: std::ptr::null_mut(), err: std::ptr::null_mut() };
    match scrap::Capturer::new(display) {
        Ok(capturer) => {
            ret.capturer = Box::into_raw(Box::new(capturer))
        }
        Err(err) => {
            ret.err = std::ffi::CString::new(err.to_string()).unwrap().into_raw();
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn capturer_free(c: *mut scrap::Capturer) {
    Box::from_raw(c);
}

#[no_mangle]
pub unsafe extern "C" fn capturer_width(c: *mut scrap::Capturer) -> usize {
    (*c).width()
}

#[no_mangle]
pub unsafe extern "C" fn capturer_height(c: *mut scrap::Capturer) -> usize {
    (*c).height()
}