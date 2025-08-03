use crate::error::AliquotError;
use crate::types::Number;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

/// Possible aliquot sequences defined in an enum.
#[derive(Clone, Debug, PartialEq)]
pub enum AliquotSeq<T: Number> {
    PerfectNumber(T),
    PrimeNumber((T, T)),
    Convergent(Vec<T>),
    AmicableNumber((T, T)),
    SociableNumber(Vec<T>),
    AspiringNumber(Vec<T>),
    IntoCycle(Vec<T>, Vec<T>),
    Unknown(Vec<T>),
}

impl<T: Number> AliquotSeq<T> {
    /// Returns the number, the sequence has been computed for. This is the
    /// first number in the aliquot sequence.
    pub fn number(&self) -> T {
        match self {
            AliquotSeq::PerfectNumber(n) => *n,
            AliquotSeq::PrimeNumber((n, _)) => *n,
            AliquotSeq::Convergent(v) => v[0],
            AliquotSeq::AmicableNumber((n, _)) => *n,
            AliquotSeq::SociableNumber(v) => v[0],
            AliquotSeq::AspiringNumber(v) => v[0],
            AliquotSeq::IntoCycle(v, _) => v[0],
            AliquotSeq::Unknown(v) => v[0],
        }
    }

    /// Returns the length of the aliquot sequence.
    pub fn len(&self) -> usize {
        match self {
            AliquotSeq::PerfectNumber(_) => 1,
            AliquotSeq::PrimeNumber(_) => 2,
            AliquotSeq::Convergent(v) => v.len(),
            AliquotSeq::AmicableNumber(_) => 2,
            AliquotSeq::SociableNumber(v) => v.len(),
            AliquotSeq::AspiringNumber(v) => v.len(),
            AliquotSeq::IntoCycle(v0, v1) => v0.len() + v1.len(),
            AliquotSeq::Unknown(v) => v.len(),
        }
    }

    /// Returns the type of the aliquot sequence as a string.
    pub fn type_str(&self) -> &str {
        match self {
            AliquotSeq::PerfectNumber(_) => "Perfect number",
            AliquotSeq::PrimeNumber(_) => "Prime number",
            AliquotSeq::Convergent(_) => "Convergent sequence",
            AliquotSeq::AmicableNumber(_) => "Amicable number",
            AliquotSeq::SociableNumber(_) => "Sociable number",
            AliquotSeq::AspiringNumber(_) => "Aspiring number",
            AliquotSeq::IntoCycle(_, _) => "Convergent into cycle",
            AliquotSeq::Unknown(_) => "Unknown sequence",
        }
    }

    /// Returns the plain sequence as a Vec of T.
    pub fn seq(&self) -> Vec<T> {
        match self {
            AliquotSeq::PerfectNumber(n) => vec![*n],
            AliquotSeq::PrimeNumber((n, one)) => vec![*n, *one],
            AliquotSeq::Convergent(v) => v.clone(),
            AliquotSeq::AmicableNumber((n, m)) => vec![*n, *m],
            AliquotSeq::SociableNumber(v) => v.clone(),
            AliquotSeq::AspiringNumber(v) => v.clone(),
            AliquotSeq::IntoCycle(v0, v1) => {
                let mut ret = v0.clone();
                ret.append(&mut v1.clone());
                ret
            }
            AliquotSeq::Unknown(v) => v.clone(),
        }
    }

    /// Returns the sequence as a string.
    pub fn seq_string(&self) -> String {
        let vec_to_string = |v: &Vec<T>| -> String {
            let mut ret = format!("[{}", v[0]);
            for val in v.iter().skip(1) {
                ret += format!(", {val}").as_str();
            }
            ret += "]";
            ret
        };
        match self {
            AliquotSeq::PerfectNumber(n) => {
                format!("{n}")
            }
            AliquotSeq::PrimeNumber((n, one)) => {
                format!("{n}, {one}")
            }
            AliquotSeq::Convergent(v) => vec_to_string(v),
            AliquotSeq::AmicableNumber((n, m)) => {
                format!("{n}, {m}")
            }
            AliquotSeq::SociableNumber(v) => vec_to_string(v),
            AliquotSeq::AspiringNumber(v) => vec_to_string(v),
            AliquotSeq::IntoCycle(v0, v1) => {
                let mut ret = vec_to_string(v0);
                ret += " -> ";
                ret += &vec_to_string(v1);
                ret
            }
            AliquotSeq::Unknown(v) => vec_to_string(v),
        }
    }

