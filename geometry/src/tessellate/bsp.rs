#[derive(Debug, PartialEq)]
pub enum TreeSplit {
    Ew,
    Ns,
}

#[derive(Debug, PartialEq)]
enum Tree {
    Ew { e: Box<Tree>, w: Box<Tree> },
    Ns { n: Box<Tree>, s: Box<Tree> },
    Space,
}
impl Tree {
    pub fn split_spaces<F: Fn(f64, f64, f64, f64) -> Option<TreeSplit>>(
        &mut self,
        p_n: f64,
        p_s: f64,
        p_w: f64,
        p_e: f64,
        visitor: &F,
    ) {
        match self {
            Tree::Ew { e, w } => {
                let split = (p_w + p_e) / 2.0;
                w.split_spaces(p_n, p_s, p_w, split, visitor);
                e.split_spaces(p_n, p_s, split, p_e, visitor);
            }
            Tree::Ns { n, s } => {
                let split = (p_n + p_s) / 2.0;
                n.split_spaces(p_n, split, p_w, p_e, visitor);
                s.split_spaces(split, p_s, p_w, p_e, visitor);
            }
            Tree::Space => {
                let mut new_self = match visitor(p_n, p_s, p_w, p_e) {
                    Some(split) => match split {
                        TreeSplit::Ew => Tree::Ew {
                            e: Box::new(Tree::Space),
                            w: Box::new(Tree::Space),
                        },
                        TreeSplit::Ns => Tree::Ns {
                            n: Box::new(Tree::Space),
                            s: Box::new(Tree::Space),
                        },
                    },
                    None => Tree::Space,
                };

                match &mut new_self {
                    Tree::Space => {}
                    new_self => {
                        new_self.split_spaces(p_n, p_s, p_w, p_e, visitor);
                    }
                };

                *self = new_self;
            }
        }
    }

    pub fn visit_splits<F: FnMut(f64, f64, f64, f64, TreeSplit)>(
        &self,
        p_n: f64,
        p_s: f64,
        p_w: f64,
        p_e: f64,
        visitor: &mut F,
    ) {
        match self {
            Tree::Ew { e, w } => {
                visitor(p_n, p_s, p_w, p_e, TreeSplit::Ew);
                let split = (p_w + p_e) / 2.0;
                w.visit_splits(p_n, p_s, p_w, split, visitor);
                e.visit_splits(p_n, p_s, split, p_e, visitor);
            }
            Tree::Ns { n, s } => {
                visitor(p_n, p_s, p_w, p_e, TreeSplit::Ns);
                let split = (p_n + p_s) / 2.0;
                n.visit_splits(p_n, split, p_w, p_e, visitor);
                s.visit_splits(split, p_s, p_w, p_e, visitor);
            }
            Tree::Space => {}
        }
    }

