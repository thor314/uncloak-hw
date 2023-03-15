use ark_ec::pairing::Pairing; // Import Pairing trait from ark_ec library
use ark_bls12_381::{Bls12_381, G1Projective as G1, G2Projective as G2, G1Affine, G2Affine}; // Import relevant types from ark_bls12_381 library
use ark_std::UniformRand; // Import UniformRand trait from ark_std library

fn main() {
     // Create a random number generator using the current thread
    let mut rng = ark_std::rand::thread_rng();

    // Let's sample uniformly random field elements:
    let a: G1Affine = G1::rand(&mut rng).into(); // Sample a random point on G1 curve
    let b: G2Affine = G2::rand(&mut rng).into(); // Sample a random point on G2 curve

    // We can compute the pairing of `a` and `b`:
    let c = Bls12_381::pairing(a, b); // Compute the pairing of points a and b
    println!("c: {:?}", c); // Print the result of the pairing operation

    // Compute the pairing partwise. Compute the Miller loop of points a and b
    let c_ml = Bls12_381::miller_loop(&a, &b); 

    // // Compute the final exponentiation of the result of the Miller loop
    let c_fe = Bls12_381::final_exponentiation(c_ml).unwrap();
    println!("c_fe: {:?}", c_fe); // Print the result of the final exponentiation

    // Check if the result of the pairing and the final exponentiation match
    assert_eq!(c, c_fe); 
}

#[cfg(test)]
mod tests {
    use super::*; // Import all items from the parent module

    #[test]
    fn test_pairing() { // Define a test function for the pairing operation
        let mut rng = ark_std::rand::thread_rng(); // Create a random number generator using the current thread
        let a: G1Affine = G1::rand(&mut rng).into(); // Sample a random point on G1 curve
        let b: G2Affine = G2::rand(&mut rng).into(); // Sample a random point on G2 curve
        let c = Bls12_381::pairing(a, b); // Compute the pairing of points a and b

        let c_ml = Bls12_381::miller_loop(&a, &b); // Compute the Miller loop of points a and b
        let c_fe = Bls12_381::final_exponentiation(c_ml).unwrap(); // Compute the final exponentiation of the result of the Miller loop

        assert_eq!(c, c_fe); // Check if the result of the pairing and the final exponentiation match
    }
}

