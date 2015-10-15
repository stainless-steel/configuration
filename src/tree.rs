use options::Options;
use std::any::Any;
use std::rc::Rc;

use Node;

/// A tree.
pub struct Tree {
    node: Rc<Node>,
    path: String,
}

impl Tree {
    /// Look up a value.
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let mut prefix = &*self.path;
        loop {
            if prefix.is_empty() {
                return self.node.get(path);
            }
            if let Some(value) = self.node.get(&format!("{}.{}", prefix, path)) {
                return Some(value);
            }
            prefix = match prefix.rfind('.') {
                Some(i) => &prefix[..i],
                _ => "",
            };
        }
    }

    /// Return a subtree.
    pub fn branch(&self, path: &str) -> Option<Tree> {
        self.get::<Node>(path).map(|_| {
            Tree {
                node: self.node.clone(),
                path: if self.path.is_empty() {
                    path.to_string()
                } else {
                    format!("{}.{}", &self.path, path)
                },
            }
        })
    }

    /// Return an array of subtrees.
    pub fn forest(&self, path: &str) -> Option<Vec<Tree>> {
        self.get::<Vec<Node>>(path).map(|array| {
            array.iter().enumerate().map(|(i, _)| {
                Tree {
                    node: self.node.clone(),
                    path: if self.path.is_empty() {
                        format!("{}.{}", path, i)
                    } else {
                        format!("{}.{}.{}", &self.path, path, i)
                    },
                }
            }).collect()
        })
    }
}

impl From<Options> for Tree {
    fn from(options: Options) -> Tree {
        Tree {
            node: Rc::new(Node::from(options)),
            path: String::new(),
        }
    }
}

impl From<Node> for Tree {
    fn from(node: Node) -> Tree {
        Tree {
            node: Rc::new(node),
            path: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use format::toml;

    #[test]
    fn branch() {
        let tree = toml::parse(r#"
            qux = 69

            [foo]
            bar = 42

            [[bar.baz]]
            qux = 42
        "#).unwrap();

        {
            let tree = tree.branch("foo").unwrap();
            assert_eq!(tree.get::<i64>("bar").unwrap(), &42);
            assert_eq!(tree.get::<i64>("qux").unwrap(), &69);
        }
        {
            let tree = tree.branch("bar").unwrap();
            assert_eq!(tree.get::<i64>("qux").unwrap(), &69);
        }
        {
            let tree = tree.branch("bar.baz.0").unwrap();
            assert_eq!(tree.get::<i64>("qux").unwrap(), &42);
        }
    }

    #[test]
    fn forest() {
        let tree = toml::parse(r#"
            [[foo.bar]]
            baz = 42

            [[foo.bar]]
            baz = 69
        "#).unwrap();

        let baz = tree.forest("foo.bar").unwrap().iter().map(|tree| {
            *tree.get::<i64>("baz").unwrap()
        }).collect::<Vec<_>>();

        assert_eq!(&baz, &[42, 69]);
    }
}
