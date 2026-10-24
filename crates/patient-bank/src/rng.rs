//! CPython's `random.Random`, exactly.
//!
//! The panel design, the renderer's filled lab values and result delays, and the
//! validator's snapshot instants all came from Python's Mersenne Twister. The
//! bank stays byte-for-byte reproducible only if these draws replay identically,
//! so this is a direct port of CPython 3.14 `_randommodule.c` and `random.py`
//! for the calls the bank uses: integer and string seeds, `random`, `uniform`,
//! `gauss`, `randint`, `choice`, `sample` and `shuffle`.

use sha2::{Digest, Sha512};

const N: usize = 624;
const M: usize = 397;

pub struct PyRandom {
    mt: [u32; N],
    index: usize,
    gauss_next: Option<f64>,
}

impl PyRandom {
    /// `random.Random(n)` for a non-negative integer seed.
    pub fn new(seed: u128) -> Self {
        let mut key = Vec::new();
        let mut n = seed;
        while n > 0 {
            key.push(n as u32);
            n >>= 32;
        }
        if key.is_empty() {
            key.push(0);
        }
        Self::from_key(&key)
    }

    /// `random.Random(s)` for a string seed (seed version 2): the seed integer is
    /// the big-endian reading of the UTF-8 bytes followed by their SHA-512.
    pub fn from_text(seed: &str) -> Self {
        let mut bytes = seed.as_bytes().to_vec();
        bytes.extend_from_slice(&Sha512::digest(seed.as_bytes()));
        // 32-bit words of the integer, least significant first.
        let mut key: Vec<u32> = bytes
            .rchunks(4)
            .map(|c| c.iter().fold(0u32, |acc, b| (acc << 8) | u32::from(*b)))
            .collect();
        while key.len() > 1 && key.last() == Some(&0) {
            key.pop();
        }
        Self::from_key(&key)
    }

    fn from_key(key: &[u32]) -> Self {
        let mut mt = [0u32; N];
        mt[0] = 19_650_218;
        for i in 1..N {
            mt[i] = 1_812_433_253u32
                .wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..N.max(key.len()) {
            mt[i] = (mt[i] ^ (mt[i - 1] ^ (mt[i - 1] >> 30)).wrapping_mul(1_664_525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= N {
                mt[0] = mt[N - 1];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            mt[i] = (mt[i] ^ (mt[i - 1] ^ (mt[i - 1] >> 30)).wrapping_mul(1_566_083_941))
                .wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                mt[0] = mt[N - 1];
                i = 1;
            }
        }
        mt[0] = 0x8000_0000;
        PyRandom {
            mt,
            index: N,
            gauss_next: None,
        }
    }

    fn next_u32(&mut self) -> u32 {
        if self.index >= N {
            for k in 0..N {
                let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % N] & 0x7fff_ffff);
                let mag = if y & 1 == 1 { 0x9908_b0df } else { 0 };
                self.mt[k] = self.mt[(k + M) % N] ^ (y >> 1) ^ mag;
            }
            self.index = 0;
        }
        let mut y = self.mt[self.index];
        self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// `random()`: a float in [0, 1) with 53 random bits.
    pub fn random(&mut self) -> f64 {
        let a = f64::from(self.next_u32() >> 5);
        let b = f64::from(self.next_u32() >> 6);
        (a * 67_108_864.0 + b) * (1.0 / 9_007_199_254_740_992.0)
    }

    fn getrandbits(&mut self, k: u32) -> u64 {
        if k <= 32 {
            return u64::from(self.next_u32() >> (32 - k));
        }
        // Words fill from the least significant end; the last one is truncated.
        let mut out = 0u64;
        let mut shift = 0;
        let mut left = k;
        while left > 0 {
            let mut w = self.next_u32();
            if left < 32 {
                w >>= 32 - left;
            }
            out |= u64::from(w) << shift;
            shift += 32;
            left = left.saturating_sub(32);
        }
        out
    }

    /// `_randbelow(n)`: uniform in [0, n) by rejection on `n.bit_length()` bits.
    fn randbelow(&mut self, n: u64) -> u64 {
        let k = 64 - n.leading_zeros();
        let mut r = self.getrandbits(k);
        while r >= n {
            r = self.getrandbits(k);
        }
        r
    }

    /// `randint(a, b)`, both ends inclusive.
    pub fn randint(&mut self, a: i64, b: i64) -> i64 {
        a + self.randbelow((b - a + 1) as u64) as i64
    }

    pub fn uniform(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.random()
    }

    pub fn gauss(&mut self, mu: f64, sigma: f64) -> f64 {
        let z = match self.gauss_next.take() {
            Some(z) => z,
            None => {
                let x2pi = self.random() * std::f64::consts::TAU;
                let g2rad = (-2.0 * (1.0 - self.random()).ln()).sqrt();
                self.gauss_next = Some(x2pi.sin() * g2rad);
                x2pi.cos() * g2rad
            }
        };
        mu + z * sigma
    }

    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.randbelow(items.len() as u64) as usize]
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.randbelow(i as u64 + 1) as usize;
            items.swap(i, j);
        }
    }

    /// `sample(population, k)`, including CPython's switch between a pool and a
    /// selected-set strategy, so the same elements come back in the same order.
    pub fn sample<T: Clone>(&mut self, population: &[T], k: usize) -> Vec<T> {
        let n = population.len();
        assert!(k <= n, "sample larger than population");
        let mut setsize = 21usize;
        if k > 5 {
            // 4 ** ceil(log(k * 3, 4)), computed the way CPython does, in floats.
            let exp = ((k as f64 * 3.0).ln() / 4f64.ln()).ceil() as u32;
            setsize += 4usize.pow(exp);
        }
        let mut result = Vec::with_capacity(k);
        if n <= setsize {
            let mut pool: Vec<T> = population.to_vec();
            for i in 0..k {
                let j = self.randbelow((n - i) as u64) as usize;
                result.push(pool[j].clone());
                pool[j] = pool[n - i - 1].clone();
            }
        } else {
            let mut selected = std::collections::HashSet::new();
            for _ in 0..k {
                let mut j = self.randbelow(n as u64) as usize;
                while selected.contains(&j) {
                    j = self.randbelow(n as u64) as usize;
                }
                selected.insert(j);
                result.push(population[j].clone());
            }
        }
        result
    }
}
