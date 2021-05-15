use fast_float::parse;

#[test]
fn test_f64_soft_float() {
    // PowerPC: Seems to be failing due to a conversion to/from single-precision.
    assert_eq!(
        parse::<f64, _>("1303e-20").unwrap(),
        f64::from_bits(0b11110001101110000010111000110111101101101111110001101110101000)
    );
    assert_eq!(
        parse::<f64, _>("2606e-20").unwrap(),
        f64::from_bits(0b11110001111110000010111000110111101101101111110001101110101000)
    );
    assert_eq!(
        parse::<f64, _>("5212e-20").unwrap(),
        f64::from_bits(0b11110010001110000010111000110111101101101111110001101110101000)
    );
}
