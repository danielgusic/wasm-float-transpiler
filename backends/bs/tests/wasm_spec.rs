use wasm_soft_float_bs::*;

wasm_soft_float_utils::impl_tests!();

// Rust's float-to-int `as` casts saturate exactly like the wasm `trunc_sat` instructions.
macro_rules! test_trunc_sat {
    ($name: ident, $f: ident, $float: ty, $int: ty) => {
        mod $name {
            use super::*;

            fn matches_as_cast(v: $float) -> bool {
                $f(v.to_bits()) == v as $int
            }

            quickcheck::quickcheck! {
                fn random_bits(bits: u64) -> bool {
                    matches_as_cast(<$float>::from_bits(bits as _))
                }
            }

            #[test]
            fn edge_cases() {
                let limits = [
                    2147483648.0, 4294967296.0, 9223372036854775808.0, 18446744073709551616.0,
                ];
                let mut values = vec![
                    0.0, -0.0, 0.5, -0.5, 1.0, -1.0, -0.999, 1.5,
                    <$float>::NAN, -<$float>::NAN, <$float>::INFINITY, <$float>::NEG_INFINITY,
                    <$float>::MAX, <$float>::MIN, <$float>::MIN_POSITIVE, <$float>::EPSILON,
                ];
                for l in limits {
                    let l = l as $float;
                    for v in [l, -l, l.next_down(), l.next_up(), -l.next_down(), -l.next_up()] {
                        values.push(v);
                    }
                }
                for v in values {
                    assert!(matches_as_cast(v), "{} ({:#x})", v, v.to_bits());
                }
            }
        }
    };
}
test_trunc_sat!(i32_trunc_sat_f32, __wasm_soft_float_i_32_trunc_ssat_f_32, f32, i32);
test_trunc_sat!(u32_trunc_sat_f32, __wasm_soft_float_i_32_trunc_usat_f_32, f32, u32);
test_trunc_sat!(i32_trunc_sat_f64, __wasm_soft_float_i_32_trunc_ssat_f_64, f64, i32);
test_trunc_sat!(u32_trunc_sat_f64, __wasm_soft_float_i_32_trunc_usat_f_64, f64, u32);
test_trunc_sat!(i64_trunc_sat_f32, __wasm_soft_float_i_64_trunc_ssat_f_32, f32, i64);
test_trunc_sat!(u64_trunc_sat_f32, __wasm_soft_float_i_64_trunc_usat_f_32, f32, u64);
test_trunc_sat!(i64_trunc_sat_f64, __wasm_soft_float_i_64_trunc_ssat_f_64, f64, i64);
test_trunc_sat!(u64_trunc_sat_f64, __wasm_soft_float_i_64_trunc_usat_f_64, f64, u64);
