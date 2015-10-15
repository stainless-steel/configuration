use options::Options;
use std::any::Any;
use std::ops::{Deref, DerefMut};

/// A node.
pub struct Node(Options);

impl Node {
    /// Look up a value.
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let (head, tail) = match path.find('.') {
            Some(i) => (&path[..i], &path[(i + 1)..]),
            _ => return self.get_ref(path),
        };
        if let Some(node) = self.get_ref::<Node>(head) {
            return node.get(tail);
        }
        if let Some(array) = self.get_ref::<Vec<Node>>(head) {
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
}

impl From<Options> for Node {
    #[inline]
    fn from(options: Options) -> Node {
        Node(options)
    }
}

impl Deref for Node {
    type Target = Options;

    #[inline]
    fn deref(&self) -> &Options {
        &self.0
    }
}

impl DerefMut for Node {
    #[inline]
    fn deref_mut(&mut self) -> &mut Options {
        &mut self.0
    }
}
