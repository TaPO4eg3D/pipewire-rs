use super::*;

extern "C" {
    #[link_name = "libspa_rs_meta_check"]
    pub fn spa_meta_check(p: *const std::ffi::c_void, m: *const spa_meta) -> bool;
}
