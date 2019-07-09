use shared::{
    minwindef::{DWORD, BYTE},
    ntdef::{LPCWSTR, LPWSTR},
};
use um::wincrypt::CRYPT_DATA_BLOB;

STRUCT! {struct SPC_SP_OPUS_INFO {
    pwszProgramName: LPCWSTR,
    pMoreInfo: *mut SPC_LINK_,
    pPublisherInfo: *mut SPC_LINK_,
}}

STRUCT! {struct SPC_LINK_ {
    dwLinkChoice: DWORD,
    u: SPC_LINK_u,
}}

UNION!{union SPC_LINK_u {
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
