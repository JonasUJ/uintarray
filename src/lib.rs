//! An array packed in a single uint.

use std::convert::From;
use std::iter::IntoIterator;
use std::mem::size_of;

// Mask for the size part in the UintArray.
const SIZE_MASK: u128 = 0b111;
const SIZE_BITS: u128 = 3;

// Mask for the length part in the UintArray.
const LEN_MASK: u128 = 0b11111 << SIZE_BITS;
const LEN_BITS: u128 = 5;

// Meta makes up the non-data part of the UintArray.
// const META_MASK: u128 = SIZE_MASK | LEN_MASK;
const META_BITS: u128 = SIZE_BITS + LEN_BITS;

/// Multiple values stored in a single uint.
///
/// Can only contain values of the type specified at creation time.
#[derive(Copy, Clone)]
pub struct UintArray(pub u128);

/// Iteration over a UintArray.
pub struct UintArrayIterator {
    ua: UintArray,
    index: u128,
}

impl IntoIterator for UintArray {
    type Item = u128;
    type IntoIter = UintArrayIterator;

    fn into_iter(self) -> Self::IntoIter {
        UintArrayIterator {
            ua: self,
            index: 0,
        }
    }
}

impl Iterator for UintArrayIterator {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        self.index += 1;
        self.ua.at(self.index - 1)
    }
}

impl From<u128> for UintArray {
    /// Creates a new `UintArray` from the given uint.
    ///
    /// # Arguments
    ///
    /// * `data` - Source UintArray. Panics if invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::from(69420);
    ///
    /// assert_eq!(16, ua.size());
    /// ```
    fn from(data: u128) -> Self {
        let ua = UintArray(data);

        if ua.len() > ua.cap() {
            panic!("UintArray length={} exceeds cap={}.", ua.len(), ua.cap());
        }

        ua
    }
}

impl UintArray {
    /// Creates a new UintArray with a specific data type.
    /// Size of the data type cannot be more than half of the UintArray data type size.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// assert_eq!(8, ua.size());
    /// ```
    pub fn new<T>() -> Self {
        let size = size_of::<T>();
        Self::new_size(size * 8)
    }

    /// Creates a new UintArray with a specific data size.
    /// Size cannot be more than half of the UintArray data type size and must be a power of 2.
    ///
    /// # Arguments
    ///
    /// * `size` - The size in bits of the contained data.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(16);
    ///
    /// assert_eq!(16, ua.size());
    /// ```
    pub fn new_size(size: usize) -> Self {
        if size > size_of::<u128>() * 4 {
            panic!("Size must not be more than half of the UintArray data type size.");
        }

        let size_log_f: f32 = (size as f32).log2();
        let size_log_u = size_log_f as u128;

        if size_log_f != size_log_u as f32 {
            panic!("Size must be a power of 2.")
        }

        // TODO: Benchmark against this
        // if size & (size - 1) != 0 {
        //     panic!("Size must be a power of 2.")
        // }

        UintArray(size_log_u)
    }

    /// Creates a bit mask for a value of `size` bits.
    #[inline]
    fn _mask(size: u128) -> u128 {
        (1 << size) - 1
    }

    /// Updates the length of the UintArray.
    #[inline]
    fn _set_len(&self, new_len: u128) -> u128 {
        (self.0 & !LEN_MASK) | new_len << SIZE_BITS
    }

    /// Panics if a value cannot be inserted.
    fn _check_insert_panic(size: u128, len: u128, item: u128) {
        if len >= Self::_cap(size) {
            panic!("Attempted inserting beyond capacity.");
        }

        if Self::_mask(size) & item != item {
            panic!("item={} does not fit in size={}", item, size);
        }
    }

    // TODO: Implement
    // pub fn from_vec<T>(values: Vec::<T>) -> Self {
    //
    // }

    /// Gets the bit size of values stored in the UintArray.
    /// Same as what is passed to new_size().
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(2);
    ///
    /// assert_eq!(2, ua.size());
    /// ```
    #[inline]
    pub fn size(&self) -> u128 {
        Self::_size(self.0)
    }

    /// Gets the size encoded in `data`.
    #[inline]
    fn _size(data: u128) -> u128 {
        1 << (data & SIZE_MASK)
    }

    /// Gets the current length of the UintArray.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(2);
    ///
    /// let ua = ua
    ///     .append(1)
    ///     .append(2);
    ///
    /// assert_eq!(2, ua.len());
    /// ```
    #[inline]
    pub fn len(&self) -> u128 {
        Self::_len(self.0)
    }

    /// Gets the length encoded in `data`.
    #[inline]
    fn _len(data: u128) -> u128 {
        (data & LEN_MASK) >> SIZE_BITS
    }

    /// How many elements can be stored in the UintArray - its capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(4);
    ///
    /// assert_eq!(30, ua.cap());
    /// ```
    #[inline]
    pub fn cap(&self) -> u128 {
        Self::_cap(self.size())
    }