    /// Returns true, if the aliquot sequence cycles.
    pub fn cycles(&self) -> bool {
        match self {
            AliquotSeq::AmicableNumber(_) => true,
            AliquotSeq::SociableNumber(_) => true,
            AliquotSeq::IntoCycle(_, _) => true,
            _ => false,
        }
    }
}

/// Stores computed aliquot sequences in a map.
pub struct Cache<T: Number> {
    max_cache_size: usize,
    cache_count: usize,
    cache: HashMap<T, AliquotSeq<T>>,
    cache_lut: HashMap<T, T>,
}

impl<T: Number> Cache<T> {
    /// Returns a new cache for aliquot sequences.
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            max_cache_size,
            cache_count: 0,
            cache: HashMap::<T, AliquotSeq<T>>::new(),
            cache_lut: HashMap::<T, T>::new(),
        }
    }

    /// Adds the whole sequence to the LUT, except the first number.
    fn add_seq_lut(&mut self, n: T, seq: &[T]) {
        for &s in seq.iter().skip(1) {
            if s > T::ONE {
                self.cache_lut.insert(s, n);
            }
        }
        self.cache_count += seq.len() - 1;
    }

    /// Adds the aliquot sequence to the cache, if it isn't present yet.
    pub fn add(&mut self, aliquot_seq: AliquotSeq<T>) {
        let len = aliquot_seq.len();
        let n = aliquot_seq.number();
        // Check if sequence fits into cache
        if len < (self.max_cache_size - self.cache_count) {
            // Check if number n exists in cache already
            if !self.cache.contains_key(&n) {
                match aliquot_seq {
                    AliquotSeq::Convergent(ref seq) => {
                        self.add_seq_lut(n, seq);
                    }
                    AliquotSeq::SociableNumber(ref seq) => {
                        self.add_seq_lut(n, seq);
                    }
                    AliquotSeq::AspiringNumber(ref seq) => {
                        self.add_seq_lut(n, seq);
                    }
                    AliquotSeq::AmicableNumber((_, p)) => {
                        // Add the amicable number in reverse order
                        // We don't need the LUT in this case
                        self.cache.insert(p, AliquotSeq::AmicableNumber((p, n)));
                    }
                    AliquotSeq::IntoCycle(ref seq, _) => {
                        self.add_seq_lut(n, seq);
                    }
                    AliquotSeq::Unknown(ref seq) => {
                        self.add_seq_lut(n, seq);
                    }
                    _ => {}
                }
                self.cache.insert(n, aliquot_seq);
                self.cache_count += len;
            }
        }
    }

    /// Adds the aliquot sequence to the cache, if it isn't present yet and
    /// returns the original aliquot sequence. This way we avoid cloning the
    /// sequence of a sociable number.
    pub fn add_and_return(&mut self, aliquot_seq: AliquotSeq<T>) -> AliquotSeq<T> {
        self.add(aliquot_seq.clone());
        aliquot_seq
    }

    /// Clears all entries in the cache without deallocating memory.
    pub fn clear(&mut self) {
        self.cache_count = 0;
        self.cache.clear();
    }

    /// Returns the number of sequences stored in the cache.
    pub fn n_seq(&self) -> usize {
        self.cache.len()
    }

    /// Return the sum of all numbers of sequences contained in the cache.
    pub fn count(&self) -> usize {
        self.cache_count
    }

    /// Returns the aliquot sequence for n or None, if there is no entry in the cache.
    pub fn get(&self, n: T) -> Option<AliquotSeq<T>> {
        let find_pos_n = move |seq: &Vec<T>| -> Option<usize> {
            seq.iter()
                .enumerate()
                .find(|(_, x)| **x == n)
                .map(|(p, _)| p)
        };
        if let Some(aliquot_seq) = self.cache.get(&n) {
            return Some(aliquot_seq.clone());
        } else if let Some(p) = self.cache_lut.get(&n) {
            // Reconstruct the sequence
            match self.cache.get(p) {
                Some(AliquotSeq::Convergent(seq)) => {
                    if let Some(pos) = find_pos_n(seq) {
                        if pos < (seq.len() - 1) {
                            let seq_new = seq[pos..].to_vec();
                            return Some(AliquotSeq::Convergent(seq_new));
                        }
                    }
                }
                Some(AliquotSeq::AspiringNumber(seq)) => {
                    if let Some(pos) = find_pos_n(seq) {
                        if pos < (seq.len() - 1) {
                            let seq_new = seq[pos..].to_vec();
                            return Some(AliquotSeq::AspiringNumber(seq_new));
                        }
                    }
                }
                Some(AliquotSeq::SociableNumber(seq)) => {
                    if let Some(pos) = find_pos_n(seq) {
                        let mut seq_new = seq[pos..].to_vec();
                        seq_new.extend_from_slice(&seq[0..pos]);
                        return Some(AliquotSeq::SociableNumber(seq_new));
                    }
                }
                Some(AliquotSeq::IntoCycle(seq, cycle)) => {
                    if let Some(pos) = find_pos_n(seq) {
                        let seq_new = seq[pos..].to_vec();
                        return Some(AliquotSeq::IntoCycle(seq_new, cycle.clone()));
                    }
                }
                Some(AliquotSeq::Unknown(seq)) => {
                    if let Some(pos) = find_pos_n(seq) {
                        if pos < (seq.len() - 1) {
                            let seq_new = seq[pos..].to_vec();
                            return Some(AliquotSeq::Unknown(seq_new));
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}

/// Generator for aliquot sequences.
pub struct Generator<T: Number> {
    max_num: T,
    max_len_seq: usize,
    cache: Cache<T>,
    debug: bool,
}

impl<T: Number> Default for Generator<T>
where
    Range<T>: Iterator<Item = T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Number> Generator<T>
where
    Range<T>: Iterator<Item = T>,
{
    /// Returns a new generator object for aliquot sequences with defaullt values.
    pub fn new() -> Self {
        Self {
            max_num: T::MAX,
            max_len_seq: 1_000_000,
            cache: Cache::new(1_000_000),
            debug: false,
        }
    }

    /// Returns a new generator object for aliquot sequences with specified parameters.
    pub fn with_params(max_num: T, max_len_seq: usize, max_cache_size: usize, debug: bool) -> Self {
        let cache = Cache::new(max_cache_size);
        Self {
            max_num,
            max_len_seq,
            cache,
            debug,
        }
    }

    /// Prints string, if debug is enabled.
    fn print_debug(&self, line: String) {
        if self.debug {
            println!("Debug: {line}");
        }
    }

    /// Sums up all proper divisors of a number n (except n itself).
    pub fn aliquot_sum(n: T) -> Result<T, AliquotError> {
        // The aliquot sum is always zero for one and undefined for zero
        if n <= T::ONE {
            return Ok(T::ZERO);
        }
        let mut sum = T::ONE;
        let start = T::TWO;
        // Run until square root of n using Newton's method
        let isqrt = |k: T| {
            if k <= T::ONE {
                return k;
            }
            let mut x0 = k / T::TWO;
            let mut x1 = (x0 + k / x0) / T::TWO;
            while x1 < x0 {
                x0 = x1;
                x1 = (x0 + k / x0) / T::TWO;
            }
            x0
        };
        let end = isqrt(n) + T::ONE;
        for i in start..end {
            let div = n / i;
            let chk = i * div;
            if chk == n {
                let add = if i != div {
                    // Both i and div are divisors of n
                    i + div
                } else {
                    // Count the divisor only once if i equals div
                    i
                };
                if add > (T::MAX - sum) {
                    let err_msg = format!("{} plus {} exceeds maximum {}", sum, add, T::MAX);
                    return Err(AliquotError::OverflowError(err_msg));
                }
                sum += add;
            }
        }
        Ok(sum)
    }

    /// Computes the aliquot sequence of a number n.
    pub fn aliquot_seq(&mut self, n: T) -> AliquotSeq<T> {
        // Store all values in a hash map for detecting cycles faster
        let mut lut_seq = HashSet::<T>::new();
        // The original number is the first number in the sequence
        let mut seq = vec![n];
        // Aliquot sequence is undefined for 0
        if n == T::ZERO || n == T::ONE {
            return AliquotSeq::Unknown(seq);
        }
        // Check if the aliquot sequence has been computed for this number already
        if let Some(aliquot_seq_cache) = self.cache.get(n) {
            self.print_debug(format!("Found sequence for {n} in the cache"));
            return aliquot_seq_cache;
        }
        for _i in 1..self.max_len_seq {
            let len_seq = seq.len();
            let last = seq[len_seq - 1];
            match Self::aliquot_sum(last) {
                Ok(next) => {
                    // Abort, if a number in the sequence exceeds the maximum value allowed
                    if next >= self.max_num {
                        self.print_debug(format!("Numbers in the sequence for {n} exceed maximum"));
                        return self.cache.add_and_return(AliquotSeq::Unknown(seq));
                    }
                    // First check if the sum is stored in the cache, so we don't need
                    // to compute the rest of the sequence
                    if let Some(aliquot_seq_cache) = self.cache.get(next) {
                        self.print_debug(format!("Found sequence for {next} in the cache to complete the sequence for {n}"));
                        match aliquot_seq_cache {
                            AliquotSeq::PerfectNumber(p) => {
                                seq.push(p);
                                return self.cache.add_and_return(AliquotSeq::AspiringNumber(seq));
                            }
                            AliquotSeq::PrimeNumber((p, one)) => {
                                seq.push(p);
                                seq.push(one);
                                return self.cache.add_and_return(AliquotSeq::Convergent(seq));
                            }
                            AliquotSeq::Convergent(v) => {
                                seq.extend_from_slice(v.as_slice());
                                return self.cache.add_and_return(AliquotSeq::Convergent(seq));
                            }
                            AliquotSeq::AmicableNumber((a0, a1)) => {
                                // Check if this is just the reverse order
                                if a0 == next && a1 == n {
                                    return AliquotSeq::AmicableNumber((n, next));
                                } else {
                                    // Otherwise n runs into cycle of amicable numbers
                                    return self
                                        .cache
                                        .add_and_return(AliquotSeq::IntoCycle(seq, vec![a0, a1]));
                                }
                            }
                            AliquotSeq::SociableNumber(v) => {
                                // Runs into a cycle of sociable numbers
                                return self
                                    .cache
                                    .add_and_return(AliquotSeq::IntoCycle(seq, v.clone()));
                            }
                            AliquotSeq::AspiringNumber(v) => {
                                seq.extend_from_slice(v.as_slice());
                                return self.cache.add_and_return(AliquotSeq::AspiringNumber(seq));
                            }
                            AliquotSeq::IntoCycle(v0, v1) => {
                                seq.extend_from_slice(v0.as_slice());
                                return self
                                    .cache
                                    .add_and_return(AliquotSeq::IntoCycle(seq, v1.clone()));
                            }
                            AliquotSeq::Unknown(v) => {
                                // We ran into an unknown sequence
                                seq.extend_from_slice(v.as_slice());
                                return self.cache.add_and_return(AliquotSeq::Unknown(seq));
                            }
                        }
                    } else if next == T::ONE {
                        self.print_debug(format!("Sequence for {n} converged to one"));
                        match len_seq {
                            1 => {
                                // If only n is contained in the sequence so far, we have a prime
                                return self
                                    .cache
                                    .add_and_return(AliquotSeq::PrimeNumber((n, T::ONE)));
                            }
                            _ => {
                                // This is a normal sequence ending with a prime followed by one
                                seq.push(T::ONE);
                                return self.cache.add_and_return(AliquotSeq::Convergent(seq));
                            }
                        }
                    } else if next == n {
                        self.print_debug(format!("Sequence for {n} converged to {n}"));
                        match len_seq {
                            1 => {
                                // There is only the original number in the sequence
                                // so this must be a perfect number
                                return self.cache.add_and_return(AliquotSeq::PerfectNumber(n));
                            }
                            2 => {
                                // This is a repeating sequence with two numbers
                                return self
                                    .cache
                                    .add_and_return(AliquotSeq::AmicableNumber((n, last)));
                            }
                            _ => {
                                // This is a repeating sequence with more than two numbers
                                return self.cache.add_and_return(AliquotSeq::SociableNumber(seq));
                            }
                        }
                    } else if next == last {
                        self.print_debug(format!(
                            "Sequence for {n} converged into the perfect number {last}"
                        ));
                        // This sequence ended with a perfect number, so we have an aspiring number
                        return self.cache.add_and_return(AliquotSeq::AspiringNumber(seq));
                    } else if lut_seq.contains(&next) {
                        self.print_debug(format!(
                            "Sequence for {n} converged into a cycle of {next}"
                        ));
                        // Find the position in the sequence and split there
                        // We now have a sequence, which converges into a cycle
                        // since the original number n is not contained in the LUT
                        let pos = seq
                            .iter()
                            .enumerate()
                            .find(|&(_, &x)| x == next)
                            .map(|(p, _)| p)
                            .unwrap_or(0);
                        let cycle = seq.split_off(pos);
                        return self.cache.add_and_return(AliquotSeq::IntoCycle(seq, cycle));
                    }
                    seq.push(next);
                    lut_seq.insert(next);
                }
                Err(err_msg) => {
                    self.print_debug(format!(
                        "Sequence of {n} unknown, because an error occurred"
                    ));
                    println!("Error: {err_msg}");
                    return self.cache.add_and_return(AliquotSeq::Unknown(seq));
                }
            }
        }
        self.cache.add_and_return(AliquotSeq::Unknown(seq))
    }

    /// Returns the associated cache object.
    pub fn cache(&self) -> &Cache<T> {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_gen<T: Number>(gener: &mut Generator<T>, n: T, exp: AliquotSeq<T>)
    where
        Range<T>: Iterator<Item = T>,
    {
        assert_eq!(gener.aliquot_seq(n), exp);
    }

    #[test]
    fn test_aliquot_seq_u16() {
        let mut gener = Generator::<u16>::new();
        test_gen(&mut gener, 2, AliquotSeq::PrimeNumber((2, 1)));
        test_gen(&mut gener, 3, AliquotSeq::PrimeNumber((3, 1)));
        test_gen(&mut gener, 5, AliquotSeq::PrimeNumber((5, 1)));
        test_gen(&mut gener, 6, AliquotSeq::PerfectNumber(6));
        test_gen(&mut gener, 7, AliquotSeq::PrimeNumber((7, 1)));
        test_gen(&mut gener, 11, AliquotSeq::PrimeNumber((11, 1)));
        test_gen(
            &mut gener,
            12,
            AliquotSeq::Convergent(vec![12, 16, 15, 9, 4, 3, 1]),
        );
        test_gen(&mut gener, 13, AliquotSeq::PrimeNumber((13, 1)));
        test_gen(&mut gener, 17, AliquotSeq::PrimeNumber((17, 1)));
        test_gen(&mut gener, 19, AliquotSeq::PrimeNumber((19, 1)));
        test_gen(&mut gener, 28, AliquotSeq::PerfectNumber(28));
        test_gen(&mut gener, 29, AliquotSeq::PrimeNumber((29, 1)));
        test_gen(
            &mut gener,
            30,
            AliquotSeq::Convergent(vec![
                30, 42, 54, 66, 78, 90, 144, 259, 45, 33, 15, 9, 4, 3, 1,
            ]),
        );
        test_gen(&mut gener, 31, AliquotSeq::PrimeNumber((31, 1)));
        test_gen(&mut gener, 41, AliquotSeq::PrimeNumber((41, 1)));
        test_gen(
            &mut gener,
            42,
            AliquotSeq::Convergent(vec![42, 54, 66, 78, 90, 144, 259, 45, 33, 15, 9, 4, 3, 1]),
        );
        test_gen(&mut gener, 43, AliquotSeq::PrimeNumber((43, 1)));
        test_gen(
            &mut gener,
            54,
            AliquotSeq::Convergent(vec![54, 66, 78, 90, 144, 259, 45, 33, 15, 9, 4, 3, 1]),
        );
        test_gen(
            &mut gener,
            60,
            AliquotSeq::Convergent(vec![60, 108, 172, 136, 134, 70, 74, 40, 50, 43, 1]),
        );
        test_gen(
            &mut gener,
            78,
            AliquotSeq::Convergent(vec![78, 90, 144, 259, 45, 33, 15, 9, 4, 3, 1]),
        );
        test_gen(
            &mut gener,
            90,
            AliquotSeq::Convergent(vec![90, 144, 259, 45, 33, 15, 9, 4, 3, 1]),
        );
        test_gen(&mut gener, 95, AliquotSeq::AspiringNumber(vec![95, 25, 6]));
        test_gen(
            &mut gener,
            96,
            AliquotSeq::Convergent(vec![96, 156, 236, 184, 176, 196, 203, 37, 1]),
        );
        test_gen(&mut gener, 220, AliquotSeq::AmicableNumber((220, 284)));
        test_gen(&mut gener, 284, AliquotSeq::AmicableNumber((284, 220)));
    }

    #[test]
    fn test_aliquot_seq_u32() {
        let mut gener = Generator::<u32>::new();
        test_gen(&mut gener, 2, AliquotSeq::PrimeNumber((2, 1)));
        test_gen(&mut gener, 3, AliquotSeq::PrimeNumber((3, 1)));
        test_gen(&mut gener, 6, AliquotSeq::PerfectNumber(6));
        test_gen(&mut gener, 17, AliquotSeq::PrimeNumber((17, 1)));
        test_gen(&mut gener, 19, AliquotSeq::PrimeNumber((19, 1)));
        test_gen(&mut gener, 41, AliquotSeq::PrimeNumber((41, 1)));
        test_gen(&mut gener, 43, AliquotSeq::PrimeNumber((43, 1)));
        test_gen(&mut gener, 95, AliquotSeq::AspiringNumber(vec![95, 25, 6]));
        test_gen(&mut gener, 220, AliquotSeq::AmicableNumber((220, 284)));
        test_gen(&mut gener, 284, AliquotSeq::AmicableNumber((284, 220)));
        test_gen(
            &mut gener,
            1264460,
            AliquotSeq::SociableNumber(vec![1264460, 1547860, 1727636, 1305184]),
        );
        test_gen(
            &mut gener,
            276,
            AliquotSeq::Unknown(vec![
                276,
                396,
                696,
                1104,
                1872,
                3770,
                3790,
                3050,
                2716,
                2772,
                5964,
                10164,
                19628,
                19684,
                22876,
                26404,
                30044,
                33796,
                38780,
                54628,
                54684,
                111300,
                263676,
                465668,
                465724,
                465780,
                1026060,
                2325540,
                5335260,
                11738916,
                23117724,
                45956820,
                121129260,
                266485716,
                558454764,
                1092873236,
                1470806764,
                1471882804,
                1642613196,
                2737688884,
                2740114636,
                2791337780,
            ]),
        );
    }

    #[test]
    fn test_aliquot_seq_u64() {
        let mut gener = Generator::<u64>::new();
        test_gen(&mut gener, 2, AliquotSeq::PrimeNumber((2, 1)));
        test_gen(&mut gener, 3, AliquotSeq::PrimeNumber((3, 1)));
        test_gen(&mut gener, 6, AliquotSeq::PerfectNumber(6));
        test_gen(&mut gener, 17, AliquotSeq::PrimeNumber((17, 1)));
        test_gen(&mut gener, 19, AliquotSeq::PrimeNumber((19, 1)));
        test_gen(&mut gener, 41, AliquotSeq::PrimeNumber((41, 1)));
        test_gen(&mut gener, 43, AliquotSeq::PrimeNumber((43, 1)));
        test_gen(&mut gener, 95, AliquotSeq::AspiringNumber(vec![95, 25, 6]));
        test_gen(&mut gener, 220, AliquotSeq::AmicableNumber((220, 284)));
        test_gen(&mut gener, 284, AliquotSeq::AmicableNumber((284, 220)));
        test_gen(
            &mut gener,
            1264460,
            AliquotSeq::SociableNumber(vec![1264460, 1547860, 1727636, 1305184]),
        );
        test_gen(
            &mut gener,
            138,
            AliquotSeq::Convergent(vec![
                138,
                150,
                222,
                234,
                312,
                528,
                960,
                2088,
                3762,
                5598,
                6570,
                10746,
                13254,
                13830,
                19434,
                20886,
                21606,
                25098,
                26742,
                26754,
                40446,
                63234,
                77406,
                110754,
                171486,
                253458,
                295740,
                647748,
                1077612,
                1467588,
                1956812,
                2109796,
                1889486,
                953914,
                668966,
                353578,
                176792,
                254128,
                308832,
                502104,
                753216,
                1240176,
                2422288,
                2697920,
                3727264,
                3655076,
                2760844,
                2100740,
                2310856,
                2455544,
                3212776,
                3751064,
                3282196,
                2723020,
                3035684,
                2299240,
                2988440,
                5297320,
                8325080,
                11222920,
                15359480,
                19199440,
                28875608,
                25266172,
                19406148,
                26552604,
                40541052,
                54202884,
                72270540,
                147793668,
                228408732,
                348957876,
                508132204,
                404465636,
                303708376,
                290504024,
                312058216,
                294959384,
                290622016,
                286081174,
                151737434,
                75868720,
                108199856,
                101437396,
                76247552,
                76099654,
                42387146,
                21679318,
                12752594,
                7278382,
                3660794,
                1855066,
                927536,
                932464,
                1013592,
                1546008,
                2425752,
                5084088,
                8436192,
                13709064,
                20563656,
                33082104,
                57142536,
                99483384,
                245978376,
                487384824,
                745600776,
                1118401224,
                1677601896,
                2538372504,
                4119772776,
                8030724504,
                14097017496,
                21148436904,
                40381357656,
                60572036544,
                100039354704,
                179931895322,
                94685963278,
                51399021218,
                28358080762,
                18046051430,
                17396081338,
                8698040672,
                8426226964,
                6319670230,
                5422685354,
                3217383766,
                1739126474,
                996366646,
                636221402,
                318217798,
                195756362,
                101900794,
                54202694,
                49799866,
                24930374,
                17971642,
                11130830,
                8904682,
                4913018,
                3126502,
                1574810,
                1473382,
                736694,
                541162,
                312470,
                249994,
                127286,
                69898,
                34952,
                34708,
                26038,
                13994,
                7000,
                11720,
                14740,
                19532,
                16588,
                18692,
                14026,
                7016,
                6154,
                3674,
                2374,
                1190,
                1402,
                704,
                820,
                944,
                916,
                694,
                350,
                394,
                200,
                265,
                59,
                1,
            ]),
        );
    }

    #[test]
    fn test_aliquot_seq_u128() {
        let mut gener = Generator::<u128>::new();
        test_gen(&mut gener, 2, AliquotSeq::PrimeNumber((2, 1)));
        test_gen(&mut gener, 3, AliquotSeq::PrimeNumber((3, 1)));
        test_gen(&mut gener, 6, AliquotSeq::PerfectNumber(6));
        test_gen(&mut gener, 17, AliquotSeq::PrimeNumber((17, 1)));
        test_gen(&mut gener, 19, AliquotSeq::PrimeNumber((19, 1)));
        test_gen(&mut gener, 41, AliquotSeq::PrimeNumber((41, 1)));
        test_gen(&mut gener, 43, AliquotSeq::PrimeNumber((43, 1)));
        test_gen(&mut gener, 95, AliquotSeq::AspiringNumber(vec![95, 25, 6]));
        test_gen(&mut gener, 220, AliquotSeq::AmicableNumber((220, 284)));
        test_gen(&mut gener, 284, AliquotSeq::AmicableNumber((284, 220)));
        test_gen(
            &mut gener,
            1264460,
            AliquotSeq::SociableNumber(vec![1264460, 1547860, 1727636, 1305184]),
        );
    }
}
