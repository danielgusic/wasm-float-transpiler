use wasm_soft_float_apfloat::*;

wasm_soft_float_utils::impl_tests!();

#[test]
fn float_width_conversions_use_ieee_bit_patterns() {
    for value in [0.0_f32, -0.0, 1.0, -1.0, 86400.0, f32::INFINITY] {
        assert_eq!(
            __wasm_soft_float_f_64_promote_f_32(value.to_bits()),
            f64::from(value).to_bits()
        );
    }

    for value in [0.0_f64, -0.0, 1.0, -1.0, 86400.0, f64::INFINITY] {
        assert_eq!(
            __wasm_soft_float_f_32_demote_f_64(value.to_bits()),
            (value as f32).to_bits()
        );
    }
}

#[test]
fn saturating_truncations_handle_wasm_edge_cases() {
    assert_eq!(
        __wasm_soft_float_i_64_trunc_ssat_f_64(f64::NAN.to_bits()),
        0
    );
    assert_eq!(
        __wasm_soft_float_i_64_trunc_ssat_f_64(f64::INFINITY.to_bits()),
        i64::MAX
    );
    assert_eq!(
        __wasm_soft_float_i_64_trunc_ssat_f_64(f64::NEG_INFINITY.to_bits()),
        i64::MIN
    );
    assert_eq!(
        __wasm_soft_float_i_32_trunc_usat_f_64(f64::NEG_INFINITY.to_bits()),
        0
    );
    assert_eq!(
        __wasm_soft_float_i_32_trunc_usat_f_64(f64::INFINITY.to_bits()),
        u32::MAX
    );
}
