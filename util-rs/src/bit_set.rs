#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::usize;

type BitSetElement = usize;

#[derive(Clone)]
struct BitSet {
    size: usize,
    storage: Vec<BitSetElement>,
}

impl BitSet {
    fn unit_size() -> usize {
        size_of::<BitSetElement>()
    }

    fn unit_size_bit() -> usize {
        8 * Self::unit_size()
    }

    fn new(size: usize, default: bool) -> BitSet {
        if size == 0 {
            return Self::empty();
        }

        let unit_size_bit = Self::unit_size_bit();
        let storage_size = (size - 1) / unit_size_bit + 1;
        let mut storage = Vec::new();
        storage.resize(storage_size, 0);

        if default {
            for i in 0..storage_size - 1 {
                storage[i] = (-1i8) as BitSetElement;
            }
            storage[storage_size - 1] =
                ((-1i8) as BitSetElement) >> (unit_size_bit - (size - 1) % unit_size_bit - 1);
        }
        BitSet {
            size: size,
            storage: storage,
        }
    }

    fn empty() -> BitSet {
        BitSet {
            size: 0,
            storage: Vec::new(),
        }
    }

    fn one() -> BitSet {
        Self::new(1, true)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn set(&mut self, pos: usize) {
        self.storage[pos / Self::unit_size_bit()] |= 1 << (pos % Self::unit_size_bit());
    }

    fn unset(&mut self, pos: usize) {
        self.storage[pos / Self::unit_size_bit()] &= !(1 << (pos % Self::unit_size_bit()));
    }

    fn get(&self, pos: usize) -> bool {
        if pos >= self.size {
            return false;
        }
        self.storage[pos / Self::unit_size_bit()] & (1 << (pos % Self::unit_size_bit())) > 0
    }

    /// self.get(i) | other.get(i-shift)
    fn or_with_shift(&self, other: &BitSet, shift: usize) -> BitSet {
        if self.size == 0 && other.size == 0 {
            return self.clone();
        }

        let unit_size_bit = Self::unit_size_bit();
        let shift_elem = shift / unit_size_bit;
        let bit_size = max(self.size, other.size + shift);
        let byte_size = (bit_size - 1) / unit_size_bit + 1;
        let mut storage = Vec::with_capacity(byte_size);

        for i in 0..byte_size {
            storage.push(0);
            if shift_elem + 1 <= i && i < other.storage.len() + shift_elem + 1
                && shift % unit_size_bit > 0
            {
                storage[i] |=
                    other.storage[i - 1 - shift_elem] >> (unit_size_bit - shift % unit_size_bit);
            }
            if shift_elem <= i && i < other.storage.len() + shift_elem {
                storage[i] |= other.storage[i - shift_elem] << (shift % unit_size_bit);
            }
            if i < self.storage.len() {
                storage[i] |= self.storage[i];
            }
        }

        BitSet {
            size: bit_size,
            storage: storage,
        }
    }

    fn increase_size(&mut self, new_size: usize, v: bool) {
        if new_size <= self.size {
            return;
        }

        if self.size == 0 {
            *self = BitSet::new(new_size, v);
            return;
        }

        let unit_size_bit = Self::unit_size_bit();
        let current_bytes = self.storage.len();
        let new_bytes = (new_size - 1) / unit_size_bit + 1;

        if v {
            self.storage.resize(new_bytes, -1i8 as BitSetElement);

            let used_bits = (self.size - 1) % unit_size_bit + 1;
            let rem_in_current_byte = if current_bytes == new_bytes {
                new_size - self.size
            } else {
                unit_size_bit - used_bits
            };

            if rem_in_current_byte > 0 {
                self.storage[current_bytes - 1] |= ((1 << rem_in_current_byte) - 1) << used_bits;
            }
        } else {
            self.storage.resize(new_bytes, 0);
        }

        self.size = new_size;
    }

    fn dump(&self) {
        if self.storage.is_empty() {
            return;
        }

        let unit_size_bit = Self::unit_size_bit();

        for x in self.storage.iter().rev().skip(1).rev() {
            print!("{:01$b}", x, unit_size_bit);
        }
        let last_used_bits = (self.size - 1) % unit_size_bit + 1;
        print!("{:01$b}", self.storage.last().unwrap(), last_used_bits);
        println!("");
    }
}

mod test {
    use super::*;
    #[test]
    fn bit_set() {
        let mut b = BitSet::new(256, false);
        b.set(65);
        b.set(190);
        let c = b.or_with_shift(&b, 64);

        b.dump();
        c.dump();

        assert!(!c.get(0));
        assert!(c.get(65));
        assert!(c.get(129));
        assert!(c.get(190));
        assert!(c.get(254));
        assert!(!c.get(270));
    }
}
