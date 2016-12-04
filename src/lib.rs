use std::clone;
use std::mem;
use std::ptr;
use std::cmp;

extern crate libc;

struct Node<T> {
    data: T,
    right: *mut Node<T>,
    left: *mut Node<T>
}

pub struct Tree<T> {
    root: *mut Node<T>
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        let p_root = 0 as *mut Node<T>;
        Tree::<T>{ root: p_root }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    pub fn insert(&mut self, value: T) {
        unsafe {
            let node = libc::malloc(
                mem::size_of::<Node<T>>() as libc::size_t)
                as *mut Node<T>;
            ptr::write(node, Node {
                data: value.clone(),
                right: ptr::null_mut(),
                left: ptr::null_mut()
            });

            if self.root.is_null() {
                self.root = node;
            } else {
                let p_root = (*self).root;
                self.insert_to(p_root, value.clone());
            }
        }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    fn insert_to(&mut self, node: *mut Node<T>, value: T) {
        unsafe {
            let new_node = libc::malloc(
                mem::size_of::<Node<T>>() as libc::size_t)
                as *mut Node<T>;
            ptr::write(new_node, Node {
                data: value.clone(),
                right: ptr::null_mut(),
                left: ptr::null_mut()
            });

            let data = (*node).data.clone();
            if value < data {
                if (*node).left.is_null() {
                    (*node).left = new_node;
                } else {
                    self.insert_to((*node).left, value.clone());
                }
            } else {
                if (*node).right.is_null() {
                    (*node).right = new_node;
                } else {
                    self.insert_to((*node).right, value.clone());
                }
            }
        }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    pub fn find(&mut self, value: T) -> bool {
        if self.root.is_null() {
            return false;
        }

        let p_root = (*self).root;
        self.find_to(p_root, value.clone())
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    fn find_to(&mut self, node: *mut Node<T>, value: T) -> bool {
        unsafe {
            let data = (*node).data.clone();

            if data == value {
                return true;
            } else if value < data {
                if (*node).left.is_null() {
                    return false;
                } else {
                    self.find_to((*node).left, value.clone())
                }
            } else { // value > data
                if (*node).right.is_null() {
                    return false;
                } else {
                    self.find_to((*node).right, value.clone())
                }
            }
        }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    pub fn max<'a>(&mut self) -> Result<T, &'a str> {
        if self.root.is_null() {
            return Err("Empty tree");
        }

        let p_root = (*self).root;
        self.max_to(p_root)
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    fn max_to<'a>(&mut self, node: *const Node<T>) -> Result<T, &'a str> {
        if node.is_null() {
            return Err("Empty node");
        }

        unsafe {
            if (*node).right.is_null() {
                Ok((*node).data.clone())
            } else {
                self.max_to((*node).right)
            }
        }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    pub fn min<'a>(&mut self) -> Result<T, &'a str> {
        if self.root.is_null() {
            return Err("Empty tree");
        }

        let p_root = (*self).root;
        self.min_to(p_root)
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    fn min_to<'a>(&mut self, node: *const Node<T>) -> Result<T, &'a str> {
        if node.is_null() {
            return Err("Empty node");
        }

        unsafe {
            if (*node).left.is_null() {
                Ok((*node).data.clone())
            } else {
                self.min_to((*node).left)
            }
        }
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    pub fn delete(&mut self, value: T) {
        if self.root.is_null() {
            panic!("No such value");
        }

        let p_root = (*self).root;
        (*self).root = self.delete_to(p_root, value);
    }
}

impl<T> Tree<T> where T: cmp::PartialOrd + clone::Clone {
    fn delete_to(&mut self, node: *mut Node<T>, value: T) -> *mut Node<T> {
        unsafe {
            if node.is_null() {
                return node;
            }

            let data = (*node).data.clone();

            if data > value {
                (*node).left = self.delete_to((*node).left, value.clone());
            } else if data < value {
                (*node).right = self.delete_to((*node).right, value.clone());
            } else { // data == value
                if (*node).left.is_null() {
                    let temp_node = (*node).right;
                    libc::free(node as *mut libc::c_void);
                    return temp_node;
                } else if (*node).right.is_null() {
                    let temp_node = (*node).left;
                    libc::free(node as *mut libc::c_void);
                    return temp_node;
                } else {
                    let new_data = self.min_to((*node).right).unwrap();
                    (*node).data = new_data.clone();
                    (*node).right = self.delete_to((*node).right, new_data.clone());
                }
            }

            return node;
        }
    }
}


impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        if (*self).root.is_null() {
            return;
        }

        let p_root = (*self).root;
        self.drop_to(p_root);
    }
}

impl<T> Tree<T> {
    fn drop_to(&mut self, node: *mut Node<T>) {
        unsafe {
            if node.is_null() {
                return;
            }

            self.drop_to((*node).left);
            self.drop_to((*node).right);
            libc::free(node as *mut libc::c_void);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn test_insert() {
        let mut tree = Tree::new() as Tree<i32>;
        assert!(!tree.find(20));

        tree.insert(10);
        tree.insert(20);
        tree.insert(99);
        tree.insert(15);
        tree.insert(0);

        assert!(tree.find(20));
    }

    #[test]
    fn test_max() {
        let mut tree = Tree::new() as Tree<i32>;

        tree.insert(10);
        tree.insert(20);
        tree.insert(99);
        tree.insert(15);
        tree.insert(0);

        assert_eq!(tree.max().unwrap(), 99);
    }

    #[test]
    fn test_min() {
        let mut tree = Tree::new() as Tree<i32>;

        tree.insert(50);
        tree.insert(20);
        tree.insert(30);
        tree.insert(-1);

        assert_eq!(tree.min().unwrap(), -1);
    }

    #[test]
    fn test_delete() {
        let mut tree = Tree::new() as Tree<i32>;
        assert!(!tree.find(20));

        tree.insert(10);
        tree.insert(20);
        tree.insert(99);
        tree.insert(15);
        tree.insert(0);

        assert!(tree.find(20));

        tree.delete(20);
        assert!(!tree.find(20));
        assert!(tree.find(99));
        assert!(tree.find(15));

        tree.delete(99);
        assert!(!tree.find(99));
        assert!(tree.find(15));

        tree.delete(15);
        assert!(!tree.find(15));
        assert!(tree.find(10));
    }

    #[test]
    #[should_panic]
    fn test_delete_empty_tree() {
        let mut tree = Tree::new() as Tree<i32>;

        // Panic
        tree.delete(100);
    }
}
