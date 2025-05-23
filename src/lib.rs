use consts::BOOL;
use consts::FILETIME;
use consts::FsDefaultParamStruct;
use consts::HANDLE;
use consts::HWND;
use consts::INVALID_HANDLE;
use consts::RemoteInfoStruct;
use consts::TLogProc;
use consts::TProgressProc;
use consts::TRequestProc;
use consts::WIN32_FIND_DATAA;
use consts::{
    FS_EXEC_ERROR, FS_EXEC_OK, FS_FILE_EXISTSRESUMEALLOWED, FS_FILE_OK, FS_FILE_READERROR,
    FS_FILE_WRITEERROR,
};
use iterators::FsDataHandler;
use iterators::ResourcesIteratorFactory;
use shareddata::GLOBAL_SHARED_DATA;
use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_char;
use std::ffi::c_int;
use std::path::Path;

mod consts;
mod helper;
mod iterators;
mod resources;
mod shareddata;
// File: lib.rs

// For further reading ...
// #[unsafe(no_mangle)] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098
//
// typedef int (DCPCALL *tProgressProc)(int PluginNr,char* SourceName, char* TargetName,int PercentDone);
// typedef void (DCPCALL *tLogProc)(int PluginNr,int MsgType,char* LogString);
// typedef BOOL (DCPCALL *tRequestProc)(int PluginNr,int RequestType,char* CustomTitle, char* CustomText,char* ReturnedText,int maxlen);
// typedef int BOOL;

