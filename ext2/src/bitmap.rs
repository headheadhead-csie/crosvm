// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Defines bitmap types.

pub struct BitMap<'a> {
    inner: &'a mut [u8],
}

impl<'a> BitMap<'a> {
    /// Creates a new bitmap from an underlying buffer.
    pub fn from_slice_mut(inner: &'a mut [u8]) -> Self {
        Self { inner }
    }

    /// Marks the first `n` bits in the bitmap with the given value.
    pub fn mark_first_elems(&mut self, n: usize, value: bool) {
        self.inner
            .iter_mut()
            .take(n / 8)
            .for_each(|v| *v = if value { 0xff } else { 0 });

        if n % 8 != 0 {
            if value {
                self.inner[n / 8] |= 0xff >> (8 - n % 8);
            } else {
                self.inner[n / 8] &= !(0xff >> (8 - n % 8));
            }
        }
    }
}

// Implements test utility methods.
#[cfg(test)]
impl<'a> BitMap<'a> {
    // Returns the number of bits in the bitmap.
    pub fn len(&self) -> usize {
        self.inner.len() * 8
    }

    // Returns the number of bits in the bitmap that are set.
    pub fn count_zeros(&self) -> usize {
        self.inner.iter().map(|b| b.count_zeros() as usize).sum()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_first_elems() {
        let mut s = [0; 10];
        let mut b = BitMap::from_slice_mut(&mut s);
        b.mark_first_elems(28, true);
        // (28 + 1) = 8 * 3 + 4. So, the first 3 bytes and 4 bits should be set.
        assert_eq!(b.as_slice(), &[0xff, 0xff, 0xff, 0b1111, 0, 0, 0, 0, 0, 0]);
    }
}
