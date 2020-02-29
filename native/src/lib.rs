use std::prelude::v1::*;
use std::ptr;
use std::slice;

use std::cell::RefCell;
use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use std::os::raw::{c_char, c_int};

use log::{error, warn};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

thread_local! {
    static LAST_ERROR: RefCell<Option<Box<dyn std::error::Error>>> = RefCell::new(None);
}

enum ErrorCode {
    General = -1,
    BufferTooSmall = -10,
    BufferNullPointer = -11,
    SyntaxNotFound = -20,
    SyntaxSetNullPointer = -21,
    ThemeNotFound = -30,
    ThemeSetNullPointer = -31,
}

/// Update the most recent error, clearing whatever may have been there before.
pub fn update_last_error<E: std::error::Error + 'static>(err: E) {
    error!("Setting LAST_ERROR: {}", err);

    {
        let mut source = err.source();
        while let Some(parent_err) = source {
            warn!("Caused by: {}", parent_err);
            source = parent_err.source();
        }
    }

    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err));
    });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<Box<dyn std::error::Error>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}

/// Write the most recent error message into a caller-provided buffer as a UTF-8
/// string, returning the number of bytes written.
///
/// # Note
///
/// This writes a **UTF-8** string into the buffer. Windows users may need to
/// convert it to a UTF-16 "unicode" afterwards.
///
/// If there are no recent errors then this returns `0` (because we wrote 0
/// bytes). `-1` is returned if there are any errors, for example when passed
/// a null pointer or a buffer of insufficient size.
#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        warn!("A null pointer passed into last_error_message() as the buffer");
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        warn!("Buffer provided for writing the last error message is too small.");
        warn!(
            "Excepted at least {} bytes but got {}",
            error_message.len() + 1,
            buffer.len()
        );
        return -1;
    }

    ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}

#[no_mangle]
pub extern "C" fn load_default_theme_set() -> *mut ThemeSet {
    let ts = ThemeSet::load_defaults();
    Box::into_raw(Box::new(ts))
}

