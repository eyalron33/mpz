
// Struct representing a hash path in a Merkle tree
struct HashPath {
    path: [Field; 3],
    index: Field
}


fn main(
    society: Field,
    alice_x: Field, 
    hash_path: HashPath,
) {
    let merkel_root = 
        std::merkle::compute_merkle_root(alice_x, hash_path.index, hash_path.path);

    assert(society == merkel_root);
}
