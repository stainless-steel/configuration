use options::Options;
use std::any::Any;
use std::ops::{Deref, DerefMut};

/// A node.
pub struct Node(Options);

impl Node {
    /// Look up a value.
    pub fn get<'l, T: Any>(&'l self, path: &str) -> Option<&'l T> {
        let chunks = path.split('.').collect::<Vec<_>>();
        let count = chunks.len();
        let mut current = self;
        let mut i = 0;
        while i < count {
            if i + 1 == count {
                return current.get_ref(chunks[i]);
            }
            if let Some(node) = current.get_ref::<Node>(chunks[i]) {
                i += 1;
                current = node;
            } else if let Some(array) = current.get_ref::<Vec<Node>>(chunks[i]) {
                i += 1;
                match chunks[i].parse::<usize>() {
                    Ok(j) => if i + 1 == count {
                        return Any::downcast_ref(&array[j]);
                    } else {
                        i += 1;
                        current = &array[j];
                    },
                    _ => return None,
                }
            } else {
                return None;
            }
        }
        unreachable!();
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
