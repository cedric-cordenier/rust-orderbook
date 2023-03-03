mod tree {
    pub struct Node<T> {
        idx: usize,
        left_idx: Option<usize>,
        right_idx: Option<usize>,
        parent_idx: Option<usize>,
        val: T
    }

    pub struct Tree<T> {
        tree: Vec<Node<T>>
    }

    impl<T: Clone> Tree<T> {
        pub fn populate_from_sorted(&mut self, parent_idx: Option<usize>, sorted: &[T]) -> Option<usize> {
            if sorted.len() == 0 {
                return None;
            }

            let mid_idx = sorted.len() / 2;

            let curr_idx = self.tree.len();
            let node = Node {
                idx: curr_idx,
                left_idx: None,
                right_idx: None,
                parent_idx,
                val: sorted[mid_idx].clone()
            };
            self.tree.push(node);

            let left_idx = self.populate_from_sorted(Some(curr_idx), &sorted[..mid_idx]);
            self.tree[curr_idx].left_idx = left_idx;

            let right_idx = self.populate_from_sorted(Some(curr_idx), &sorted[mid_idx+1..]);
            self.tree[curr_idx].right_idx = right_idx;

            Some(curr_idx)
        }

        pub fn new() -> Tree<T> {
            Tree::<T> {
                tree: Vec::new()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::tree::*;

    #[test]
    fn populate_tree_from_sorted() {
        let mut t: Tree<usize> = Tree::new();
        let arr: &[usize] = &[1, 2, 3, 4, 5, 6];
        t.populate_from_sorted(None, arr);
        assert_eq!(true, false);
    }
}
