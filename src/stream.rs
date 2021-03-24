use std::cmp::min;

pub struct Stream<'a, T> {
    index: usize,
    data: &'a [T],
}

impl<'a, T: Copy> Stream<'a, T> {
    /// Returns the current character.
    pub fn current_copied(&self) -> Option<T> {
        self.data.get(self.index).copied()
    }

    /// Returns the current element and panicks if it's out of range
    pub fn current_unchecked(&self) -> T {
        self.data[self.index]
    }

    /// Moves to the next character and returns it.
    pub fn next_copied(&mut self) -> Option<T> {
        self.skip();
        self.data.get(self.index).copied()
    }

    /// Returns a reference to the perevious entry in the stream
    pub fn peek_back_copied(&self) -> Option<T> {
        self.data.get(self.position() - 1).copied()
    }
}

impl<'a, T: Eq + Copy> Stream<'a, T> {
    /// Returns the current character and moves to the next character if the character matches
    /// the provided input.
    pub fn next_if_current_is(&mut self, expect: T) -> Option<T> {
        self.next_if_current_present_in(&[expect])
    }

    /// Returns the current character and moves to the next character if the character exists
    /// in the provided array.
    pub fn next_if_current_present_in(&mut self, expect: &[T]) -> Option<T> {
        let c = self.current_copied()?;

        if expect.contains(&c) {
            self.skip();
            return Some(c);
        }

        None
    }

    /// Skips the current character if it matches the input, and returns true in this case.
    /// Returns false and does not skip if the current character does not match the input.
    pub fn skip_if_current_is(&mut self, expect: T) -> bool {
        self.next_if_current_is(expect)
            .map(|c| c == expect)
            .unwrap_or(false)
    }
}

impl<'a, T> Stream<'a, T> {
    /// Creates a new stream
    pub fn new(data: &'a [T]) -> Stream<T> {
        Self { data, index: 0 }
    }

    /// Returns the current element
    pub fn current(&self) -> Option<&T> {
        self.data.get(self.index)
    }

    /// Checks whether the stream has reached the end
    pub fn is_eof(&self) -> bool {
        self.index >= self.data.len()
    }

    /// Get the length of the stream.
    pub fn length(&self) -> usize {
        self.data.len()
    }
 
    /// Returns the next element
    pub fn next(&mut self) -> Option<&T> {
        self.data.get(self.index + 1).map(|c| {
            self.skip();
            c
        })
    }

    /// Returns the next element and panicks if it's out of bounds
    pub fn next_unchecked(&mut self) -> &T {
        self.skip();
        self.data.get(self.index).unwrap()
    }

    /// Returns a reference to the next entry in the stream
    pub fn peek(&self) -> Option<&T> {
        self.data.get(self.position() + 1)
    }

    /// Returns a reference to the perevious entry in the stream
    pub fn peek_back(&self) -> Option<&T> {
        self.data.get(self.position() - 1)
    }

    /// The current position of the stream on the data
    pub fn position(&self) -> usize {
        self.index
    }

    /// Skips the current character and moves on to the next one
    pub fn skip(&mut self) {
        self.index += 1;
    }

    /// Skips n amount of characters
    pub fn skip_n(&mut self, n: usize) {
        self.index += n;
    }

    /// Moves back to the previous character
    pub fn unskip(&mut self) {
        self.index -= 1;
    }

    /// Returns a subslice of this stream but also checks stream length
    /// to prevent out of bounds panicking
    pub fn slice(&self, from: usize, to: usize) -> &'a [T] {
        &self.data[from..min(self.data.len(), to)]
    }

    /// Returns a subslice of this stream
    pub fn slice_unchecked(&self, from: usize, to: usize) -> &'a [T] {
        &self.data[from..to]
    }

    /// Same as slice, but the second argument is how many elements to slice
    pub fn slice_len(&self, from: usize, len: usize) -> &'a [T] {
        self.slice(from, self.index + len)
    }
}
