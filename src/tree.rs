use std::slice::Iter;

pub struct Node<T> {
    idx: usize,
    left_idx: Option<usize>,
    right_idx: Option<usize>,
    parent_idx: Option<usize>,
    pub val: T
}

pub struct Tree<T> {
    tree: Vec<Node<T>>
}

impl<T: Clone + Ord> Tree<T> {
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

    pub fn val_at(& self, idx: Option<usize>) -> Option<T> {
        match self.at(idx) {
            None => {
                return None;
            }
            Some(i) => {
                return Some(i.val.clone());
            }
        }
    }

    pub fn at(& self, idx: Option<usize>) -> Option<&Node<T>> {
        match idx {
            None => {
                return None;
            },
            Some(i) => {
                if i >= self.tree.len() {
                    return None;
                }

                Some(&self.tree[i])
            }
        }
    }

    pub fn add(&mut self, val: T, curr_idx: usize) -> &mut T {
        if self.tree.len() == 0 {
            self.tree.push(
                Node {
                    idx: 0,
                    left_idx: None,
                    right_idx: None,
                    parent_idx: None,
                    val,
                }
            );
            return &mut self.tree[0].val
        }

        if self.tree[curr_idx].val == val {
            return &mut self.tree[curr_idx].val;
        } else if self.tree[curr_idx].val < val {
            match self.tree[curr_idx].left_idx {
                None => {
                    let next_insert_idx = self.tree.len();
                    self.tree.push(
                        Node {
                            idx: next_insert_idx,
                            left_idx: None,
                            right_idx: None,
                            parent_idx: Some(curr_idx),
                            val
                        }
                    );
                    self.tree[curr_idx].left_idx = Some(next_insert_idx);
                    return &mut self.tree[next_insert_idx].val;
                }
                Some(i) => {
                    return self.add(val, i);
                }
            }
        } else {
            match self.tree[curr_idx].right_idx {
                None => {
                    let next_insert_idx = self.tree.len();
                    self.tree.push(
                        Node {
                            idx: next_insert_idx,
                            left_idx: None,
                            right_idx: None,
                            parent_idx: Some(curr_idx),
                            val
                        }
                    );
                    self.tree[curr_idx].right_idx = Some(next_insert_idx);
                    return &mut self.tree[next_insert_idx].val;
                }
                Some(i) => {
                    return self.add(val, i);
                }
            }
        }
    }

    pub fn iter(&mut self) -> Iter<Node<T>> {
        self.tree.iter()
    }

    pub fn new() -> Tree<T> {
        Tree::<T> {
            tree: Vec::new()
        }
    }
}

#[test]
fn populate_tree_from_sorted() {
    let mut t: Tree<usize> = Tree::new();
    let arr: &[usize] = &[1, 2, 3, 4, 5, 6];
    t.populate_from_sorted(None, arr);
    let got_vec = t.iter().map(|n| n.val).collect::<Vec<usize>>();
    assert_eq!(vec![4, 2, 1, 3, 6, 5], got_vec);
}

#[test]
fn tree_contains_correct_left_and_right_idxs() {
    let mut t: Tree<usize> = Tree::new();
    let arr: &[usize] = &[1, 2, 3, 4, 5, 6];
    t.populate_from_sorted(None, arr);

    // Our tree should look like:
    //              4
    //      2           6
    //  1       3   5

    let first_el = &t.tree[0];
    // first_el has no parent as it's the root
    assert_eq!(first_el.parent_idx, None);
    assert_eq!(t.val_at(first_el.left_idx).unwrap(), 2);
    assert_eq!(t.val_at(first_el.right_idx).unwrap(), 6);

    let second_el = &t.tree[1];
    assert_eq!(second_el.parent_idx.unwrap(), 0);
    assert_eq!(t.val_at(second_el.left_idx).unwrap(), 1);
    assert_eq!(t.val_at(second_el.right_idx).unwrap(), 3);

    let third_el = &t.tree[4];
    assert_eq!(third_el.parent_idx.unwrap(), 0);
    assert_eq!(t.val_at(third_el.left_idx).unwrap(), 5);
    assert_eq!(t.val_at(third_el.right_idx), None);
}
