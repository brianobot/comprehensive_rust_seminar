struct SliceIter<'a, T> {
    slice: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for SliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.slice.len() {
            return None;
        } else {
            let next = &self.slice[self.index];
            self.index += 1;
            return Some(next);
        }
    }
}

fn main() {
    let int_slice = &[1, 2, 3, 4, 5, 6, 7];
    let char_slice = &['a', 'b', 'c', 'd'];

    let int_slice_iter = SliceIter {
        slice: int_slice,
        index: 0,
    };
    let char_slice_iter = SliceIter {
        slice: char_slice,
        index: 0,
    };

    for item in int_slice_iter {
        println!("Item: {:?}", item);
    }

    for item in char_slice_iter {
        println!("Char: {:?}", item);
    }

    // the IntoIterator trait creates an iterator from a type
    // types like Vec<T>, &Vec<T>, &[T] implement this trait
    // this is why you can call these types in for loop without them directly having the next method
    //
    let list = vec![1, 2, 3, 4, 5];
    // this for loop takes ownership of the items in the list, this is because IntoIterator::into_iter takes ownership of self
    for item in list {
        println!("Item in list: {}", item);
    }
}
