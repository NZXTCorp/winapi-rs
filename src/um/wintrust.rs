use shared::{
    guiddef::GUID,
    minwindef::{BYTE, DWORD, LPVOID},
    ntdef::{HANDLE, LPCWSTR, LPWSTR, WCHAR},
    windef::{HRESULT, HWND},
};
use um::wincrypt::CRYPT_DATA_BLOB;

STRUCT! {struct SPC_SP_OPUS_INFO {
    pwszProgramName: LPCWSTR,
    pMoreInfo: *mut SPC_LINK_,
    pPublisherInfo: *mut SPC_LINK_,
}}

pub const SPC_URL_LINK_CHOICE: DWORD = 1;
pub const SPC_MONIKER_LINK_CHOICE: DWORD = 2;
pub const SPC_FILE_LINK_CHOICE: DWORD = 3;

STRUCT! {struct SPC_LINK_ {
    dwLinkChoice: DWORD,
    u: SPC_LINK_u,
}}

UNION! {union SPC_LINK_u {
    [u32; 6] [u64; 4],
    pwszUrl pwszUrl_mut: LPWSTR,
    Moniker Moniker_mut: SPC_SERIALIZED_OBJECT,
    pwszFile pwszFile_mut: LPWSTR,
}}

STRUCT! {struct SPC_SERIALIZED_OBJECT {
    ClassId: SPC_UUID,
    SerializedData: CRYPT_DATA_BLOB,
}}

pub const SPC_UUID_LENGTH: usize = 16;

pub type SPC_UUID = [BYTE; SPC_UUID_LENGTH];

STRUCT! {struct WINTRUST_DATA {
    cbStruct: DWORD,
    pPolicyCallbackData: LPVOID,
    pSIPClientData: LPVOID,
    dwUIChoice: DWORD,
    fdwRevocationChecks: DWORD,
    dwUnionChoice: DWORD,
    u: WINTRUST_DATA_u,
    dwStateAction: DWORD,
    hWVTStateData: HANDLE,
    pwszURLReference: *mut WCHAR,
    dwProvFlags: DWORD,
    dwUIContext: DWORD,
    // #if (NTDDI_VERSION >= NTDDI_WIN8)
    // pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}}

type PWINTRUST_DATA = *mut WINTRUST_DATA;

UNION!{union WINTRUST_DATA_u {
    [u32; 1] [u64; 1],
    pFile pFile_mut: *mut WINTRUST_FILE_INFO,
    /*
    pCatalog pCatalog_mut: *mut WINTRUST_CATALOG_INFO,
    pBlob pBlob_mut: *mut WINTRUST_BLOB_INFO,
    pSgnr pSgnr_mut: *mut WINTRUST_SGNR_INFO,
    pCert pCert_mut: *mut WINTRUST_CERT_INFO,
    */
}}

const WTD_UI_ALL: DWORD = 1;
const WTD_UI_NONE: DWORD = 2;
const WTD_UI_NOBAD: DWORD = 3;
const WTD_UI_NOGOOD: DWORD = 4;

const WTD_REVOKE_NONE: DWORD = 0x00000000;
const WTD_REVOKE_WHOLECHAIN: DWORD = 0x00000001;

const WTD_CHOICE_FILE: DWORD = 1;
const WTD_CHOICE_CATALOG: DWORD = 2;
const WTD_CHOICE_BLOB: DWORD = 3;
const WTD_CHOICE_SIGNER: DWORD = 4;
const WTD_CHOICE_CERT: DWORD = 5;

const WTD_STATEACTION_IGNORE: DWORD = 0x00000000;
const WTD_STATEACTION_VERIFY: DWORD = 0x00000001;
const WTD_STATEACTION_CLOSE: DWORD = 0x00000002;
const WTD_STATEACTION_AUTO_CACHE: DWORD = 0x00000003;
const WTD_STATEACTION_AUTO_CACHE_FLUSH: DWORD = 0x00000004;

const WTD_PROV_FLAGS_MASK: DWORD = 0x0000FFFF;
const WTD_USE_IE4_TRUST_FLAG: DWORD = 0x00000001;
const WTD_NO_IE4_CHAIN_FLAG: DWORD = 0x00000002;
const WTD_NO_POLICY_USAGE_FLAG: DWORD = 0x00000004;
const WTD_REVOCATION_CHECK_NONE: DWORD = 0x00000010;
const WTD_REVOCATION_CHECK_END_CERT: DWORD = 0x00000020;
const WTD_REVOCATION_CHECK_CHAIN: DWORD = 0x00000040;
const WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: DWORD = 0x00000080;
const WTD_SAFER_FLAG: DWORD = 0x00000100;
const WTD_HASH_ONLY_FLAG: DWORD = 0x00000200;
const WTD_USE_DEFAULT_OSVER_CHECK: DWORD = 0x00000400;
const WTD_LIFETIME_SIGNING_FLAG: DWORD = 0x00000800;
const WTD_CACHE_ONLY_URL_RETRIEVAL: DWORD = 0x00001000; // affects CRL retrieval and AIA retrieval
const WTD_DISABLE_MD2_MD4: DWORD = 0x00002000;
const WTD_MOTW: DWORD = 0x00004000; // Mark-Of-The-Web
const WTD_CODE_INTEGRITY_DRIVER_MODE: DWORD = 0x00008000; // Code Integrity driver mode

const WTD_UICONTEXT_EXECUTE: DWORD = 0;
const WTD_UICONTEXT_INSTALL: DWORD = 1;

STRUCT! {struct WINTRUST_FILE_INFO {
    cbStruct: DWORD,
    pcwszFilePath: LPCWSTR,
    hFile: HANDLE,
    pgKnownSubject: *const GUID,
}}

extern "system" {
    fn WinVerifyTrust(hwnd: HWND, pgActionID: *mut GUID, pWVTData: LPVOID) -> HRESULT;
}
