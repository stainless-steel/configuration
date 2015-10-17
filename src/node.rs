use options::Options;
use std::any::Any;

/// A node.
pub struct Node(Options);

impl Node {
    /// Create a node.
    #[inline]
    pub fn new() -> Node {
        Node(Options::new())
    }

    /// Read a value.
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let (head, tail) = match path.find('.') {
            Some(i) => (&path[..i], &path[(i + 1)..]),
            _ => return self.0.get_ref(path),
        };
        if let Some(node) = self.0.get_ref::<Node>(head) {
            return node.get(tail);
        }
        if let Some(array) = self.0.get_ref::<Vec<Node>>(head) {
            let (head, tail) = match tail.find('.') {
                Some(i) => (&tail[..i], &tail[(i + 1)..]),
                _ => (tail, ""),
            };
            if let Ok(i) = head.parse::<usize>() {
                if tail.is_empty() {
                    return Any::downcast_ref(&array[i]);
                } else {
                    return array[i].get(tail);
                }
            }
        }
        None
    }

    /// Write a value.
    #[cfg(feature = "unstable")]
    pub fn set<T: Any>(&mut self, path: &str, value: T) -> Option<()> {
        let (head, tail) = match path.find('.') {
            Some(i) => (&path[..i], &path[(i + 1)..]),
            _ => {
                self.0.set(path, value);
                return Some(());
            },
        };
        if let Some(node) = self.0.get_mut::<Node>(head) {
            return node.set(tail, value);
        }
        let mut node = Node(Options::new());
        let result = node.set(tail, value);
        self.0.set(head, node);
        result
    }
}

impl From<Options> for Node {
    #[inline]
    fn from(options: Options) -> Node {
        Node(options)
    }
}
