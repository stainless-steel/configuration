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
    /// Return a subtree.
    pub fn branch(&self, path: &str) -> Option<Tree> {
        let path = self.chain(path);
        if self.node.get::<Node>(&path).is_none() {
            return None;
        }
        Some(Tree {
            node: self.node.clone(),
            path: path,
        })
    }

    /// Return an array of subtrees.
    pub fn forest(&self, path: &str) -> Option<Vec<Tree>> {
        let path = self.chain(path);
        let array = match self.node.get::<Vec<Node>>(&path) {
            Some(array) => array,
            _ => return None,
        };
        Some(array.iter().enumerate().map(|(i, _)| {
            Tree {
                node: self.node.clone(),
                path: format!("{}.{}", path, i),
            }
        }).collect())
    }

    /// Read a value.
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let (head, tail) = match path.rfind('.') {
            Some(i) => (self.chain(&path[..i]), &path[(i + 1)..]),
            _ => (self.path.clone(), path),
        };
        let mut head = &*head;
        loop {
            if head.is_empty() {
                return self.node.get(tail);
            }
            if let Some(value) = self.node.get(&format!("{}.{}", head, tail)) {
                return Some(value);
            }
            head = match head.rfind('.') {
                Some(i) => &head[..i],
                _ => "",
            };
        }
    }

    /// Write a value.
    pub fn set<T: Any>(&mut self, path: &str, value: T) -> Option<()> {
        let path = self.chain(path);
        match Rc::get_mut(&mut self.node) {
            Some(node) => node.set(&path, value),
            _ => None,
        }
    }

    fn chain(&self, chunk: &str) -> String {
        if self.path.is_empty() {
            return chunk.to_string();
        }
        let mut path = self.path.clone();
        path.push('.');
        path.push_str(chunk);
        path
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

    #[test]
    fn get() {
        let tree = toml::parse(r#"
            qux = 69

            [foo]
            bar = 42
        "#).unwrap();

        assert_eq!(tree.get::<i64>("qux").unwrap(), &69);
        assert_eq!(tree.get::<i64>("foo.bar").unwrap(), &42);
        assert_eq!(tree.get::<i64>("foo.qux").unwrap(), &69);
    }

    #[test]
    fn set() {
        let mut tree = toml::parse(r#"
            qux = 69

            [foo]
            bar = 42
        "#).unwrap();

        tree.set("qux", 42).unwrap();
        assert_eq!(tree.get::<i32>("qux").unwrap(), &42);

        tree.set("foo.bar", 69).unwrap();
        assert_eq!(tree.get::<i32>("foo.bar").unwrap(), &69);

        tree.set("foo.bar.baz", 42).unwrap();
        assert_eq!(tree.get::<i32>("foo.bar.baz").unwrap(), &42);
    }
}
