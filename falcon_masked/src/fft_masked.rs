use falcon::falcon::fpr;
use crate::fpr_masked::{fpr_add, fpr_double, fpr_half, fpr_inv, fpr_mul, fpr_neg, fpr_sqr, fpr_sub, FPR_GM_TAB, FPR_P2_TAB, FPR_ZERO};

pub fn fpc_add(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let fpct_re: [fpr; 2] = fpr_add(a_re, b_re);
    let fpct_im: [fpr; 2] = fpr_add(a_im, b_im);
    return (fpct_re, fpct_im);
}

pub fn fpc_sub(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let fpct_re: [fpr; 2] = fpr_sub(a_re, b_re);
    let fpct_im: [fpr; 2] = fpr_sub(a_im, b_im);
    return (fpct_re, fpct_im);
}

pub fn fpc_mul(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let fpct_d_re: [fpr; 2] = fpr_sub(
        &fpr_mul(a_re, b_re),
        &fpr_mul(a_im, b_im));
    let fpct_d_im: [fpr; 2] = fpr_add(
        &fpr_mul(a_re, b_im),
        &fpr_mul(a_im, b_re));
    return (fpct_d_re, fpct_d_im);
}

pub fn fpc_div(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let mut fpct_m: [fpr; 2] = fpr_add(&fpr_sqr(b_re), &fpr_sqr(b_im));
    fpct_m = fpr_inv(&fpct_m);
    let b_re: [fpr; 2] = fpr_mul(b_re, &fpct_m);
    let b_im: [fpr; 2] = fpr_mul(&fpr_neg(b_im), &fpct_m);
    let fpct_d_re: [fpr; 2] = fpr_sub(
        &fpr_mul(a_re, &b_re),
        &fpr_mul(a_im, &b_im));
    let fpct_d_im: [fpr; 2] = fpr_add(
        &fpr_mul(a_re, &b_im),
        &fpr_mul(a_im, &b_re));
    (fpct_d_re, fpct_d_im)
}

pub fn fpc_sqr(a_re: &[fpr], a_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let fpct_d_re: [fpr; 2] = fpr_sub(&fpr_sqr(a_re), &fpr_sqr(a_im));
    let fpct_d_im: [fpr; 2] = fpr_double(&fpr_mul(a_re, a_im));
    (fpct_d_re, fpct_d_im)
}

pub fn fpc_inv(a_re: &[fpr], a_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let mut fpct_m: [fpr; 2] = fpr_add(&fpr_sqr(a_re), &fpr_sqr(a_im));
    fpct_m = fpr_inv(&fpct_m);
    let fpct_d_re: [fpr; 2] = fpr_mul(a_re, &fpct_m);
    let fpct_d_im: [fpr; 2] = fpr_mul(&fpr_neg(a_im), &fpct_m);
    (fpct_d_re, fpct_d_im)
}

