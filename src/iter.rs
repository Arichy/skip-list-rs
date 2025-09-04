use std::mem::ManuallyDrop;

use crate::{Key, NodePtr, SkipList, Value};

pub struct SkipListIntoIter<K: Key, V: Value> {
    skip_list: ManuallyDrop<SkipList<K, V>>,
    ptr: NodePtr<K, V>,
}

impl<K: Key, V: Value> Iterator for SkipListIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_list.is_tail(self.ptr) {
            return None;
        }

        let next = unsafe { self.ptr.as_ref() }.forward[0].ptr;

        let node = unsafe { Box::from_raw(self.ptr.as_ptr()) };
        let key = unsafe { node.key.assume_init() };
        let value = unsafe { node.value.assume_init() };

        self.ptr = next;

        Some((key, value))
    }
}

impl<K: Key, V: Value> IntoIterator for SkipList<K, V> {
    type IntoIter = SkipListIntoIter<K, V>;
    type Item = (K, V);

    fn into_iter(self) -> Self::IntoIter {
        let first = unsafe { self.head.as_ref() }.forward[0].ptr;

        SkipListIntoIter {
            skip_list: ManuallyDrop::new(self),
            ptr: first,
        }
    }
}

impl<K: Key, V: Value> Drop for SkipListIntoIter<K, V> {
    fn drop(&mut self) {
        for _ in &mut *self {}

        unsafe {
            let _ = Box::from_raw(self.skip_list.head.as_ptr());
            let _ = Box::from_raw(self.skip_list.tail.as_ptr());
        }
    }
}

pub struct SkipListIter<'a, K: Key, V: Value> {
    skip_list_ref: &'a SkipList<K, V>,
    ptr: NodePtr<K, V>,
}

impl<'a, K: Key, V: Value> Iterator for SkipListIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_list_ref.is_tail(self.ptr) {
            return None;
        }

        let next = unsafe { self.ptr.as_ref() }.forward[0].ptr;

        let key = unsafe { self.ptr.as_ref() }.key();
        let value = unsafe { self.ptr.as_ref() }.value();

        self.ptr = next;

        Some((key, value))
    }
}

impl<'a, K: Key, V: Value> IntoIterator for &'a SkipList<K, V> {
    type IntoIter = SkipListIter<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        let first = unsafe { self.head.as_ref() }.forward[0].ptr;

        SkipListIter {
            skip_list_ref: self,
            ptr: first,
        }
    }
}

impl<'a, K: Key, V: Value> SkipList<K, V> {
    pub fn iter(&'a self) -> SkipListIter<'a, K, V> {
        let first = unsafe { self.head.as_ref() }.forward[0].ptr;

        SkipListIter {
            skip_list_ref: self,
            ptr: first,
        }
    }
}

// pub struct SkipListIterMut<'a, K: Key, V: Value> {
//     skip_list_mut: &'a mut SkipList<K, V>,
//     ptr: NodePtr<K, V>,
// }

// impl<'a, K: Key, V: Value> Iterator for SkipListIterMut<'a, K, V> {
//     type Item = (&'a K, &'a mut V);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.skip_list_mut.is_tail(self.ptr) {
//             return None;
//         }

//         let next = unsafe { self.ptr.as_ref() }.forward[0].ptr;

//         let key = unsafe { self.ptr.as_ref() }.key();
//         let value = unsafe { self.ptr.as_mut() }.value_mut();

//         self.ptr = next;

//         Some((key, value))
//     }
// }

// impl<'a, K: Key, V: Value> IntoIterator for &'a mut SkipList<K, V> {
//     type IntoIter = SkipListIterMut<'a, K, V>;
//     type Item = (&'a K, &'a mut V);

//     fn into_iter(self) -> Self::IntoIter {
//         let first = unsafe { self.head.as_ref() }.forward[0].ptr;

//         SkipListIterMut {
//             skip_list_mut: self,
//             ptr: first,
//         }
//     }
// }
