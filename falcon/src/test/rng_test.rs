#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::falcon_c::rng_c::{Buf as BufC, State as StateC, Prng as PrngC, prng_refill as prng_refill_c, prng_init as prng_init_c};
    use crate::rng::{Buf, Prng, prng_init, prng_refill, State};
    use crate::shake::{i_shake256_init, InnerShake256Context, St};
    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, InnerShake256Context as InnerShake256ContextC, St as StC};

    #[test]
    fn test_prng_refill() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();

            prng_refill(&mut prng);

            unsafe {
                assert!(!test_prng_equality(&prng, &prng_c));

                prng_refill_c(&prng_c as *const PrngC);
                assert!(test_prng_equality(&prng, &prng_c))
            }
        }
    }

    #[test]
    fn test_prng_init() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();

            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let st = St { a: random_state };

            let mut sc_rust = InnerShake256Context { st, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            prng_init(&mut prng, &mut sc_rust);

            unsafe {
                falcon_inner_i_shake256_init(&sc_c);
                assert!(!test_prng_equality(&prng, &prng_c));

                prng_init_c(&prng_c, &sc_c);
                assert!(test_prng_equality(&prng, &prng_c));
            }
        }
    }

    fn create_random_prngs() -> (Prng, PrngC) {
        let mut rng = rand::thread_rng();
        let buf_d: [u8; 512] = core::array::from_fn(|_| rng.gen::<u8>());
        let state_d: [u8; 256] = core::array::from_fn(|_| rng.gen::<u8>());

        let ptr:usize = rand::random();
        let typ:i32 = rand::random();

        let buf = Buf {d: buf_d};
        let state = State {d: state_d};

        let buf_c: BufC = BufC {d: buf_d.clone()};
        let state_c: StateC = StateC {d: state_d.clone()};

        let prng = Prng {buf, ptr, state, typ};
        let prng_c = PrngC {buf: buf_c, ptr: ptr as u64, state: state_c, typ};

        return (prng, prng_c);
    }

    unsafe fn test_prng_equality(prng: &Prng, prng_c: &PrngC) -> bool {
        if prng.buf.d != prng_c.buf.d {
            return false;
        }

        if prng.ptr != prng_c.ptr as usize {
            return false;
        }

        if prng.state.d != prng_c.state.d {
            return false;
        }

        if prng.typ != prng_c.typ {
            return false;
        }

        return true;
    }
}