    /// Returns the capacity of a UintArray with size `size`.
    #[inline]
    fn _cap(size: u128) -> u128 {
        (size_of::<u128>() as u128 * 8 - META_BITS) / size
    }

    /// Get the item at position `pos`. First item is at `pos = 0` (i.e. it's zero-indexed).
    /// Returns None if out of bounds.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the item to get.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(4);
    ///
    /// let ua = ua
    ///     .append(2)
    ///     .append(4)
    ///     .append(8);
    ///
    /// assert_eq!(Some(4), ua.at(1))
    /// ```
    pub fn at(&self, pos: u128) -> Option<u128> {
        if pos >= self.len() {
            None
        } else {
            let size = self.size();
            let offset = size * pos + META_BITS;
            self._at(size, offset)
        }
    }

    /// Get the item at a given position, disregarding whether it exists.
    fn _at(&self, size: u128, offset: u128) -> Option<u128> {
        Some((Self::_mask(size) << offset & self.0) >> offset)
    }

    /// Creates a new UintArray with the given item appended to the end.
    /// Panics if appending would exceed capacity or if the item doesn't fit in the UintArray size.
    ///
    /// # Arguments
    ///
    /// * `item` - Item to append.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(4);
    ///
    /// let ua = ua
    ///     .append(1)
    ///     .append(2)
    ///     .append(3);
    ///
    /// assert_eq!(Some(1), ua.at(0));
    /// assert_eq!(3, ua.len());
    /// ```
    pub fn append(&self, item: u128) -> Self {
        let len = self.len();
        let size = self.size();

        Self::_check_insert_panic(size, len, item);

        UintArray(self._set_len(len + 1) | item << len * size + META_BITS)
    }

    /// Creates a new UintArray with the given item inserted at the given position.
    /// Panics if appending would exceed capacity or if the item doesn't fit in the UintArray size.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position to insert the item at.
    /// * `item` - The item to insert.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new_size(4);
    ///
    /// let ua = ua
    ///     .append(1)
    ///     .append(2)
    ///     .insert(1, 3);
    ///
    /// assert_eq!(Some(3), ua.at(1));
    /// ```
    pub fn insert(&self, pos: u128, item: u128) -> Self {
        let len = self.len();
        let size = self.size();
        Self::_check_insert_panic(size, len, item);

        // TODO: Use .append in this case?
        let pos = if pos > len { len } else { pos };

        let offset = pos * size + META_BITS;
        let pos_mask = Self::_mask(offset);

        // Pushes everything after the offset off by `size` and inserts the item inbetween.
        //
        // If offset is at 4, the `size` is 2 and the new item is AA,
        // it will do the following:
        //
        // 000011110000 -> 0000    0000 -> 001111  0000 -> 001111AA0000
        //                   1111                AA
        UintArray(self._set_len(len + 1) & pos_mask | (self.0 & !pos_mask) << size | item << offset)
    }

    /// Extends the UintArray with the values of the iterator.
    /// Panics if inserting would exceed the capacity or an item is greater than size.
    ///
    /// # Arguments
    ///
    /// * `iter` - Iterator of items to append.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua.extend(1..4);
    ///
    /// assert_eq!(Some(2), ua.at(1));
    /// assert_eq!(3, ua.len());
    /// ```
    pub fn extend<T: IntoIterator<Item = u128>>(&self, iter: T) -> Self {
        let len = self.len();
        let size = self.size();
        let cap = self.cap();

        let mut iter_len: u128 = 0;
        let mut max: u128 = 0;
        let mut items: u128 = 0;

        for i in iter {
            iter_len += 1;

            if i > max {
                max = i;
            }

            if iter_len > cap {
                panic!("Cannot extend beyond capacity.");
            }

            // Everything is put in sequence in `items`.
            items = items | i << (iter_len - 1) * size;
        }

        let new_len = len + iter_len;

        // We got the max, so we only need to check once.
        Self::_check_insert_panic(size, new_len, max);

        // Add `items` to the end.
        UintArray(self._set_len(new_len) | items << size * len + META_BITS)
    }

    /// Clears all values from the UintArray.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua
    ///     .append(15)
    ///     .append(14)
    ///     .clear();
    ///
    /// assert_eq!(0, ua.len());
    /// assert_eq!(8, ua.size());
    /// ```
    #[inline]
    pub fn clear(&self) -> Self {
        UintArray(self.0 & SIZE_MASK)
    }

