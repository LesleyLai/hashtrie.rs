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
    Branch([Option<Rc<Node<T>>>; BRANCH_SIZE]),
}

fn get_at<T>(node: &Node<T>, hash: Hash) -> Option<&T> {
    use Node::*;

    match node {
        Leaf(v) => {
            assert!(hash == 0);
            Some(v)
        }
        Branch(children) => match children[branch_index(hash)] {
            None => None,
            Some(ref node) => get_at(&node, shift_hash(hash)),
        },
    }
}

struct HashTrie<T> {
    root: Node<T>,
}

impl<T> HashTrie<T> {
    fn new() -> HashTrie<T> {
        HashTrie {
            root: Node::Branch([
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None,
            ]),
        }
    }

    // fn insert(&self, element: T) -> HashTrie<T> {
    //     HashTrie {
    //         root: Node::Branch(HashMap::new()),
    //     }
    // }
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
        let leaf = Rc::new(Node::Leaf(1));

        let level1_map = [
            None,
            Some(leaf),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let level1 = Rc::new(Node::Branch(level1_map));

        let hash: usize = 0b00001_00010;
        let shifted: usize = shift_hash(hash);

        assert_eq!(get_at(&level1, shifted), Some(&1));

        let trie_map = [
            None,
            None,
            Some(level1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let trie: Node<i32> = Node::Branch(trie_map);

        assert_eq!(get_at(&trie, hash), Some(&1));
        assert_eq!(get_at(&trie, 0), None);
    }

    #[test]
    fn insert_element() {
        let trie: HashTrie<i32> = HashTrie::new();
        // let trie2 = trie.insert(100);
        // let trie_2: Node<i32> = insert(trie, 1);
    }
}
