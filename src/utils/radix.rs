#[allow(unused_imports)]
use radix_tree::{Node, Radix};

pub fn split_path(node: Node<char, bool>, path: &str) -> Vec<(usize, usize)> {
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
fn recursive_split<'a>(
    nodes: Vec<Node<char, bool>>,
    path: &'a str,
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
        let branch: String = node.path.iter().collect();

        if &path[i + j..i + j + size] == branch.as_str() {
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
    let mut tree = Node::<char, bool>::new("", Some(false));

    tree.insert("foobar", true);
    tree.insert("foobaz", true);
    tree.insert("foo", true);

    let f: Box<dyn Fn(&Node<char, bool>) -> bool> = Box::new(|node| {
        return node.nodes.len() > 1;
    });

    println!("{:?}", tree.find("bar"));

    let r = recursive_split(vec![tree], "foobar", 0, 0, vec![], &*f);

    println!("{:?}", r);
}