// thread_local!(static G_PLUGIN_NO: RefCell<Option<c_int>>  = RefCell::new(None));
thread_local!(static G_PROGRESS_PROC: RefCell<Option<TProgressProc> >  = RefCell::new(None));
thread_local!(static G_LOG_PROC: RefCell<Option<TLogProc> >  = RefCell::new(None));
thread_local!(static G_REQUEST_PROC: RefCell<Option<TRequestProc> >  = RefCell::new(None));

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsInit(
    plugin_nr: c_int,
    p_progress_proc: TProgressProc,
    p_log_proc: TLogProc,
    p_request_proc: TRequestProc,
) -> c_int {
    eprintln!("FsInit enter");
    let mut shared_data = GLOBAL_SHARED_DATA.lock().unwrap();
    shared_data.plugin_nr = plugin_nr;

    G_PROGRESS_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_progress_proc);
    });
    G_LOG_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_log_proc);
    });
    G_REQUEST_PROC.with(|funcptr| {
        *funcptr.borrow_mut() = Some(p_request_proc);
    });
    unsafe {
        G_LOG_PROC.with(|logger_opt| match *logger_opt.borrow() {
            Some(logger) => {
                let c_string = CString::new("FsInit logger").expect("CString::new failed");
                let ptr = c_string.into_raw();
                logger(plugin_nr, 0, ptr);
                let _ = CString::from_raw(ptr as *mut _);
            }
            None => eprintln!("FsInit local"),
        });
    }
    tracing_subscriber::fmt::init();
    eprintln!("FsInit exit");

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsFindFirst(
    path: *mut c_char,
    find_data: *mut WIN32_FIND_DATAA,
) -> HANDLE {
    eprintln!("FsFindFirst enter");
    unsafe {
        let path_str = CStr::from_ptr(path).to_string_lossy();
        eprintln!("FsFindFirst on path {}", path_str);
        let path = Path::new(path_str.as_ref());
        let mut rit = ResourcesIteratorFactory::new(path);

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
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsFindNext(hdl: HANDLE, find_data: *mut WIN32_FIND_DATAA) -> c_int {
    eprintln!("FsFindNext enter");
    let ret_val: c_int = {
        if hdl != INVALID_HANDLE {
            let riit = hdl as *mut Box<dyn FsDataHandler>;
            //= hdl as *mut Box<ResourcesIterator>;
            //let mut riit: ManuallyDrop<Box<ResourcesIterator>> = unsafe { (hdl as ManuallyDrop<Box<ResourcesIterator>>) };
            //as *mut ResourcesIterator;
            unsafe {
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
            }
        } else {
            0
        }
    };

    eprintln!("FsFindNext exit");
    ret_val
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsFindClose(hdl: HANDLE) -> c_int {
    eprintln!("FsFindClose enter");
    //let riit: &mut ResourcesIterator = unsafe { &mut *(hdl as *mut ResourcesIterator) };
    //    let mdrit: &mut ManuallyDrop<ResourcesIterator> = unsafe { &mut *(hdl as *mut ManuallyDrop<ResourcesIterator>) };
    //    ManuallyDrop::into_inner(&mdrit);
    if hdl != INVALID_HANDLE {
        unsafe {
            let _ = Box::from_raw(hdl as *mut Box<dyn FsDataHandler>);
        }
    }
    eprintln!("FsFindClose exit");
    FS_FILE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsMkDir(_path: *mut c_char) -> BOOL {
    eprintln!("FsMkDir enter");
    eprintln!("FsMkDir exit");
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsRemoveDir(_remote_name: *mut c_char) -> BOOL {
    eprintln!("FsRemoveDir enter");
    eprintln!("FsRemoveDir exit");
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsRenMovFile(
    _old_name: *mut c_char,
    _new_name: *mut c_char,
    _mmove: BOOL,
    _over_write: BOOL,
    _ri: *mut RemoteInfoStruct,
) -> c_int {
    eprintln!("FsRenMovFile enter");
    eprintln!("FsRenMovFile exit");
    FS_FILE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsGetFile(
    _remote_name: *mut c_char,
    _local_name: *mut c_char,
    _copy_flags: c_int,
    _ri: *mut RemoteInfoStruct,
) -> c_int {
    eprintln!("FsGetFile enter");
    unsafe {
        let remote_name_str = CStr::from_ptr(_remote_name).to_string_lossy();
        let local_name_str = CStr::from_ptr(_local_name).to_string_lossy();

        eprintln!(
            "FsGetFile on remote name {} into local_name {} flags {}",
            remote_name_str, local_name_str, _copy_flags
        );

        let remote_path = Path::new(remote_name_str.as_ref());
        let local_path = Path::new(local_name_str.as_ref());

        if _copy_flags == 0 && local_path.exists() {
            eprintln!("FsGetFile exit FS_FILE_EXISTSRESUMEALLOWED");
            FS_FILE_EXISTSRESUMEALLOWED
        } else {
            let rit = ResourcesIteratorFactory::new(&remote_path);
            let ret_val = rit.fs_get(&remote_path, &local_path, _copy_flags);
            if (ret_val.is_ok()) {
                eprintln!("FsGetFile exit FS_FILE_OK");
                FS_FILE_OK
            } else {
                eprintln!("FsGetFile exit FS_FILE_READERROR");
                FS_FILE_READERROR
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsPutFile(
    _local_name: *mut c_char,
    _remote_name: *mut c_char,
    _copy_flags: c_int,
) -> c_int {
    eprintln!("FsPutFile enter");
    unsafe {
        let remote_name_str = CStr::from_ptr(_remote_name).to_string_lossy();
        let local_name_str = CStr::from_ptr(_local_name).to_string_lossy();

        eprintln!(
            "FsPutFile on remote name {} into local_name {} flags {}",
            remote_name_str, local_name_str, _copy_flags
        );

        let remote_path = Path::new(remote_name_str.as_ref());
        let local_path = Path::new(local_name_str.as_ref());

        if _copy_flags == 0 && remote_path.exists() {
            eprintln!("FsPutFile exit FS_FILE_EXISTSRESUMEALLOWED");
            FS_FILE_EXISTSRESUMEALLOWED
        } else {
            let rit = ResourcesIteratorFactory::new(&remote_path);
            let ret_val = rit.fs_put(&remote_path, &local_path, _copy_flags);
            if (ret_val.is_ok()) {
                eprintln!("FsPutFile exit FS_FILE_OK");
                FS_FILE_OK
            } else {
                eprintln!("FsPutFile exit FS_FILE_WRITEERROR");
                FS_FILE_WRITEERROR
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsExecuteFile(
    _main_win: HWND,
    _remote_name: *mut c_char,
    _verb: *mut c_char,
) -> c_int {
    unsafe {
        eprintln!("FsExecuteFile enter");
        let remote_name_str = CStr::from_ptr(_remote_name).to_string_lossy();
        let verb_str = CStr::from_ptr(_verb).to_string_lossy();

        eprintln!(
            "FsExecuteFile on path {} with verb {}",
            remote_name_str, verb_str
        );
        let path = Path::new(remote_name_str.as_ref());
        let rit = ResourcesIteratorFactory::new(&path);
        let ret_val = rit.fs_execute(&path, verb_str.as_ref());
        if (ret_val.is_ok()) {
            eprintln!("FsExecuteFile exit FS_EXEC_OK");
            FS_EXEC_OK
        } else {
            eprintln!("FsExecuteFile exit FS_EXEC_ERROR");
            FS_EXEC_ERROR
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsDeleteFile(_remote_name: *mut c_char) -> BOOL {
    eprintln!("FsDeleteFile enter");
    unsafe {
        let remote_name_str = CStr::from_ptr(_remote_name).to_string_lossy();

        eprintln!("FsDeleteFile on remote name {}", remote_name_str);

        let remote_path = Path::new(remote_name_str.as_ref());

        let rit = ResourcesIteratorFactory::new(&remote_path);
        let ret_val = rit.fs_delete(&remote_path);
        if (ret_val.is_ok()) {
            eprintln!("FsDeleteFile exit FS_FILE_OK");
            FS_FILE_OK
        } else {
            eprintln!("FsDeleteFile exit FS_FILE_WRITEERROR");
            FS_FILE_WRITEERROR
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsSetTime(
    _remote_name: *mut c_char,
    _creation_time: *mut FILETIME,
    _last_access_time: *mut FILETIME,
    _last_write_time: *mut FILETIME,
) -> BOOL {
    eprintln!("FsSetTime enter");
    eprintln!("FsSetTime exit");
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsDisconnect(_disconnect_root: *mut c_char) -> BOOL {
    eprintln!("FsDisconnect enter");
    eprintln!("FsDisconnect exit");
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsSetDefaultParams(dps: *mut FsDefaultParamStruct) {
    eprintln!("FsSetDefaultParams enter");
    unsafe {
        let str = CStr::from_ptr((*dps).default_ini_name.as_ptr()).to_string_lossy();
        eprintln!(
            "FsSetDefaultParams  {} version {}:{} size {}",
            str,
            (*dps).plugin_interface_version_hi,
            (*dps).plugin_interface_version_low,
            (*dps).size
        );
    }
    eprintln!("FsSetDefaultParams exit");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FsGetDefRootName(def_root_name: *mut c_char, maxlen: c_int) {
    eprintln!("FsGetDefRootName enter");
    let plugin_name = "k8s";
    let bytes = plugin_name.as_bytes();
    let len = bytes.len();
    unsafe {
        std::ptr::copy(
            plugin_name.as_bytes().as_ptr().cast(),
            def_root_name,
            maxlen as usize,
        );
        std::ptr::write(def_root_name.offset(len as isize) as *mut u8, 0u8);
    }
    eprintln!("FsGetDefRootName exit");
}
