fn hash_and_print(input: [Field; 2]) {
    // Calculate the Pedersen hash of the input array
    let result: Field = std::hash::pedersen_hash(input);

    // Print the result
    println(result);
}

fn main(
    input1: Field,
    input2: Field
) {
    // Example input array of two Field elements
    let input: [Field; 2] = [input1, input2];

    // Call the hash_and_print function with the input
    hash_and_print(input);
}
