#[link(name = "vrfy", kind = "static")]
extern "C" {
    pub fn mq_add(x: u32, y: u32) -> u32;
    pub fn mq_sub(x: u32, y: u32) -> u32;
    pub fn mq_rshift1(x: u32) -> u32;
    pub fn mq_montymul(x: u32, y: u32) -> u32;
    pub fn mq_montysqr(x: u32) -> u32;
    pub fn mq_div_12289(x: u32, y: u32) -> u32;
    pub fn mq_NTT(a: *const u16, logn: u32);
    pub fn mq_iNTT(a: *const u16, logn: u32);
    pub fn mq_poly_tomonty(f: *const u16, logn: u32);
    pub fn mq_poly_montymul_ntt(f: *const u16, g: *const u16, logn: u32);
    pub fn mq_poly_sub(f: *const u16, g: *const u16, logn: u32);
    pub fn to_ntt_monty(a: *const u16, logn: u32);
    pub fn verify_raw(c0: *const u16, s2: *const i16, h: *const u16, logn: u32, tmp: *const u8);
    pub fn compute_public(f: *const i32, g: *const i32, logn: u32, tmp: *const u8);
    pub fn complete_private(f: *const i32, g: *const i32, F: *const i32, logn: u32, tmp: *const u8);
    pub fn is_invertible(s2: *const i16, logn: u32, tmp: *const u8);
    pub fn verify_recover(h: *const u16, c0: *const u16, s1: *const i16, s2: *const i16, logn: u32, tmp: *const u8);
    pub fn count_nttzero(sig: *const i16, logn: u32, tmp: *const u8);
}