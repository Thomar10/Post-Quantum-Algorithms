use core::ops::Sub;

use rand::{Rng, thread_rng};

use falcon::falcon::fpr;
use falcon::fpr::{fpr_add as add, fpr_div as div, fpr_double as double, fpr_expm_p63 as expm_p63, fpr_floor as floor, fpr_half as half, fpr_inv as inv, fpr_lt as lt, fpr_mul as mul, fpr_neg as neg, fpr_of, fpr_rint as rint, fpr_sqrt as sqrt, fpr_sub as sub, fpr_trunc as trunc};

static A_HALF: fpr = 4602678819172646912;
static A_HALF_NEG: fpr = 13826050856027422720;
static SIGN_BIT: fpr = 1 << 63;

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
    let d = fpr_inv(y);
    fpr_mul(x, &d)
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

    println!("{}", fpr_to_double(x[0]));
    println!("{}", fpr_to_double(x[1]));
    // d[0] = floor(x[0]);
    // d[1] = floor(x[1]);
    // println!("d {}", floor(x[0]));
    // println!("dr {}", rint(x[0]));
    // println!("d1 {}", floor(x[1]));
    // println!("d1r {}", rint(x[1]));
    // println!("{}", d[0] + d[1]);
    //First pos second neg
    if !(x[0] & SIGN_BIT > 0) && (x[1] & SIGN_BIT > 0) {
        println!("First pos second neg");
        println!("d {}", floor(x[0]));
        println!("dr {}", rint(x[0]));
        println!("d1 {}", floor(x[1]));
        println!("d1r {}", rint(x[1]));
        println!();
        d[0] = floor(x[0]);//if x[0] & A_HALF > A_HALF { rint(x[0]) } else { floor(x[0]) };
        d[1] = rint(x[1]);//if x[1] & A_HALF_NEG > A_HALF_NEG { rint(x[1]) } else { floor(x[1]) };
        println!("d0 {}", d[0]);
        println!("d1 {}", d[1]);
        println!("pn {}", d[0] + d[1]);
    }
    //First negative second pos
    else if (x[0] & SIGN_BIT > 0) && !(x[1] & SIGN_BIT > 0) {
        println!("First neg second pos");
        d[0] = rint(x[0]);//if x[0] & A_HALF_NEG > A_HALF_NEG { floor(x[0]) } else { rint(x[0]) };
        d[1] = floor(x[1]);//if x[1] & A_HALF > A_HALF { rint(x[1]) } else { floor(x[1]) };
        println!("d0 {}", d[0]);
        println!("d1 {}", d[1]);
        println!("np {}", d[0] + d[1]);
    }
    // Both negative
    else if (x[0] & SIGN_BIT > 0) && (x[1] & SIGN_BIT > 0) {
        println!("negative both");
        println!("d {}", floor(x[0]));
        println!("dr {}", rint(x[0]));
        println!("d1 {}", floor(x[1]));
        println!("d1r {}", rint(x[1]));
        println!();
        if (!(x[0] & A_HALF_NEG < A_HALF_NEG)) && (!(x[1] & A_HALF_NEG < A_HALF_NEG)) {
            println!("BG half");
            d[0] = floor(x[0]);
            d[1] = floor(x[1]);
        } else if (!(x[0] & A_HALF_NEG < A_HALF_NEG)) && (x[1] & A_HALF_NEG < A_HALF_NEG) {
            println!("FG half");
            d[0] = floor(x[0]);
            d[1] = rint(x[1]);
        } else if (x[0] & A_HALF_NEG < A_HALF_NEG) && (!(x[1] & A_HALF_NEG < A_HALF_NEG)){
            println!("SG half");
            d[0] = rint(x[0]);
            d[1] = floor(x[1]);
        } else {
            println!("NG half");
            d[0] = floor(x[0]);
            d[1] = floor(x[1]);
        }
        println!("d0 {}", d[0]);
        println!("d1 {}", d[1]);
        println!("neg {}", d[0] + d[1]);
    }
    // Both positive
    else {
        println!("positive both");
        println!("d {}", floor(x[0]));
        println!("dr {}", rint(x[0]));
        println!("d1 {}", floor(x[1]));
        println!("d1r {}", rint(x[1]));
        println!();
        d[0] = if x[0] & A_HALF > A_HALF { rint(x[0]) } else { floor(x[0]) };
        d[1] = if x[1] & A_HALF > A_HALF { rint(x[1]) } else { floor(x[1]) };
        println!("d0 {}", d[0]);
        println!("d1 {}", d[1]);
        println!("pos {}", d[0] + d[1]);
    }
    d
}

pub fn fpr_to_double(x: fpr) -> f64 {
    return f64::from_bits(x);
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
    let mut rng = thread_rng();
    let r1: fpr = f64::to_bits(rng.gen_range(-100f64..100f64));
    let share_two: fpr = f64::to_bits(rng.gen_range(-100f64..100f64));
    let share_one = sub(r1, share_two);
    let y = fpr_mul(&[share_one, share_two], x);
    let y_open_inv = inv(add(y[0], y[1]));
    d[0] = mul(share_one, y_open_inv);
    d[1] = mul(share_two, y_open_inv);
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