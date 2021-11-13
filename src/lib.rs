#![feature(const_fn_floating_point_arithmetic)]

#[cfg(test)]
mod tests;

mod table;
use table::{TABLE, MAX_Z};

// Maps some z value to the corresponding table index.
const fn z_to_index(z: f32) -> usize {
    (z / MAX_Z * TABLE.len() as f32) as usize
}

// Maps some table index to the corresponding z value.
const fn index_to_z(i: usize) -> f32 {
    i as f32 * MAX_Z / TABLE.len() as f32
}

/// Lookup function for the Z table.
/// Given a z value, returns the corresponding value of
/// the cumulative distribution function of the standard
/// normal distribution.
/// The input values may be negative.
pub const fn lookup(z: f32) -> f32 {
    if z >= 0.0 {
        lookup_index(z_to_index(z))
    } else {
        1.0 - lookup_index(z_to_index(-z))
    }
}

/// Lookup function for the Z table.
/// Given a z value, returns the corresponding value of
/// the cumulative distribution function of the normal distribution
/// with the provided mean and standard derivation.
/// The input values may be negative.
pub const fn lookup_with(z: f32, mean: f32, standard_derivation: f32) -> f32 {
    lookup((z - mean) / standard_derivation)
}

/// Reverse lookup function for the Z table.
/// Given a value of the cumulative distribution function
/// of the standard normal distribution, returns
/// the corresponding z value.
/// Only inputs between and including 0 and 1 are allowed.
pub const fn reverse_lookup(p: f32) -> f32 {
    assert!(0.0 <= p && p <= 1.0);
    if p >= 0.5 {
        index_to_z(reverse_lookup_index(p))
    } else {
        -index_to_z(reverse_lookup_index(1.0 - p))
    }
}

/// Reverse lookup function for the Z table.
/// Given a value of the cumulative distribution function
/// of the normal distribution with the provided mean and
/// standard derivation, returns the corresponding z value.
/// Only inputs between and including 0 and 1 are allowed.
pub const fn reverse_lookup_with(p: f32, mean: f32, standard_derivation: f32) -> f32 {
    reverse_lookup(p) * standard_derivation + mean
}

// Provides a compile time reverse lookup for the lookup table.
const fn reverse_lookup_index(p: f32) -> usize {
    assert!(0.5 <= p && p <= 1.0);
    let mut prev_abs = std::f32::MAX;
    let mut i = 0;
    loop {
        let curr_abs = abs(p - lookup_index(i));
        if prev_abs < curr_abs {
            if prev_abs < curr_abs {
                return i - 1;
            } else {
                return i;
            }
        }
        prev_abs = curr_abs;
        i += 1;
        if i == TABLE.len() {
            return i;
        }
    }
}

// Computes the absolute value.
const fn abs(x: f32) -> f32 {
    if x >= 0.0 {
        x
    } else {
        -x
    }
}

// Lookup and index in internal lookup table
// without panicking at invalid indices.
const fn lookup_index(i: usize) -> f32 {
    if i >= TABLE.len() {
        1.0
    } else {
        TABLE[i]
    }
}
