use crate::glue::libstark::{
    sequence_usize_get_index, sequence_usize_size, sequence_color_size, SequenceColor, SequenceUsize, sequence_color_get_index,
};

pub trait Sequence<T>  {
    fn get_element_at_index(&self, idx: usize) -> T;
    fn size(&self) -> usize;
}

////////////// SequenceUsize
impl Sequence<usize> for SequenceUsize {
    fn get_element_at_index(&self, idx: usize) -> usize {
        return sequence_usize_get_index(&self, idx);
    }

    fn size(&self) -> usize {
        return sequence_usize_size(&self);
    }
}

impl Sequence<Vec<usize>> for SequenceColor {
    fn get_element_at_index(&self, idx: usize) -> Vec<usize> {
        return sequence_color_get_index(&self, idx);
    }

    fn size(&self) -> usize {
        return sequence_color_size(&self);
    }
}


impl<'a, T> IntoIterator for &'a (dyn Sequence<T>) {
    type Item = T;
    type IntoIter = SequenceIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIter {
            seq: self,
            index: 0,
        }
    }
}

pub struct SequenceIter<'a, T> {
    seq: &'a dyn Sequence<T>,
    index: usize,
}

impl<'a, T> Iterator for SequenceIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.seq.size() {
            let value = self.seq.get_element_at_index(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
