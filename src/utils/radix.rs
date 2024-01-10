#[allow(unused_imports)]
use radix_tree::{Node, Radix};

pub fn split_path(node: Node<char, bool>, path: Vec<char>) -> Vec<(usize, usize)> {
    let f: Box<dyn Fn(&Node<char, bool>) -> bool> = Box::new(|node| {
        let mut paths = node.nodes.len();

        if !node.data.is_none() && node.data.unwrap() {
            paths += 1; // interior termination
        }

        // println!("{:?} {:?}", node.path, node.data);

        return paths > 1;
    });

    // path must exist in radix

    return recursive_split(vec![node], path, 0, 0, vec![], &*f);
}
fn recursive_split(
    nodes: Vec<Node<char, bool>>,
    path: Vec<char>,
    mut i: usize,
    mut j: usize,
    mut offsets: Vec<(usize, usize)>,
    f: &dyn Fn(&Node<char, bool>) -> bool,
) -> Vec<(usize, usize)> {
    if i + j == path.len() {
        offsets.push((i, i + j));

        return offsets;
    }

    for node in nodes {
        let size = node.path.len();

        if path[i + j..i + j + size] == node.path {
            j += node.path.len();

            if f(&node) {
                offsets.push((i, i + j));

                i += j; // slide window
                j = 0; // reset window
            }

            return recursive_split(node.nodes, path, i, j, offsets, f);
        }

        // println!("{:?} {:?} {:?} {:?} {:?} {:?}", node.path, node.data.unwrap(), node.nodes.len(), i, j, offsets);
    }

    panic!("{:?}", path)
}

#[test]
fn test_split() {
    let mut tree = Node::<char, bool>::new(vec![], Some(false));

    tree.insert(vec!['f', 'o', 'o', 'b', 'a', 'r'], true);
    tree.insert(vec!['f', 'o', 'o', 'b', 'a', 'z'], true);
    tree.insert(vec!['f', 'o', 'o'], true);

    let f: Box<dyn Fn(&Node<char, bool>) -> bool> = Box::new(|node| {
        return node.nodes.len() > 1;
    });

    println!("{:?}", tree.find(vec!['b', 'a', 'r']));

    let r = recursive_split(vec![tree], "foobar".chars().collect(), 0, 0, vec![], &*f);

    println!("{:?}", r);
}
