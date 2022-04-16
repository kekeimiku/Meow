#![allow(dead_code)]

use std::{
    ffi::{CStr, CString, NulError},
    num::ParseIntError,
    str::Utf8Error,
};

use dec::decode_a64;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseIntError),
    Utf8Error(Utf8Error),
    NulError(NulError),
    DecodeError(String),
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::NulError(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::DecodeError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => write!(f, "Parse error: {}", e),
            Error::Utf8Error(e) => write!(f, "Utf8 error: {}", e),
            Error::NulError(e) => write!(f, "Nul error: {}", e),
            Error::DecodeError(e) => write!(f, "Decode error: {}", e),
        }
    }
}

static LEN: usize = 2048;

// // 把 [str] 转换成 [hex str] 再转换成 u32
// fn vstr_to_u32(vs: &mut [&str]) -> Result<u32, Error> {
//     //大小端
//     vs.reverse();
//     //去掉vec中每个元素的\x然后组合成十六进制字符串然后转换成u32
//     Ok(u32::from_str_radix(
//         &vs.iter_mut()
//             .map(|f| f.replace("\\x", ""))
//             .collect::<Vec<_>>()
//             .join(""),
//         16,
//     )?)
// }

////////////////////////////////////////////////////////////////

fn jstr_to_vu32(s: &str) -> Result<Vec<u32>, Error> {
    Ok(s.as_bytes()
        .chunks(16)
        .map(|b| -> Result<u32, ParseIntError> {
            let mut t1 = unsafe { core::str::from_utf8_unchecked(b) }
                .split("\\x")
                .collect::<Vec<&str>>();
            t1.remove(0);
            t1.reverse();
            Ok(u32::from_str_radix(&t1.join(""), 16)?)
        })
        .into_iter()
        .collect::<Result<Vec<u32>, ParseIntError>>()?)
}

fn decode(ptr: *mut u8) -> Result<CString, Error> {
    // 从js传过来的字符串
    let js_str = unsafe { CStr::from_ptr(ptr as *const _).to_str()? };

    if js_str.is_empty(){
        return Err(Error::DecodeError("输入为空\n".into()))
    }

    //ok.remove(0);
    Ok(CString::new(
        jstr_to_vu32(js_str)?
            .iter()
            .map(|s| -> Result<String, Error> {
                Ok(format!(
                    "{:?}\n",
                    decode_a64(*s).ok_or_else(|| "decode err\n".to_string())?
                ))
            })
            .into_iter()
            .collect::<Result<String, Error>>()?,
    )?)
}

////////////////////////////////////////////////////////////////

///
/// // fn decode(ptr: *mut u8) -> Result<CString, Error> {
//     // 从js传过来的字符串
//     let js_str = unsafe { CStr::from_ptr(ptr as *const _).to_str()? };
//     let mut jvasm = js_str.split('\\').collect::<Vec<_>>();
//     jvasm.remove(0);
//     // 反编译结果
//     // 返回的字符串
//     Ok(CString::new(format!(
//         "{:?}",
//         decode_a64(vstr_to_u32(&mut jvasm)?).ok_or_else(|| "decode err".to_string())?
//     ))?)
// }

#[no_mangle]
pub extern "C" fn alloc() -> *mut std::os::raw::c_void {
    let mut buf = Vec::with_capacity(LEN);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// # Safety
///
////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut std::os::raw::c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, LEN);
}

// fn decode(ptr: *mut u8) -> Result<CString, Error> {
//     // 从js传过来的字符串
//     let js_str = unsafe { CStr::from_ptr(ptr as *const _).to_str()? };
//     let mut jvasm = js_str.split('\\').collect::<Vec<_>>();
//     jvasm.remove(0);
//     // 返回结果
//     Ok(CString::new(format!(
//         "{:?}",
//         decode_a64(vstr_to_u32(&mut jvasm)?).ok_or_else(|| "decode err".to_string())?
//     ))?)
// }

fn write_str(ptr: *mut u8, cstr: &CStr) {
    let bytes = cstr.to_bytes_with_nul();
    let header_bytes = unsafe { std::slice::from_raw_parts_mut(ptr, LEN) };
    header_bytes[..bytes.len()].copy_from_slice(bytes);
}

#[no_mangle]
pub extern "C" fn start(ptr: *mut u8) {
    match decode(ptr) {
        Ok(cstr) => write_str(ptr, &cstr),
        Err(err) => {
            let cstr = CString::new(err.to_string()).unwrap();
            write_str(ptr, &cstr)
        }
    }
}

// \xa2\x01\x80\xd2\x20\x00\x80\xd2\x30\x00\x80\xd2\x09\xfc\x5f\x08
// 草 \xa2\x01\x80\xd2\x20\x00\x80\xd2\x30\x00\x80\xd2\x09\xfc\x5f\x08
