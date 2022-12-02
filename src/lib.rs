use libc;
use std::cell::RefCell;
use std::ffi::c_int;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::sync::Once;

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
thread_local!(static G_PROGRESS_PROC: RefCell<Option<consts::TProgressProc> >  = RefCell::new(None));
thread_local!(static G_LOG_PROC: RefCell<Option<consts::TLogProc> >  = RefCell::new(None));
thread_local!(static G_REQUEST_PROC: RefCell<Option<consts::TRequestProc> >  = RefCell::new(None));

#[no_mangle]
pub unsafe extern "C" fn FsInit(
    plugin_nr: c_int,
    p_progress_proc: consts::TProgressProc,
    p_log_proc: consts::TLogProc,
    p_request_proc: consts::TRequestProc,
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

    0
}

// #[no_mangle]
// pub unsafe extern "C" fn FsFindFirst(char* Path, WIN32_FIND_DATAA *FindData) -> c_void{

// }

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
