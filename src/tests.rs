use super::*;

#[test]
fn test_values() {
    approx_eq(lookup(0.0), 0.5);
    approx_eq(lookup(1.0), 0.8413);
    approx_eq(lookup(-1.0), 0.1587);
    approx_eq(lookup(4.0), 1.0);
    approx_eq(lookup(-4.0), 0.0);
    approx_eq(lookup(5.0), 1.0);
    approx_eq(lookup(-5.0), 0.0);
}

#[test]
fn test_lookup() {
    for i in 0..400 {
        approx_eq(lookup(i as f32 / 40.0), 1.0 - lookup(-i as f32 / 40.0));
    }
}

#[test]
fn test_lookup_with() {
    approx_eq(lookup_with(0.0, 0.0, 1.0), 0.5);
    approx_eq(lookup_with(0.0, 0.0, 100.0), 0.5);
    approx_eq(lookup_with(100.0, 100.0, 1.0), 0.5);
    approx_eq(lookup_with(100.0, 100.0, 100.0), 0.5);
}

fn approx_eq(a: f32, b: f32) {
    assert!((a - b).abs() < 0.0001);
}