//Column order - Don't know which is best right now
pub fn fft(f: &mut [[fpr; 2]], logn: u32) {
    let mut u: u32 = 1;
    let mut m: usize = 2;
    let (mut t, n, hn): (usize, usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    t = hn;

    while u < logn {
        let (ht, hm, mut i1, mut j1): (usize, usize, usize, usize);
        ht = t >> 1;
        hm = m >> 1;
        i1 = 0;
        j1 = 0;
        while i1 < hm {
            let (mut j, j2): (usize, usize);
            j2 = j1 + ht;
            let (s_re, s_im): ([fpr; 2], [fpr; 2]);
            s_re = [FPR_GM_TAB[((m + i1) << 1) + 0], 0];
            s_im = [FPR_GM_TAB[((m + i1) << 1) + 1], 0];
            j = j1;

            while j < j2 {
                let (x_re, x_im, mut y_re, mut y_im): ([fpr; 2], [fpr; 2], [fpr; 2], [fpr; 2]);
                x_re = f[j];
                x_im = f[j + hn];
                y_re = f[j + ht];
                y_im = f[j + ht + hn];
                (y_re, y_im) = fpc_mul(&y_re, &y_im, &s_re, &s_im);
                (f[j], f[j + hn]) = fpc_add(&x_re, &x_im, &y_re, &y_im);
                (f[j + ht], f[j + ht + hn]) = fpc_sub(&x_re, &x_im, &y_re, &y_im);

                j += 1;
            }

            i1 += 1;
            j1 += t;
        }
        u += 1;
        m <<= 1;
        t = ht;
    }
}

//Row order - Don't know which is best right now
/*pub fn m_fft_r(f: &mut [&mut [fpr]], logn: u32) {
    let mut u: u32 = 1;
    let mut m: usize = 2;
    let (mut t, n, hn): (usize, usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    t = hn;

    while u < logn {
        let (ht, hm, mut i1, mut j1): (usize, usize, usize, usize);
        ht = t >> 1;
        hm = m >> 1;
        i1 = 0;
        j1 = 0;
        while i1 < hm {
            let (mut j, j2): (usize, usize);
            j2 = j1 + ht;
            let (s_re, s_im): ([fpr; 2], [fpr; 2]);
            s_re = [FPR_GM_TAB[((m + i1) << 1) + 0]; 2];
            s_im = [FPR_GM_TAB[((m + i1) << 1) + 1]; 2];
            j = j1;

            while j < j2 {
                let (x_re, x_im, mut y_re, mut y_im): ([fpr; 2], [fpr; 2], [fpr; 2], [fpr; 2]);
                x_re = get_column(f, j);
                x_im = get_column(f, j + hn);
                y_re = get_column(f, j + ht);
                y_im = get_column(f, j + ht + hn);
                (y_re, y_im) = fpc_mul(&y_re, &y_im, &s_re, &s_im);
                let (mut res_re, mut res_im): ([fpr; 2], [fpr; 2]) = fpc_add(&x_re, &x_im, &y_re, &y_im);
                set_column(f, j,res_re);
                set_column(f, j + hn, res_im);
                (res_re, res_im) = fpc_sub(&x_re, &x_im, &y_re, &y_im);
                set_column(f, j + ht, res_re);
                set_column(f, j + ht + hn, res_im);

                j += 1;
            }

            i1 += 1;
            j1 += t;
        }
        u += 1;
        m <<= 1;
        t = ht;
    }
}

//Should probably be moved somewhere else
fn get_column(arr: &[&[fpr]], index: usize) -> [fpr; 2] {
    let mut col: [fpr; 2] = [0; 2];
    for i in 0..2 {
        col[i] = arr[index][i];
    }
    return col;
}

fn set_column(arr: &mut [&mut [fpr]], index: usize, value: [fpr; 2]) {
    for i in 0..value.len() {
        arr[index][i] = value[i];
    }
}*/

pub fn ifft(f: &mut [[fpr; 2]], logn: u32) {
    let mut u: u32 = logn;
    let mut t: usize = 1;
    let n: usize = (1 as usize) << logn;
    let hn: usize = n >> 1;
    let mut m: usize = n;
    while u > 1 {
        let (hm, dt, mut i1, mut j1): (usize, usize, usize, usize);
        hm = m >> 1;
        dt = t << 1;

        i1 = 0;
        j1 = 0;
        while j1 < hn {
            let (mut j, j2): (usize, usize);
            j2 = j1 + t;

            let (s_re, s_im): ([fpr; 2], [fpr; 2]);
            s_re = [FPR_GM_TAB[((hm + i1) << 1) + 0], 0];
            s_im = fpr_neg(&[FPR_GM_TAB[((hm + i1) << 1) + 1], 0]);
            j = j1;

            while j < j2 {
                let (mut x_re, mut x_im, y_re, y_im): ([fpr; 2], [fpr; 2], [fpr; 2], [fpr; 2]);
                x_re = f[j];
                x_im = f[j + hn];
                y_re = f[j + t];
                y_im = f[j + t + hn];
                (f[j], f[j + hn]) = fpc_add(&x_re, &x_im, &y_re, &y_im);
                (x_re, x_im) = fpc_sub(&x_re, &x_im, &y_re, &y_im);
                (f[j + t], f[j + t + hn]) = fpc_mul(&x_re, &x_im, &s_re, &s_im);
                j += 1;
            }
            i1 += 1;
            j1 += dt;
        }
        t = dt;
        m = hm;
        u -= 1;
    }

    if logn > 0 {
        let ni: [fpr; 2] = [FPR_P2_TAB[logn as usize], 0];
        let mut u = 0;
        while u < n {
            f[u] = fpr_mul(&f[u], &ni);
            u += 1;
        }
    }
}


pub fn poly_add(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_add(&a[u], &b[u]);
    }
}

