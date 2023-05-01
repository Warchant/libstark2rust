use crate::glue::libstark::{sequence_usize_get_index, sequence_usize_size, SequenceUsize};

pub trait Sequence<T> {
    fn get_element_at_index(&self, idx: usize) -> T;
    fn size(&self) -> usize;
}

impl Sequence<usize> for SequenceUsize {
    fn get_element_at_index(&self, idx: usize) -> usize {
        return sequence_usize_get_index(&self, idx);
    }

    fn size(&self) -> usize {
        return sequence_usize_size(&self);
    }
}

impl<'a> IntoIterator for &'a SequenceUsize {
    type Item = usize;
    type IntoIter = SequenceUsizeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceUsizeIterator {
            seq: &self,
            index: 0,
        }
    }
}

pub struct SequenceUsizeIterator<'a> {
    seq: &'a SequenceUsize,
    index: usize,
}

impl Iterator for SequenceUsizeIterator<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.index >= 0 && self.index < self.seq.size() {
            let value = self.seq.get_element_at_index(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