    /// Removes the first occurrence of an item from the UintArray.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua
    ///     .extend(1..4)
    ///     .remove(2);
    ///
    /// assert_eq!(Some(3), ua.at(1));
    /// assert_eq!(2, ua.len());
    /// ```
    pub fn remove(&self, item: u128) -> Self {
        let len = self.len();
        let size = self.size();

        let pos = self._index(item, len, size);

        let pos = match pos {
            Some(i) => i,
            None => return *self,
        };

        let offset = pos * size + META_BITS;
        let pos_mask = Self::_mask(offset);

        // Same operation as that of self.pop()
        UintArray(self._set_len(len - 1) & pos_mask | (self.0 & !pos_mask) >> size & !pos_mask)
    }

    /// Removes an item from the UintArray at a given index and returns it and the UintArray.
    ///
    /// # Arguments
    ///
    /// * `pos` - The index of the item to remove and return.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua
    ///     .extend(1..4);
    ///
    /// let (ua, item) = ua.pop(1);
    ///
    /// assert_eq!(Some(2), item);
    /// ```
    pub fn pop(&self, pos: u128) -> (Self, Option<u128>) {
        let len = self.len();
        let size = self.size();

        if pos >= len {
            return (*self, None);
        }

        let offset = pos * size + META_BITS;
        let pos_mask = Self::_mask(offset);

        (
            // Move everything after `pos` down by `size`, discarding any overlap and effectivly
            // removing the item in question.
            //
            // If the offset is at the transition from 0's to 1's and the size is 2,
            // it will do the following:
            //
            // 1111110000 ->     0000 -> 11110000
            //               111111
            UintArray(self._set_len(len - 1) & pos_mask | (self.0 & !pos_mask) >> size & !pos_mask),
            self._at(size, offset),
        )
    }

    /// Returns the index of the first occurrence of an item in the UintArray.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to return the index of.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua
    ///     .extend(1..4);
    ///
    /// assert_eq!(Some(1), ua.index(2));
    /// ```
    pub fn index(&self, item: u128) -> Option<u128> {
        let len = self.len();
        let size = self.size();
        self._index(item, len, size)
    }

    /// Returns the index of the first occurrence of an item in the UintArray.
    fn _index(&self, item: u128, len: u128, size: u128) -> Option<u128> {
        let mut pos = 0;
        self._until(len, size, |x| {
            pos += 1;
            // Search until x == item
            (pos - 1, x == item)
        })
    }

    /// Run a function to all items in the UintArray until it signals to stop
    /// and return the final value.
    fn _until<F>(&self, len: u128, size: u128, mut f: F) -> Option<u128>
    where
        F: FnMut(u128) -> (u128, bool),
    {
        let mask = Self::_mask(size);

        for i in 0..len {
            let offset = i * size + META_BITS;

            // Apply f to current item
            let (value, stop) = f((self.0 & mask << offset) >> offset);

            if stop {
                return Some(value);
            }
        }

        None
    }

    /// Returns the number of occurrences of an item in the UintArray.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to return count of.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua
    ///     .append(1)
    ///     .append(2)
    ///     .append(1)
    ///     .append(1);
    ///
    /// assert_eq!(3, ua.count(1));
    /// ```
    pub fn count(&self, item: u128) -> u128 {
        self.aggregate(|x| if x == item { 1 } else { 0 })
    }

    /// Aggregate the elements of the UintArray into a single u128.
    ///
    /// # Arguments
    ///
    /// * `f` - A function applied to each element of the UintArray.
    ///
    /// # Examples
    ///
    /// ```
    /// use uintarray::UintArray;
    /// let ua = UintArray::new::<u8>();
    ///
    /// let ua = ua.extend(1..4);
    ///
    /// // Very simple function that just returns the value itself
    /// let sum = ua.aggregate(|x| x);
    ///
    /// assert_eq!(6, sum);
    /// ```
    pub fn aggregate<F>(&self, f: F) -> u128
    where
        F: Fn(u128) -> u128,
    {
        self._aggregate(self.len(), self.size(), f)
    }

    /// Aggregate the elements of the UintArray into a single u128.
    fn _aggregate<F>(&self, len: u128, size: u128, f: F) -> u128
    where
        F: Fn(u128) -> u128,
    {
        let mut n = 0;
        self._apply(len, size, |x| n += f(x));
        n
    }

    /// Apply a function to all items in the UintArray.
    fn _apply<F>(&self, len: u128, size: u128, mut f: F)
    where
        F: FnMut(u128),
    {
        let mask = Self::_mask(size);

        for i in 0..len {
            let offset = i * size + META_BITS;

            // Apply f to current item
            f((self.0 & mask << offset) >> offset);
        }
    }

    /// Returns a prettily formatted representation of the UintArray.
    pub fn format(&self) -> String {
        let mut formatted = String::new();
        let size = self.size();

        for i in (0..size_of::<u128>() as u128 * 8).rev() {
            formatted.push(if self.0 & 1 << i == 0 { '0' } else { '1' });

            if i % 32 == 0 {
                formatted.push('\n');
            } else if i % size == 0 {
                formatted.push(' ');
            }
        }

        formatted
    }
}