pub fn poly_sub(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_sub(&a[u], &b[u]);
    }
}

pub fn poly_neg(a: &mut [[fpr; 2]], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_neg(&a[u]);
    }
}

pub fn poly_adj_fft(a: &mut [[fpr; 2]], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in (n >> 1)..n {
        a[u] = fpr_neg(&a[u]);
    }
}

pub fn poly_mul_fft(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_mul(&a[u], &a[u + hn], &b[u], &b[u + hn]);
    }
}

pub fn poly_muladj_fft(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_mul(&a[u], &a[u + hn], &b[u], &fpr_neg(&b[u + hn]));
    }
}

pub fn poly_mulselfadj_fft(a: &mut [[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        a[u] = fpr_add(&fpr_sqr(&a[u]), &fpr_sqr(&a[u + hn]));
        a[u + hn] = [FPR_ZERO; 2];
    }
}

pub fn poly_mulconst(a: &mut [[fpr; 2]], x: fpr, logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_mul(&a[u], &[x, FPR_ZERO]);
    }
}


pub fn poly_div_fft(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_div(&a[u], &a[u + hn], &b[u], &b[u + hn]);
    }
}

pub fn poly_invnorm2_fft(d: &mut [[fpr; 2]], a: &[[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;

    for u in 0..hn {
        d[u] = fpr_inv(&fpr_add(
            &fpr_add(&fpr_sqr(&a[u]), &fpr_sqr(&a[u + hn])),
            &fpr_add(&fpr_sqr(&b[u]), &fpr_sqr(&b[u + hn]))));
    }
}

#[allow(non_snake_case)]
pub fn poly_add_muladj_fft(d: &mut [[fpr; 2]], F: &[[fpr; 2]], G: &[[fpr; 2]], f: &[[fpr; 2]], g: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let (a_re, a_im) = fpc_mul(&F[u], &F[u + hn], &f[u], &fpr_neg(&f[u + hn]));
        let (b_re, b_im) = fpc_mul(&G[u], &G[u + hn], &g[u], &fpr_neg(&g[u + hn]));
        d[u] = fpr_add(&a_re, &b_re);
        d[u + hn] = fpr_add(&a_im, &b_im);
    }
}

pub fn poly_mul_autoadj_fft(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        a[u] = fpr_mul(&a[u], &b[u]);
        a[u + hn] = fpr_mul(&a[u + hn], &b[u]);
    }
}

pub fn poly_div_autoadj_fft(a: &mut [[fpr; 2]], b: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let ib = fpr_inv(&b[u]);
        a[u] = fpr_mul(&a[u], &ib);
        a[u + hn] = fpr_mul(&a[u + hn], &ib);
    }
}

