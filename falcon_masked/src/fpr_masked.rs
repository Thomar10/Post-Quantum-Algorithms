use falcon::falcon::fpr;
use falcon::fpr::{fpr_add as add, fpr_div as div, fpr_double as double,
                  fpr_expm_p63 as expm_p63, fpr_floor as floor, fpr_half as half,
                  fpr_inv as inv, fpr_lt as lt, fpr_mul as mul,
                  fpr_neg as neg, fpr_rint as rint, fpr_sqrt as sqrt,
                  fpr_sub as sub, fpr_trunc as trunc};

pub fn fpr_add(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = add(x[0], y[0]);
    d[1] = add(x[1], y[1]);
    d
}

#[inline(always)]
pub fn fpr_sub(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = sub(x[0], y[0]);
    d[1] = sub(x[1], y[1]);
    d
}

pub fn fpr_expm_p63(x: &[fpr], ccs: &[fpr]) -> [u64; 2] {
    let mut d = [0; 2];
    d[0] = expm_p63(x[0], ccs[0]);
    d[1] = expm_p63(x[1], ccs[1]);
    d
}

pub fn fpr_mul(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = mul(x[0], y[0]);
    d[1] = add(mul(x[1], y[0]),
               add(mul(y[1], x[0]), mul(x[1], y[1])));
    d
}

pub fn fpr_sqrt(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = sqrt(x[0]);
    d[1] = sqrt(x[1]);
    d
}

#[inline(always)]
pub fn fpr_trunc(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = trunc(x[0]);
    d[1] = trunc(x[1]);
    d
}


pub fn fpr_div(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = div(x[0], y[0]);
    d[1] = add(div(x[1], y[0]),
               div(y[1], x[0]));
    d
}


#[inline(always)]
pub fn fpr_rint(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = rint(x[0]);
    d[1] = rint(x[1]);
    d
}

#[inline(always)]
pub fn fpr_floor(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = floor(x[0]);
    d[1] = floor(x[1]);
    d
}


#[inline(always)]
pub fn fpr_neg(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = neg(x[0]);
    d[1] = neg(x[1]);
    d
}

#[inline(always)]
pub fn fpr_half(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = half(x[0]);
    d[1] = half(x[1]);
    d
}

#[inline(always)]
pub fn fpr_double(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = double(x[0]);
    d[1] = double(x[1]);
    d
}

#[inline(always)]
pub fn fpr_inv(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = inv(x[0]);
    d[1] = inv(x[1]);
    d
}


#[inline(always)]
pub fn fpr_lt(x: &[fpr], y: fpr) -> i32 {
    let xx = add(x[0], x[1]);
    lt(xx, y)
}

#[inline(always)]
pub fn fpr_sqr(x: &[fpr]) -> [fpr; 2] {
    fpr_mul(x, x)
}