use std::cmp::min;

/// A data stream.
/// Used by the Parser to iterate over the supplied JSON data and to parse it into Values.
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
    pub fn next_entry(&mut self) -> Option<&T> {
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

    /// Returns a reference to the previous entry in the stream
    pub fn peek_back(&self) -> Option<&T> {
        self.data.get(self.position() - 1)
    }

    /// The current position of the stream on the data
    pub fn position(&self) -> usize {
        self.index
    }

    /// Reset the stream read head to index 0.
    pub fn reset(&mut self) {
        self.index = 0;
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
