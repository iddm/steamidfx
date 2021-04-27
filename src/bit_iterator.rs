//! Provides a simple implementation of an iterator over bits of an integer.
//! Used to navigate through the bits in the steam id.

/// Gets the bits starting from the specified position in the amount passed.
fn bits(num: u64, from: u8, amount: u8) -> u64 {
    assert!(amount <= 64);
    let length = from - amount;
    let mask = ((1 << amount) - 1) << length;
    (mask & num) >> length
}

/// An iterator over the bits of a `u64` number.
pub(crate) struct BitIterator {
    // The object it iterates over.
    object: u64,
    // The length in bits of a single iteration.
    iter_length: u8,
    // Current bit position within the `object`.
    current_pos: u8,
}

impl BitIterator {
    /// Instantiates a new bit iterator. Requires an object to iterate over and
    /// a length of a single iteration.
    pub fn new(object: u64, iter_length: u8) -> BitIterator {
        let current_pos = std::mem::size_of::<u64>() as u8 * 8;
        assert!(iter_length <= current_pos);
        BitIterator {
            object,
            iter_length,
            current_pos,
        }
    }

    /// Changes the length of a single iteration.
    pub fn change_iter_length(&mut self, iter_length: u8) -> &mut BitIterator {
        assert!(iter_length <= self.current_pos);
        self.iter_length = iter_length;
        self
    }

    /// Changes the iteration length to `amount` and tries to fetch the `amount` of bits
    /// from the `object`.
    pub fn next_bits<T: std::convert::TryFrom<u64>>(&mut self, amount: u8) -> Option<T> {
        self.change_iter_length(amount);
        if let Some(next) = self.next() {
            T::try_from(next).ok()
        } else {
            None
        }
    }
}

impl Iterator for BitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_pos = self.current_pos.checked_sub(self.iter_length)?;
        let item = bits(self.object, self.current_pos, self.iter_length) as Self::Item;
        self.current_pos = new_pos;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bit_iterator_test() {
        let num = 76561197983318796;
        let mut iter = crate::bit_iterator::BitIterator::new(num, 8);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(16));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(95));
        assert_eq!(iter.next(), Some(195));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }
}
