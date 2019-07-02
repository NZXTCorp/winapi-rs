use shared::{
    minwindef::{BOOL, DWORD, FARPROC, HMODULE},
    ntdef::LPCSTR,
};
pub const _DELAY_IMP_VER: i32 = 2;
pub type RVA = DWORD;
STRUCT!{struct ImgDelayDescr {
    grAttrs: DWORD,
    rvaDLLName: RVA,
    rvaHmod: RVA,
    rvaIAT: RVA,
    rvaINT: RVA,
    rvaBoundIAT: RVA,
    rvaUnloadIAT: RVA,
    dwTimeStamp: DWORD,
}}
pub type PImgDelayDescr = *mut ImgDelayDescr;
pub type PCImgDelayDescr = *const ImgDelayDescr;
ENUM!{enum DLAttr {
    dlattrRva = 0x1,
}}
pub const dliStartProcessing: u32 = 0;
pub const dliNodeStartProcessing: u32 = dliStartProcessing;
pub const dliNotePreLoadLibrary: u32 = 1;
pub const dliNotePreGetProcAddress: u32 = 2;
pub const dliFailLoadLib: u32 = 3;
pub const dliFailGetProc: u32 = 4;
pub const dliNoteEndProcessing: u32 = 5;
STRUCT!{struct DelayLoadProc {
    fImportByName: BOOL,
    u: DelayLoadProc_u,
}}
UNION!{union DelayLoadProc_u {
    [u32; 1] [u64; 1],
    szProcName szProc_mut: LPCSTR,
    dwOrdinal dwOrdinal_mut: DWORD,
}}
STRUCT!{struct DelayLoadInfo {
    cb: DWORD,
    pidd: PCImgDelayDescr,
    ppfn: *mut FARPROC,
    szDll: LPCSTR,
    dlp: DelayLoadProc,
    hmodCur: HMODULE,
    pfnCur: FARPROC,
    dwLastError: DWORD,
}}
pub type PDelayLoadInfo = *mut DelayLoadInfo;
FN!{stdcall PfnDliHook(
    dliNotify: u32,
    pdli: PDelayLoadInfo,
) -> FARPROC}
