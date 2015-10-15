use options::Options;
use std::any::Any;
use std::rc::Rc;

use Node;

pub struct Tree {
    node: Rc<Node>,
    path: String,
}

impl Tree {
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let mut prefix = &*self.path;
        loop {
            if prefix.is_empty() {
                return self.node.lookup(path);
            }
            if let Some(value) = self.node.lookup(&format!("{}.{}", prefix, path)) {
                return Some(value);
            }
            prefix = match prefix.rfind('.') {
                Some(i) => &prefix[..i],
                _ => "",
            };
        }
    }

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

    pub fn collection(&self, path: &str) -> Option<Vec<Tree>> {
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
    fn collection() {
        let tree = toml::parse(r#"
            [[foo.bar]]
            baz = 42

            [[foo.bar]]
            baz = 69
        "#).unwrap();

        let baz = tree.collection("foo.bar").unwrap().iter().map(|tree| {
            *tree.get::<i64>("baz").unwrap()
        }).collect::<Vec<_>>();

        assert_eq!(&baz, &[42, 69]);
    }
}
