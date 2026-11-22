use core::cell::Cell;
use core::fmt;

pub const MAX_TWEAKS: usize = 256;
const WORDS: usize = MAX_TWEAKS / 64;

/// Fixed-width 256-bit mask with O(1) set algebra and zero heap allocation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BitSet256 {
    words: [u64; WORDS],
}

impl BitSet256 {
    pub const EMPTY: Self = Self { words: [0; WORDS] };

    #[inline]
    pub const fn new() -> Self {
        Self::EMPTY
    }

    #[inline]
    pub fn set(&mut self, idx: usize) {
        if idx < MAX_TWEAKS {
            self.words[idx >> 6] |= 1u64 << (idx & 63);
        }
    }

    #[inline]
    pub fn clear(&mut self, idx: usize) {
        if idx < MAX_TWEAKS {
            self.words[idx >> 6] &= !(1u64 << (idx & 63));
        }
    }

    #[inline]
    pub fn set_to(&mut self, idx: usize, value: bool) {
        if value {
            self.set(idx);
        } else {
            self.clear(idx);
        }
    }

    #[inline]
    pub fn toggle(&mut self, idx: usize) -> bool {
        if idx >= MAX_TWEAKS {
            return false;
        }
        let mask = 1u64 << (idx & 63);
        self.words[idx >> 6] ^= mask;
        (self.words[idx >> 6] & mask) != 0
    }

    #[inline]
    pub const fn contains(&self, idx: usize) -> bool {
        if idx >= MAX_TWEAKS {
            return false;
        }
        (self.words[idx >> 6] & (1u64 << (idx & 63))) != 0
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        (self.words[0] | self.words[1] | self.words[2] | self.words[3]) == 0
    }

    #[inline]
    pub const fn count(&self) -> u32 {
        self.words[0].count_ones()
            + self.words[1].count_ones()
            + self.words[2].count_ones()
            + self.words[3].count_ones()
    }

    #[inline]
    pub const fn and(self, rhs: Self) -> Self {
        Self {
            words: [
                self.words[0] & rhs.words[0],
                self.words[1] & rhs.words[1],
                self.words[2] & rhs.words[2],
                self.words[3] & rhs.words[3],
            ],
        }
    }

    #[inline]
    pub const fn or(self, rhs: Self) -> Self {
        Self {
            words: [
                self.words[0] | rhs.words[0],
                self.words[1] | rhs.words[1],
                self.words[2] | rhs.words[2],
                self.words[3] | rhs.words[3],
            ],
        }
    }

    #[inline]
    pub const fn xor(self, rhs: Self) -> Self {
        Self {
            words: [
                self.words[0] ^ rhs.words[0],
                self.words[1] ^ rhs.words[1],
                self.words[2] ^ rhs.words[2],
                self.words[3] ^ rhs.words[3],
            ],
        }
    }

    #[inline]
    pub const fn and_not(self, rhs: Self) -> Self {
        Self {
            words: [
                self.words[0] & !rhs.words[0],
                self.words[1] & !rhs.words[1],
                self.words[2] & !rhs.words[2],
                self.words[3] & !rhs.words[3],
            ],
        }
    }

    #[inline]
    pub const fn is_subset_of(self, superset: Self) -> bool {
        self.and_not(superset).is_empty()
    }

    #[inline]
    pub const fn first_one(&self) -> Option<usize> {
        let mut w = 0;
        while w < WORDS {
            let word = self.words[w];
            if word != 0 {
                return Some((w << 6) + word.trailing_zeros() as usize);
            }
            w += 1;
        }
        None
    }

    /// Deterministic 64-bit FNV-1a fingerprint for confirmation tokens.
    #[inline]
    pub const fn fingerprint(&self) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        let mut i = 0;
        while i < WORDS {
            let bytes = self.words[i].to_le_bytes();
            let mut b = 0;
            while b < 8 {
                hash ^= bytes[b] as u64;
                hash = hash.wrapping_mul(0x0100_0000_01b3);
                b += 1;
            }
            i += 1;
        }
        hash
    }

    #[inline]
    pub const fn iter_ones(&self) -> BitSetIter {
        BitSetIter {
            words: self.words,
            word_idx: 0,
        }
    }

    #[inline]
    pub const fn raw_words(&self) -> [u64; WORDS] {
        self.words
    }
}

pub struct BitSetIter {
    words: [u64; WORDS],
    word_idx: usize,
}

impl Iterator for BitSetIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.word_idx < WORDS {
            let w = self.words[self.word_idx];
            if w != 0 {
                let tz = w.trailing_zeros() as usize;
                self.words[self.word_idx] = w & (w - 1);
                return Some((self.word_idx << 6) + tz);
            }
            self.word_idx += 1;
        }
        None
    }
}

/// Single-threaded UI-safe bitset shared with Slint `FilterModel` predicates.
pub struct CellBitSet256 {
    words: [Cell<u64>; WORDS],
}

impl CellBitSet256 {
    pub fn new(initial: BitSet256) -> Self {
        let w = initial.raw_words();
        Self {
            words: [
                Cell::new(w[0]),
                Cell::new(w[1]),
                Cell::new(w[2]),
                Cell::new(w[3]),
            ],
        }
    }

    #[inline]
    pub fn replace(&self, mask: BitSet256) {
        let w = mask.raw_words();
        for (i, &word) in w.iter().enumerate().take(WORDS) {
            self.words[i].set(word);
        }
    }

    #[inline]
    pub fn contains(&self, idx: usize) -> bool {
        if idx >= MAX_TWEAKS {
            return false;
        }
        (self.words[idx >> 6].get() & (1u64 << (idx & 63))) != 0
    }
}

/// Zero-allocation stack string formatter with UTF-8 safe truncation.
pub struct StackFmt<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StackFmt<N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        // Safe because `write_str` only appends valid UTF-8 slices at char boundaries.
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }
}

impl<const N: usize> Default for StackFmt<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for StackFmt<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let avail = N.saturating_sub(self.len);
        if avail == 0 {
            return Ok(());
        }
        let mut take = s.len().min(avail);
        while take > 0 && !s.is_char_boundary(take) {
            take -= 1;
        }
        self.buf[self.len..self.len + take].copy_from_slice(&s.as_bytes()[..take]);
        self.len += take;
        Ok(())
    }
}

/// Zero-allocation ASCII case-insensitive substring matcher for live search filtering.
#[inline]
pub fn contains_ascii_icase(haystack: &str, needle: &str) -> bool {
    let n = needle.as_bytes();
    if n.is_empty() {
        return true;
    }
    let h = haystack.as_bytes();
    if n.len() > h.len() {
        return false;
    }
    h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n))
}
