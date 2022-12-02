use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub const FS_FILE_OK: c_int = 0;
const FS_FILE_EXISTS: c_int = 1;
const FS_FILE_NOTFOUND: c_int = 2;
const FS_FILE_READERROR: c_int = 3;
const FS_FILE_WRITEERROR: c_int = 4;
const FS_FILE_USERABORT: c_int = 5;
const FS_FILE_NOTSUPPORTED: c_int = 6;
const FS_FILE_EXISTSRESUMEALLOWED: c_int = 7;
const FS_EXEC_OK: c_int = 0;
const FS_EXEC_ERROR: c_int = 1;
const FS_EXEC_YOURSELF: c_int = -1;
const FS_EXEC_SYMLINK: c_int = -2;
const FS_COPYFLAGS_OVERWRITE: c_int = 1;
const FS_COPYFLAGS_RESUME: c_int = 2;
const FS_COPYFLAGS_MOVE: c_int = 4;
const FS_COPYFLAGS_EXISTS_SAMECASE: c_int = 8;
const FS_COPYFLAGS_EXISTS_DIFFERENTCASE: c_int = 16;

// flags for tRequestProc
const RT_Other: c_int = 0;
const RT_UserName: c_int = 1;
const RT_Password: c_int = 2;
const RT_Account: c_int = 3;
const RT_UserNameFirewall: c_int = 4;
const RT_PasswordFirewall: c_int = 5;
const RT_TargetDir: c_int = 6;
const RT_URL: c_int = 7;
const RT_MsgOK: c_int = 8;
const RT_MsgYesNo: c_int = 9;
const RT_MsgOKCancel: c_int = 10;

// flags for tLogProc
const MSGTYPE_CONNECT: c_int = 1;
const MSGTYPE_DISCONNECT: c_int = 2;
const MSGTYPE_DETAILS: c_int = 3;
const MSGTYPE_TRANSFERCOMPLETE: c_int = 4;
const MSGTYPE_CONNECTCOMPLETE: c_int = 5;
const MSGTYPE_IMPORTANTERROR: c_int = 6;
const MSGTYPE_OPERATIONCOMPLETE: c_int = 7;

// flags for FsStatusInfo
const FS_STATUS_START: c_int = 0;
const FS_STATUS_END: c_int = 1;
const FS_STATUS_OP_LIST: c_int = 1;
const FS_STATUS_OP_GET_SINGLE: c_int = 2;
const FS_STATUS_OP_GET_MULTI: c_int = 3;
const FS_STATUS_OP_PUT_SINGLE: c_int = 4;
const FS_STATUS_OP_PUT_MULTI: c_int = 5;
const FS_STATUS_OP_RENMOV_SINGLE: c_int = 6;
const FS_STATUS_OP_RENMOV_MULTI: c_int = 7;
const FS_STATUS_OP_DELETE: c_int = 8;
const FS_STATUS_OP_ATTRIB: c_int = 9;
const FS_STATUS_OP_MKDIR: c_int = 10;
const FS_STATUS_OP_EXEC: c_int = 11;
const FS_STATUS_OP_CALCSIZE: c_int = 12;
const FS_STATUS_OP_SEARCH: c_int = 13;
const FS_STATUS_OP_SEARCH_TEXT: c_int = 14;
const FS_STATUS_OP_SYNC_SEARCH: c_int = 15;
const FS_STATUS_OP_SYNC_GET: c_int = 16;
const FS_STATUS_OP_SYNC_PUT: c_int = 17;
const FS_STATUS_OP_SYNC_DELETE: c_int = 18;
const FS_ICONFLAG_SMALL: c_int = 1;
const FS_ICONFLAG_BACKGROUND: c_int = 2;
const FS_ICON_USEDEFAULT: c_int = 0;
const FS_ICON_EXTRACTED: c_int = 1;
const FS_ICON_EXTRACTED_DESTROY: c_int = 2;
const FS_ICON_DELAYED: c_int = 3;
const FS_BITMAP_NONE: c_int = 0;
const FS_BITMAP_EXTRACTED: c_int = 1;
const FS_BITMAP_EXTRACT_YOURSELF: c_int = 2;
const FS_BITMAP_EXTRACT_YOURSELF_ANDDELETE: c_int = 3;
const FS_BITMAP_CACHE: c_int = 256;
const FS_CRYPT_SAVE_PASSWORD: c_int = 1;
const FS_CRYPT_LOAD_PASSWORD: c_int = 2;
const FS_CRYPT_LOAD_PASSWORD_NO_UI: c_int = 3; // Load password only if master password has already been entered!
const FS_CRYPT_COPY_PASSWORD: c_int = 4; // Copy encrypted password to new connection name
const FS_CRYPT_MOVE_PASSWORD: c_int = 5; // Move password when renaming a connection
const FS_CRYPT_DELETE_PASSWORD: c_int = 6; // Delete password
const FS_CRYPTOPT_MASTERPASS_SET: c_int = 1; // The user already has a master password defined

// flags for FsFindFirst/FsFindNext
const FILE_ATTRIBUTE_DIRECTORY: c_uint = 0x00000010;
const FILE_ATTRIBUTE_REPARSE_POINT: c_uint = 0x00000400;
const FILE_ATTRIBUTE_UNIX_MODE: c_uint = 0x80000000;

pub type TProgressProc = unsafe extern "C" fn(c_int, *mut c_char, *mut c_char, c_int) -> c_int;
pub type TLogProc = unsafe extern "C" fn(c_int, c_int, *mut c_char) -> c_void;
pub type TRequestProc =
    unsafe extern "C" fn(c_int, c_int, *mut c_char, *mut c_char, *mut c_char, c_int) -> c_int;

type HANDLE = *mut c_void;
pub const INVALID_HANDLE: HANDLE = -1isize as HANDLE; // {0xffffffffffffffff as *mut core::ffi::c_void}

pub type DWORD = c_uint;
struct FILETIME {
    dw_low_date_time: DWORD,
    dw_high_date_time: DWORD,
}

// typedef struct _WIN32_FIND_DATAA {
// 	DWORD dwFileAttributes;
// 	FILETIME ftCreationTime;
// 	FILETIME ftLastAccessTime;
// 	FILETIME ftLastWriteTime;
// 	DWORD nFileSizeHigh;
// 	DWORD nFileSizeLow;
// 	DWORD dwReserved0;
// 	DWORD dwReserved1;
// 	CHAR cFileName[MAX_PATH];
// 	CHAR cAlternateFileName[14];
// } WIN32_FIND_DATAA,*LPWIN32_FIND_DATAA;

pub const MAX_PATH: usize = 260;

struct WIN32_FIND_DATAA {
    dwFileAttributes: DWORD,
    ftCreationTime: FILETIME,
    ftLastAccessTime: FILETIME,
    ftLastWriteTime: FILETIME,
    nFileSizeHigh: DWORD,
    nFileSizeLow: DWORD,
    dwReserved0: DWORD,
    dwReserved1: DWORD,
    cFileName: [c_char; MAX_PATH],
    cAlternateFileName: [c_char; 14],
}