#[allow(non_snake_case)]
pub fn poly_LDL_fft(g00: &[[fpr; 2]], g01: &mut [[fpr; 2]], g11: &mut [[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let g00_re = g00[u];
        let g00_im = g00[u + hn];
        let mut g01_re = g01[u];
        let mut g01_im = g01[u + hn];
        let g11_re = g11[u];
        let g11_im = g11[u + hn];
        let (mu_re, mu_im) = fpc_div(&g01_re, &g01_im, &g00_re, &g00_im);
        (g01_re, g01_im) = fpc_mul(&mu_re, &mu_im, &g01_re, &fpr_neg(&g01_im));
        (g11[u], g11[u + hn]) = fpc_sub(&g11_re, &g11_im, &g01_re, &g01_im);
        g01[u] = mu_re;
        g01[u + hn] = fpr_neg(&mu_im);
    }
}

#[allow(non_snake_case)]
pub fn poly_LDLmv_fft(d11: &mut [[fpr; 2]], l10: &mut [[fpr; 2]], g00: &[[fpr; 2]], g01: &[[fpr; 2]], g11: &[[fpr; 2]], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let g00_re = g00[u];
        let g00_im = g00[u + hn];
        let mut g01_re = g01[u];
        let mut g01_im = g01[u + hn];
        let g11_re = g11[u];
        let g11_im = g11[u + hn];
        let (mu_re, mu_im) = fpc_div(&g01_re, &g01_im, &g00_re, &g00_im);
        (g01_re, g01_im) = fpc_mul(&mu_re, &mu_im, &g01_re, &fpr_neg(&g01_im));
        (d11[u], d11[u + hn]) = fpc_sub(&g11_re, &g11_im, &g01_re, &g01_im);
        l10[u] = mu_re;
        l10[u + hn] = fpr_neg(&mu_im);
    }
}

pub fn poly_split_fft(f0: &mut [[fpr; 2]], f1: &mut [[fpr; 2]], f: &[[fpr; 2]], logn: u32) {
    let (n, hn, qn): (usize, usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    qn = hn >> 1;

    f0[0] = f[0];
    f1[0] = f[hn];
    for u in 0..qn {
        let (a_re, a_im, b_re, b_im): ([fpr; 2], [fpr; 2], [fpr; 2], [fpr; 2]);

        a_re = f[(u << 1) + 0];
        a_im = f[(u << 1) + 0 + hn];
        b_re = f[(u << 1) + 1];
        b_im = f[(u << 1) + 1 + hn];

        let (mut t_re, mut t_im) = fpc_add(&a_re, &a_im, &b_re, &b_im);
        f0[u] = fpr_half(&t_re);
        f0[u + qn] = fpr_half(&t_im);

        (t_re, t_im) = fpc_sub(&a_re, &a_im, &b_re, &b_im);
        (t_re, t_im) = fpc_mul(&t_re, &t_im,
                               &[FPR_GM_TAB[((u + hn) << 1) + 0], 0],
                               &fpr_neg(&[FPR_GM_TAB[((u + hn) << 1) + 1], 0]));
        f1[u] = fpr_half(&t_re);
        f1[u + qn] = fpr_half(&t_im);
    }
}

pub fn poly_merge_fft(f: &mut [[fpr; 2]], f0: &[[fpr; 2]], f1: &[[fpr; 2]], logn: u32) {
    let (n, hn, qn): (usize, usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    qn = hn >> 1;

    f[0] = f0[0];
    f[hn] = f1[0];
    for u in 0..qn {
        let (a_re, a_im): ([fpr; 2], [fpr; 2]);

        a_re = f0[u];
        a_im = f0[u + qn];
        let (b_re, b_im) = fpc_mul(&f1[u], &f1[u + qn],
                                   &[FPR_GM_TAB[((u + hn) << 1) + 0], 0],
                                   &[FPR_GM_TAB[((u + hn) << 1) + 1], 0]);
        let (mut t_re, mut t_im) = fpc_add(&a_re, &a_im, &b_re, &b_im);
        f[(u << 1) + 0] = t_re;
        f[(u << 1) + 0 + hn] = t_im;
        (t_re, t_im) = fpc_sub(&a_re, &a_im, &b_re, &b_im);
        f[(u << 1) + 1] = t_re;
        f[(u << 1) + 1 + hn] = t_im;
    }
}



