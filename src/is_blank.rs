#[no_mangle]
pub extern fn is_blank(string: *const c_char) -> bool {
  let c_str = unsafe {
    assert!(!string.is_null());

    CStr::from_ptr(string)
  };

  str::from_utf8(c_str.to_bytes()).unwrap().trim().is_empty()
}
