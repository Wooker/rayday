use centered_interval_tree::{inner_info::InnerInfo, CenteredIntervalTree, Node};
use std::{cell::RefCell, fmt::Debug, ops::Add, rc::Rc};

pub struct CenTreeNodeIterator<I, V> {
    pub(crate) stack: Vec<(Option<Rc<RefCell<Node<I, V>>>>, usize, bool, usize)>,
}

impl<I, V> Iterator for CenTreeNodeIterator<I, V>
where
    I: Clone + Debug + PartialOrd,
    V: Clone + Debug,
{
    type Item = (InnerInfo<I, V>, usize, bool, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, layer, mut has_overlaps, mut width)) = self.stack.pop() {
            let info = node.as_ref().unwrap().borrow().info.clone();

            if let Some(right) = node.as_ref().unwrap().borrow().right.as_ref() {
                self.stack.push((Some(Rc::clone(right)), layer, false, 1));
            }
            if let Some(center) = &node.as_ref().unwrap().borrow().center {
                has_overlaps = true;
                self.stack
                    .push((Some(Rc::clone(&center)), layer + 1, false, 1));
            }
            if let Some(left) = &node.as_ref().unwrap().borrow().left {
                self.stack.push((Some(Rc::clone(&left)), layer, false, 1));
            }

            return Some((info, layer, has_overlaps, 1));
        }

        None
    }
}

pub(crate) trait CenTreeIterator<I, V>
where
    I: PartialOrd + Ord + Clone + Debug,
    V: Clone + Ord + Debug,
{
    fn events_iter(&self) -> CenTreeNodeIterator<I, V>;
}

impl<I, V> CenTreeIterator<I, V> for CenteredIntervalTree<I, V>
where
    I: PartialOrd + Ord + Clone + Debug,
    V: Clone + Ord + Debug,
{
    fn events_iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();

        if let Some(root) = self.inner.as_ref() {
            stack.push((Some(Rc::clone(root)), 0, false, 1));
        }

        CenTreeNodeIterator::<I, V> { stack }
    }
}
