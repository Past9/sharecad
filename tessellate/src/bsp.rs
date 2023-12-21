pub enum TreeSplit {
    Ew,
    Ns,
    None,
}

#[derive(Debug, PartialEq)]
enum Tree {
    Ew { e: Box<Tree>, w: Box<Tree> },
    Ns { n: Box<Tree>, s: Box<Tree> },
    Space,
}
impl Tree {
    pub fn visit_spaces<F: Fn(f64, f64, f64, f64) -> TreeSplit>(
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
                w.visit_spaces(p_n, p_s, p_w, split, visitor);
                e.visit_spaces(p_n, p_s, split, p_e, visitor);
            }
            Tree::Ns { n, s } => {
                let split = (p_n + p_s) / 2.0;
                n.visit_spaces(p_n, split, p_w, p_e, visitor);
                s.visit_spaces(split, p_s, p_w, p_e, visitor);
            }
            Tree::Space => {
                let mut new_self = match visitor(p_n, p_s, p_w, p_e) {
                    TreeSplit::Ew => Tree::Ew {
                        e: Box::new(Tree::Space),
                        w: Box::new(Tree::Space),
                    },
                    TreeSplit::Ns => Tree::Ns {
                        n: Box::new(Tree::Space),
                        s: Box::new(Tree::Space),
                    },
                    TreeSplit::None => Tree::Space,
                };

                match &mut new_self {
                    Tree::Space => {}
                    new_self => {
                        new_self.visit_spaces(p_n, p_s, p_w, p_e, visitor);
                    }
                };

                *self = new_self;
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

    pub fn visit_spaces<F: Fn(f64, f64, f64, f64) -> TreeSplit>(&mut self, visitor: &F) {
        self.tree
            .visit_spaces(self.n, self.s, self.w, self.e, visitor);
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    use super::{BspTree, TreeSplit};

    #[test]
    fn test_split_visitor() {
        let mut bsp = BspTree::new(1.0, 0.0, 0.0, 1.0);

        bsp.visit_spaces(&|n, s, w, e| {
            if (e - w).abs() > 0.25 {
                TreeSplit::Ew
            } else if (n - s).abs() > 0.5 {
                TreeSplit::Ns
            } else {
                TreeSplit::None
            }
        });

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
