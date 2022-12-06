use consts::FsDefaultParamStruct;
use consts::RemoteInfoStruct;
use consts::TLogProc;
use consts::TProgressProc;
use consts::TRequestProc;
use consts::FILETIME;
use consts::FS_EXEC_OK;
use consts::FS_FILE_OK;
use consts::HANDLE;
use consts::HWND;
use consts::INVALID_HANDLE;
use consts::WIN32_FIND_DATAA;
use std::ffi::CString;

use consts::BOOL;
use std::cell::RefCell;
use std::ffi::c_char;
use std::ffi::c_int;

mod consts;

// File: lib.rs

// For further reading ...
// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098
//
// typedef int (DCPCALL *tProgressProc)(int PluginNr,char* SourceName, char* TargetName,int PercentDone);
// typedef void (DCPCALL *tLogProc)(int PluginNr,int MsgType,char* LogString);
// typedef BOOL (DCPCALL *tRequestProc)(int PluginNr,int RequestType,char* CustomTitle, char* CustomText,char* ReturnedText,int maxlen);
// typedef int BOOL;

thread_local!(static G_PLUGIN_NO: RefCell<Option<c_int>>  = RefCell::new(None));
thread_local!(static G_PROGRESS_PROC: RefCell<Option<TProgressProc> >  = RefCell::new(None));
thread_local!(static G_LOG_PROC: RefCell<Option<TLogProc> >  = RefCell::new(None));
thread_local!(static G_REQUEST_PROC: RefCell<Option<TRequestProc> >  = RefCell::new(None));

#[no_mangle]
pub unsafe extern "C" fn FsInit(
    plugin_nr: c_int,
    p_progress_proc: TProgressProc,
    p_log_proc: TLogProc,
    p_request_proc: TRequestProc,
) -> c_int {
    G_PLUGIN_NO.with(|plug_no| {
        *plug_no.borrow_mut() = Some(plugin_nr);
    });
    G_PROGRESS_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_progress_proc);
    });
    G_LOG_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_log_proc);
    });
    G_REQUEST_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_request_proc);
    });
    G_LOG_PROC.with(|logger_opt| match *logger_opt.borrow() {
        Some(logger) => {
            let c_string = CString::new("FsInit logger").expect("CString::new failed");
            let ptr = c_string.into_raw();
            logger(plugin_nr, 0, ptr);
            let _ = CString::from_raw(ptr as *mut _);
        }
        None => println!("FsInit local"),
    });

    0
}

#[no_mangle]
pub unsafe extern "C" fn FsFindFirst(
    path: *mut c_char,
    find_data: *mut WIN32_FIND_DATAA,
) -> HANDLE {
    print!("FsFindFirst enter");
    print!("FsFindFirst exit");
    INVALID_HANDLE
}

#[no_mangle]
pub unsafe extern "C" fn FsFindNext(hdl: HANDLE, find_data: *mut WIN32_FIND_DATAA) -> c_int {
    print!("FsFindNext enter");
    print!("FsFindNext exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsFindClose(hdl: HANDLE) -> c_int {
    print!("FsFindClose enter");
    print!("FsFindClose exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsMkDir(path: *mut c_char) -> BOOL {
    print!("FsMkDir enter");
    print!("FsMkDir exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsRemoveDir(remote_name: *mut c_char) -> BOOL {
    print!("FsRemoveDir enter");
    print!("FsRemoveDir exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsRenMovFile(
    old_name: *mut c_char,
    new_name: *mut c_char,
    mmove: BOOL,
    over_write: BOOL,
    ri: *mut RemoteInfoStruct,
) -> c_int {
    print!("FsRenMovFile enter");
    print!("FsRenMovFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsGetFile(
    remote_name: *mut c_char,
    local_name: *mut c_char,
    copy_flags: c_int,
    ri: *mut RemoteInfoStruct,
) -> c_int {
    print!("FsGetFile enter");
    print!("FsGetFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsPutFile(
    local_name: *mut c_char,
    remote_name: *mut c_char,
    copy_flags: c_int,
) -> c_int {
    print!("FsPutFile enter");
    print!("FsPutFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsExecuteFile(
    main_win: HWND,
    remote_name: *mut c_char,
    verb: *mut c_char,
) -> c_int {
    print!("FsExecuteFile enter");
    print!("FsExecuteFile exit");
    FS_EXEC_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsDeleteFile(remote_name: *mut c_char) -> BOOL {
    print!("FsDeleteFile enter");
    print!("FsDeleteFile exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsSetTime(
    remote_name: *mut c_char,
    creation_time: *mut FILETIME,
    last_access_time: *mut FILETIME,
    last_write_time: *mut FILETIME,
) -> BOOL {
    print!("FsSetTime enter");
    print!("FsSetTime exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsDisconnect(disconnect_root: *mut c_char) -> BOOL {
    print!("FsDisconnect enter");
    print!("FsDisconnect exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsSetDefaultParams(dps: *mut FsDefaultParamStruct) {
    print!("FsSetDefaultParams enter");
    print!("FsSetDefaultParams exit");
}

#[no_mangle]
pub unsafe extern "C" fn FsGetDefRootName(def_root_name: *mut c_char, maxlen: c_int) {
    print!("FsGetDefRootName enter");
    let plugin_name = "k8s";
    let bytes = plugin_name.as_bytes();
    let len = bytes.len();
    std::ptr::copy(
        plugin_name.as_bytes().as_ptr().cast(),
        def_root_name,
        maxlen as usize,
    );
    std::ptr::write(def_root_name.offset(len as isize) as *mut u8, 0u8);
    print!("FsGetDefRootName exit");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
