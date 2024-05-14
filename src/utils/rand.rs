use rand::seq::SliceRandom;
use rand::Rng;

pub fn distribute_randomly(
    total: u64,
    iterations: usize,
    min_value: u64,
    max_value: u64,
) -> Vec<u64> {
    println!("Total: {}, Iterations: {}", total, iterations);
    assert!(
        total >= iterations as u64,
        "Total must be greater than or equal to the number of iterations"
    );

    let mut rng = rand::thread_rng();
    let mut amounts = vec![0; iterations];
    let mut remaining = total;

    // Create a list of indices and shuffle it
    let mut indices: Vec<usize> = (0..iterations).collect();
    indices.shuffle(&mut rng);

    for &index in &indices {
        // Determine the maximum amount that can be added to the current iteration
        let min_per_iteration = total / iterations as u64;
        let max_per_iteration = total - (iterations as u64 - 1) * min_per_iteration;
        let max_addition = remaining.min(max_per_iteration);

        // Generate a random amount to add within the allowed range
        let amount_to_add = if index == *indices.last().unwrap() {
            remaining
        } else {
            rng.gen_range(min_value..=max_value).min(max_addition)
        };

        // Update the amounts and remaining total
        amounts[index] += amount_to_add;
        remaining -= amount_to_add;

        // If there's no remaining amount, break the loop
        if remaining == 0 {
            break;
        }
    }

    amounts
}
