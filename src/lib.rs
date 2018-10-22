use std::collections::HashMap;
use std::option::Option;
use std::rc::Rc;

pub type Hash = usize;

const BIT_PER_CHUNK: usize = 5;
const BRANCH_SIZE: usize = 1 << BIT_PER_CHUNK;
const CHUNK_MASK: usize = (1 << BIT_PER_CHUNK) - 1;

fn shift_hash(hash: Hash) -> Hash {
    hash >> BIT_PER_CHUNK
}

fn branch_index(hash: Hash) -> usize {
    hash & CHUNK_MASK
}

enum Node<T> {
    Leaf(T),
    Branch(HashMap<usize, Node<T>>),
}

fn get_at<T>(node: &Node<T>, hash: Hash) -> Option<&T> {
    use Node::*;

    match node {
        Leaf(v) => {
            assert!(hash == 0);
            Some(v)
        }
        Branch(children) => match &children.get(&branch_index(hash)) {
            None => None,
            Some(node) => get_at(&node, shift_hash(hash)),
        },
    }
}

#[cfg(test)]
mod chunked_hash_tests {
    use super::*;
    #[test]
    fn shift_hashes() {
        let hash: usize = 0b00011_00001;
        assert_eq!(hash, 0b00011_00001);

        let shifted: usize = shift_hash(hash);
        assert_eq!(shifted, 0b11);
    }

    #[test]
    fn explicit_node_creation_and_lookup() {
        let leaf = Node::Leaf(1);

        let mut level1_map = HashMap::new();
        level1_map.insert(1, leaf);
        let level1: Node<i32> = Node::Branch(level1_map);

        let hash: usize = 0b00001_00010;
        let shifted: usize = shift_hash(hash);

        assert_eq!(get_at(&level1, shifted), Some(&1));

        let mut trie_map = HashMap::new();
        trie_map.insert(2, level1);
        let trie: Node<i32> = Node::Branch(trie_map);

        assert_eq!(get_at(&trie, hash), Some(&1));
    }
}