    pub fn visit_spaces<F: FnMut(f64, f64, f64, f64)>(
        &self,
        p_n: f64,
        p_s: f64,
        p_w: f64,
        p_e: f64,
        visitor: &mut F,
    ) {
        match self {
            Tree::Ew { e, w } => {
                let split = (p_w + p_e) / 2.0;
                w.visit_spaces(p_n, p_s, p_w, split, visitor);
                e.visit_spaces(p_n, p_s, split, p_e, visitor);
            }
            Tree::Ns { n, s } => {
                let split = (p_n + p_s) / 2.0;
                n.visit_spaces(p_n, split, p_w, p_e, visitor);
                s.visit_spaces(split, p_s, p_w, p_e, visitor);
            }
            Tree::Space => {
                visitor(p_n, p_s, p_w, p_e);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BspTree {
    n: f64,
    s: f64,
    w: f64,
    e: f64,
    tree: Tree,
}
impl BspTree {
    pub fn new(n: f64, s: f64, w: f64, e: f64) -> Self {
        Self {
            n,
            s,
            w,
            e,
            tree: Tree::Space,
        }
    }

    pub fn split_spaces<F: Fn(f64, f64, f64, f64) -> Option<TreeSplit>>(&mut self, visitor: &F) {
        self.tree
            .split_spaces(self.n, self.s, self.w, self.e, visitor);
    }

    pub fn visit_splits<F: FnMut(f64, f64, f64, f64, TreeSplit)>(&self, visitor: &mut F) {
        self.tree
            .visit_splits(self.n, self.s, self.w, self.e, visitor);
    }

    pub fn visit_spaces<F: FnMut(f64, f64, f64, f64)>(&self, visitor: &mut F) {
        self.tree
            .visit_spaces(self.n, self.s, self.w, self.e, visitor);
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    use super::{BspTree, TreeSplit};

    fn make_tree() -> BspTree {
        let mut bsp = BspTree::new(1.0, 0.0, 0.0, 1.0);

        bsp.split_spaces(&|n, s, w, e| {
            if (e - w).abs() > 0.25 {
                Some(TreeSplit::Ew)
            } else if (n - s).abs() > 0.5 {
                Some(TreeSplit::Ns)
            } else {
                None
            }
        });

        bsp
    }

    #[test]
    fn test_space_visitor() {
        let bsp = make_tree();

        let mut spaces: Vec<(f64, f64, f64, f64)> = vec![];
        bsp.visit_spaces(&mut |n, s, w, e| {
            spaces.push((n, s, w, e));
        });

        assert_eq!(
            vec![
                (1.0, 0.5, 0.0, 0.25),
                (0.5, 0.0, 0.0, 0.25),
                (1.0, 0.5, 0.25, 0.5),
                (0.5, 0.0, 0.25, 0.5),
                (1.0, 0.5, 0.5, 0.75),
                (0.5, 0.0, 0.5, 0.75),
                (1.0, 0.5, 0.75, 1.0),
                (0.5, 0.0, 0.75, 1.0),
            ],
            spaces
        );
    }

    #[test]
    fn test_split_visitor() {
        let bsp = make_tree();

        let mut splits: Vec<(f64, f64, f64, f64, TreeSplit)> = vec![];
        bsp.visit_splits(&mut |n, s, w, e, split| {
            splits.push((n, s, w, e, split));
        });

        assert_eq!(
            vec![
                (1.0, 0.0, 0.0, 1.0, TreeSplit::Ew),
                (1.0, 0.0, 0.0, 0.5, TreeSplit::Ew),
                (1.0, 0.0, 0.0, 0.25, TreeSplit::Ns),
                (1.0, 0.0, 0.25, 0.5, TreeSplit::Ns),
                (1.0, 0.0, 0.5, 1.0, TreeSplit::Ew),
                (1.0, 0.0, 0.5, 0.75, TreeSplit::Ns),
                (1.0, 0.0, 0.75, 1.0, TreeSplit::Ns),
            ],
            splits
        );
    }

    #[test]
    fn test_tree() {
        let bsp = make_tree();

        assert_eq!(
            bsp,
            BspTree {
                n: 1.0,
                s: 0.0,
                w: 0.0,
                e: 1.0,
                tree: Tree::Ew {
                    e: Box::new(Tree::Ew {
                        e: Box::new(Tree::Ns {
                            n: Box::new(Tree::Space),
                            s: Box::new(Tree::Space),
                        }),
                        w: Box::new(Tree::Ns {
                            n: Box::new(Tree::Space),
                            s: Box::new(Tree::Space),
                        }),
                    }),
                    w: Box::new(Tree::Ew {
                        e: Box::new(Tree::Ns {
                            n: Box::new(Tree::Space),
                            s: Box::new(Tree::Space),
                        }),
                        w: Box::new(Tree::Ns {
                            n: Box::new(Tree::Space),
                            s: Box::new(Tree::Space),
                        }),
                    }),
                }
            }
        );
    }
}
