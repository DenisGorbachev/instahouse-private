pub trait InfiniteIterator {
    type Item;

    fn next(&mut self) -> Self::Item;
}

pub trait InfiniteIteratorExt: InfiniteIterator + Sized {
    fn into_iter(self) -> InfiniteIteratorAdapter<Self> {
        InfiniteIteratorAdapter {
            iter: self,
        }
    }
}

impl<T: InfiniteIterator + Sized> InfiniteIteratorExt for T {}

pub struct InfiniteIteratorAdapter<T: InfiniteIterator> {
    iter: T,
}

impl<T: InfiniteIterator> Iterator for InfiniteIteratorAdapter<T> {
    type Item = <T as InfiniteIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next())
    }
}
