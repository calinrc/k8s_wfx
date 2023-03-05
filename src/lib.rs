use consts::FsDefaultParamStruct;
use consts::RemoteInfoStruct;
use consts::TLogProc;
use consts::TProgressProc;
use consts::TRequestProc;
use consts::BOOL;
use consts::FILETIME;
use consts::FS_EXEC_OK;
use consts::FS_FILE_OK;
use consts::HANDLE;
use consts::HWND;
use consts::INVALID_HANDLE;
use consts::WIN32_FIND_DATAA;
use iterators::ResourcesItertatorFactory;
use iterators::FindDataUpdater;
use std::cell::RefCell;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CStr;
use std::ffi::CString;
use std::path::Path;

mod consts;
mod iterators;
mod resources;


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
    eprintln!("FsInit enter");
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
        None => eprintln!("FsInit local"),
    });
    tracing_subscriber::fmt::init();
    eprintln!("FsInit exit");

    0
}

#[no_mangle]
pub unsafe extern "C" fn FsFindFirst(
    path: *mut c_char,
    find_data: *mut WIN32_FIND_DATAA,
) -> HANDLE {
    eprintln!("FsFindFirst enter");
    let path_str = CStr::from_ptr(path).to_string_lossy();
    eprintln!("FsFindFirst on path {}", path_str);
    let path = Path::new(path_str.as_ref());
    let parent = path.parent();
    eprintln!("Parent is none {}", parent.is_none());
    let mut rit = ResourcesItertatorFactory::new(path);
    
    let handle = match (*rit).next() {
        Some(_) => {
            // Thin pointer
            rit.update_find_data(find_data);
            let thin_ptr = Box::new(rit);
            let mbrit = Box::into_raw(thin_ptr);
             mbrit as *mut _ as HANDLE
        }
        None => INVALID_HANDLE,
    };
    eprintln!("FsFindFirst exit");
    handle
}

#[no_mangle]
pub unsafe extern "C" fn FsFindNext(hdl: HANDLE, find_data: *mut WIN32_FIND_DATAA) -> c_int {
    eprintln!("FsFindNext enter");
    let ret_val: c_int = {
        if hdl != INVALID_HANDLE {
            let riit = hdl as *mut Box<dyn FindDataUpdater>;
             //= hdl as *mut Box<ResourcesIterator>;
            //let mut riit: ManuallyDrop<Box<ResourcesIterator>> = unsafe { (hdl as ManuallyDrop<Box<ResourcesIterator>>) };
            //as *mut ResourcesIterator;
            match (*riit).next() {
                Some(_) => {
                    (*riit).update_find_data(find_data);
                    1
                }
                None => {
                    println!("None elem");
                    0
                }
            }
        } else {
            0
        }
    };

    eprintln!("FsFindNext exit");
    ret_val
}

#[no_mangle]
pub unsafe extern "C" fn FsFindClose(hdl: HANDLE) -> c_int {
    eprintln!("FsFindClose enter");
    //let riit: &mut ResourcesIterator = unsafe { &mut *(hdl as *mut ResourcesIterator) };
//    let mdrit: &mut ManuallyDrop<ResourcesIterator> = unsafe { &mut *(hdl as *mut ManuallyDrop<ResourcesIterator>) };
//    ManuallyDrop::into_inner(&mdrit);
    if hdl != INVALID_HANDLE {
        let _ = Box::from_raw(hdl as *mut Box<dyn FindDataUpdater>);
    }
    eprintln!("FsFindClose exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsMkDir(path: *mut c_char) -> BOOL {
    eprintln!("FsMkDir enter");
    eprintln!("FsMkDir exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsRemoveDir(remote_name: *mut c_char) -> BOOL {
    eprintln!("FsRemoveDir enter");
    eprintln!("FsRemoveDir exit");
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
    eprintln!("FsRenMovFile enter");
    eprintln!("FsRenMovFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsGetFile(
    remote_name: *mut c_char,
    local_name: *mut c_char,
    copy_flags: c_int,
    ri: *mut RemoteInfoStruct,
) -> c_int {
    eprintln!("FsGetFile enter");
    eprintln!("FsGetFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsPutFile(
    local_name: *mut c_char,
    remote_name: *mut c_char,
    copy_flags: c_int,
) -> c_int {
    eprintln!("FsPutFile enter");
    eprintln!("FsPutFile exit");
    FS_FILE_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsExecuteFile(
    main_win: HWND,
    remote_name: *mut c_char,
    verb: *mut c_char,
) -> c_int {
    eprintln!("FsExecuteFile enter");
    eprintln!("FsExecuteFile exit");
    FS_EXEC_OK
}

#[no_mangle]
pub unsafe extern "C" fn FsDeleteFile(remote_name: *mut c_char) -> BOOL {
    eprintln!("FsDeleteFile enter");
    eprintln!("FsDeleteFile exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsSetTime(
    remote_name: *mut c_char,
    creation_time: *mut FILETIME,
    last_access_time: *mut FILETIME,
    last_write_time: *mut FILETIME,
) -> BOOL {
    eprintln!("FsSetTime enter");
    eprintln!("FsSetTime exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsDisconnect(disconnect_root: *mut c_char) -> BOOL {
    eprintln!("FsDisconnect enter");
    eprintln!("FsDisconnect exit");
    0
}

#[no_mangle]
pub unsafe extern "C" fn FsSetDefaultParams(dps: *mut FsDefaultParamStruct) {
    eprintln!("FsSetDefaultParams enter");
    let str = CStr::from_ptr((*dps).default_ini_name.as_ptr()).to_string_lossy();

    eprintln!(
        "FsSetDefaultParams  {} version {}:{} size {}",
        str,
        (*dps).plugin_interface_version_hi,
        (*dps).plugin_interface_version_low,
        (*dps).size
    );
    eprintln!("FsSetDefaultParams exit");
}

#[no_mangle]
pub unsafe extern "C" fn FsGetDefRootName(def_root_name: *mut c_char, maxlen: c_int) {
    eprintln!("FsGetDefRootName enter");
    let plugin_name = "k8s";
    let bytes = plugin_name.as_bytes();
    let len = bytes.len();
    std::ptr::copy(
        plugin_name.as_bytes().as_ptr().cast(),
        def_root_name,
        maxlen as usize,
    );
    std::ptr::write(def_root_name.offset(len as isize) as *mut u8, 0u8);
    eprintln!("FsGetDefRootName exit");
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
