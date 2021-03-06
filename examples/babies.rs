use z_table::{lookup_with, reverse_lookup_with};

fn main() {
    // Some birth weights of newborns in kg.
    let birth_weights: [f32; 5] = [2.5, 2.7, 3.1, 3.4, 3.6];
    let n = birth_weights.len() as f32;
    // Calculate the average weight of a newborn.
    let mean: f32 = birth_weights.iter().sum::<f32>() / n;
    // Calculate the variance and standard derivation.
    let variance: f32 = birth_weights.iter().map(|x| x.powi(2)).sum::<f32>() / n - mean.powi(2);
    let standard_derivation = variance.sqrt();

    println!(
        "The probability of a newborn to weight less than 3.5 kg is {} %",
        lookup_with(3.5, mean, standard_derivation) * 100.0
    );

    println!(
        "The weight of a newborn is with a 90 % probability under {} kg",
        reverse_lookup_with(0.9, mean, standard_derivation)
    );
}
