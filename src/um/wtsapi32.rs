use shared::{
    minwindef::BOOL,
    ntdef::{PHANDLE, ULONG},
};

extern "system" {
    pub fn WTSQueryUserToken(SessionId: ULONG, phToken: PHANDLE) -> BOOL;
}