#[no_mangle]
pub unsafe extern "C" fn release_theme_set(ts_ptr: *mut ThemeSet) {
    if !ts_ptr.is_null() {
        drop(Box::from_raw(ts_ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_all_themes(
    buf: *mut c_char,
    buf_len: c_int,
    ts_ptr: *const ThemeSet,
) -> c_int {
    if buf.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null buffer"));
        return ErrorCode::BufferNullPointer as c_int;
    }
    if ts_ptr.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null pointer"));
        return ErrorCode::ThemeSetNullPointer as c_int;
    }

    let names = (&*ts_ptr)
        .themes
        .keys()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    let buf = slice::from_raw_parts_mut(buf as *mut u8, buf_len as usize);
    if names.len() >= buf.len() {
        update_last_error(Error::new(ErrorKind::Other, "Buffer too small"));
        return ErrorCode::BufferTooSmall as c_int;
    }

    ptr::copy_nonoverlapping(names.as_ptr(), buf.as_mut_ptr(), names.len());
    names.len() as c_int
}

#[no_mangle]
pub extern "C" fn load_default_syntax_set(newlines: bool) -> *mut SyntaxSet {
    let ss = if newlines {
        SyntaxSet::load_defaults_newlines()
    } else {
        SyntaxSet::load_defaults_nonewlines()
    };

    Box::into_raw(Box::new(ss))
}

#[no_mangle]
pub unsafe extern "C" fn release_syntax_set(ss_ptr: *mut SyntaxSet) {
    if !ss_ptr.is_null() {
        drop(Box::from_raw(ss_ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_all_syntaxes(
    buf: *mut c_char,
    buf_len: c_int,
    ss_ptr: *const SyntaxSet,
) -> c_int {
    if buf.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null buffer"));
        return ErrorCode::BufferNullPointer as c_int;
    }
    if ss_ptr.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null pointer"));
        return ErrorCode::SyntaxSetNullPointer as c_int;
    }

    let names = (&*ss_ptr)
        .syntaxes()
        .iter()
        .map(|syntax| syntax.name.clone())
        .collect::<Vec<String>>()
        .join("\n");
    let buf = slice::from_raw_parts_mut(buf as *mut u8, buf_len as usize);
    if names.len() >= buf.len() {
        update_last_error(Error::new(ErrorKind::Other, "Buffer too small"));
        return ErrorCode::BufferTooSmall as c_int;
    }

    ptr::copy_nonoverlapping(names.as_ptr(), buf.as_mut_ptr(), names.len());
    names.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn find_syntax_by_extension(
    buffer: *mut c_char,
    buf_len: c_int,
    ss_ptr: *const SyntaxSet,
    ext: *const c_char,
) -> c_int {
    if buffer.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null buffer"));
        return ErrorCode::BufferNullPointer as c_int;
    }
    if ss_ptr.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null SyntaxSet"));
        return ErrorCode::SyntaxSetNullPointer as c_int;
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, buf_len as usize);
    let ext = match CStr::from_ptr(ext).to_str() {
        Ok(t) => t,
        Err(e) => {
            update_last_error(e);
            return -1;
        }
    };

    let syntax = match (&*ss_ptr).find_syntax_by_extension(ext) {
        Some(t) => t,
        None => {
            update_last_error(Error::new(ErrorKind::Other, "Syntax not found"));
            return ErrorCode::SyntaxNotFound as c_int;
        }
    };

    let sn = &syntax.name;
    if sn.len() >= buffer.len() {
        update_last_error(Error::new(ErrorKind::Other, "Buffer too small"));
        return ErrorCode::BufferTooSmall as c_int;
    }
    ptr::copy_nonoverlapping(sn.as_ptr(), buffer.as_mut_ptr(), sn.len());
    buffer[sn.len()] = 0;
    sn.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn find_syntax_by_token(
    buffer: *mut c_char,
    buf_len: c_int,
    ss_ptr: *const SyntaxSet,
    token: *const c_char,
) -> c_int {
    if buffer.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null buffer"));
        return ErrorCode::BufferNullPointer as c_int;
    }
    if ss_ptr.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null SyntaxSet"));
        return ErrorCode::SyntaxSetNullPointer as c_int;
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, buf_len as usize);
    let token = match CStr::from_ptr(token).to_str() {
        Ok(t) => t,
        Err(e) => {
            update_last_error(e);
            return -1;
        }
    };

    let syntax = match (&*ss_ptr).find_syntax_by_token(token) {
        Some(t) => t,
        None => {
            update_last_error(Error::new(ErrorKind::Other, "Syntax not found"));
            return ErrorCode::SyntaxNotFound as c_int;
        }
    };

    let sn = &syntax.name;
    if sn.len() >= buffer.len() {
        update_last_error(Error::new(ErrorKind::Other, "Buffer too small"));
        return ErrorCode::BufferTooSmall as c_int;
    }
    ptr::copy_nonoverlapping(sn.as_ptr(), buffer.as_mut_ptr(), sn.len());
    buffer[sn.len()] = 0;
    sn.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn highlight_to_html(
    buffer: *mut c_char,
    buf_len: i32,
    src: *const c_char,
    ss_ptr: *const SyntaxSet,
    syntax_name: *const c_char,
    ts_ptr: *const ThemeSet,
    theme_name: *const c_char,
) -> c_int {
    if buffer.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null buffer"));
        return -1;
    }
    if ss_ptr.is_null() || ts_ptr.is_null() {
        update_last_error(Error::new(ErrorKind::Other, "Null pointer"));
        return if ss_ptr.is_null() {
            ErrorCode::SyntaxSetNullPointer
        } else {
            ErrorCode::ThemeSetNullPointer
        } as c_int;
    }

    let buffer = slice::from_raw_parts_mut(buffer as *mut u8, buf_len as usize);
    let src = match CStr::from_ptr(src).to_str() {
        Ok(t) => t,
        Err(e) => {
            update_last_error(e);
            return -1;
        }
    };
    let syntax_name = match CStr::from_ptr(syntax_name).to_str() {
        Ok(t) => t,
        Err(e) => {
            update_last_error(e);
            return -1;
        }
    };
    let theme_name = match CStr::from_ptr(theme_name).to_str() {
        Ok(t) => t,
        Err(e) => {
            update_last_error(e);
            return ErrorCode::General as c_int;
        }
    };

    let syntax = match (&*ss_ptr).find_syntax_by_name(syntax_name) {
        Some(t) => t,
        None => {
            update_last_error(Error::new(ErrorKind::Other, "Syntax not found"));
            return ErrorCode::SyntaxNotFound as c_int;
        }
    };

    let theme = match (&*ts_ptr).themes.get(theme_name) {
        Some(t) => t,
        None => {
            update_last_error(Error::new(ErrorKind::Other, "Theme not found"));
            return ErrorCode::ThemeNotFound as c_int;
        }
    };

    let highlighted = highlighted_html_for_string(src, &*ss_ptr, syntax, theme);
    if highlighted.len() >= buffer.len() {
        update_last_error(Error::new(ErrorKind::Other, "Buffer too small"));
        return ErrorCode::BufferTooSmall as c_int;
    }

    ptr::copy_nonoverlapping(highlighted.as_ptr(), buffer.as_mut_ptr(), highlighted.len());
    buffer[highlighted.len()] = 0;

    highlighted.len() as c_int
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
