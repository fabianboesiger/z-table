use super::*;

#[test]
fn test_lookup_values() {
    approx_eq(lookup(0.0), 0.5);
    approx_eq(lookup(1.0), 0.8413);
    approx_eq(lookup(-1.0), 0.1587);
    approx_eq(lookup(4.0), 1.0);
    approx_eq(lookup(-4.0), 0.0);
    approx_eq(lookup(5.0), 1.0);
    approx_eq(lookup(-5.0), 0.0);
}

#[test]
fn test_reverse_lookup_values() {
    approx_eq(reverse_lookup(0.5), 0.0);
    approx_eq(reverse_lookup(0.8413), 1.0);
    approx_eq(reverse_lookup(0.1587), -1.0);
    approx_eq(reverse_lookup(1.0), 4.0);
    approx_eq(reverse_lookup(0.0), -4.0);
}

#[test]
fn test_lookup_with() {
    approx_eq(lookup_with(0.0, 0.0, 1.0), 0.5);
    approx_eq(lookup_with(0.0, 0.0, 100.0), 0.5);
    approx_eq(lookup_with(100.0, 100.0, 1.0), 0.5);
    approx_eq(lookup_with(100.0, 100.0, 100.0), 0.5);
}

#[test]
fn test_reverse_lookup_with() {
    approx_eq(reverse_lookup_with(0.5, 0.0, 1.0), 0.0);
    approx_eq(reverse_lookup_with(0.5, 0.0, 100.0), 0.0);
    approx_eq(reverse_lookup_with(0.5, 100.0, 1.0), 100.0);
    approx_eq(reverse_lookup_with(0.5, 100.0, 100.0), 100.0);
}

fn approx_eq(a: f32, b: f32) {
    assert!((a - b).abs() < 0.0001);
}
