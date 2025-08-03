# aliquot
Implementation of a generator of aliquot sequences in Rust

## General information about aliquot sequences
Every aliquot sequence starts with a natural number, for which the sum of its proper divisors is computed.
This aliquot sum is the next number in the sequence. Most sequences converge and end with a prime number followed by a one. But this is not always the case and there are some other types of aliquot sequences:

- Perfect numbers: The sum of its proper divisors is the number itself. Example: 6.
- Aspiring numbers: The sequence ends with a perfect number. Example: 95.
- Amicable numbers: The sequence consists of two numbers and cycles between them. Example: 220 and 284.
- Sociable numbers: A sequence which cycles and consists of more than two numbers. Example: 1264460, 1547860, 1727636, 1305184, 1264460...
- Unknown sequences: Noone has found an end of the sequence. Example: 276.

For more information have a look at the Wikipedia page on aliquot sequences.

## Generating aliquot sequences in Rust
### Using CLI
Using this CLI tool one can compute the aliquot sequences for multiple numbers and output them to stdout.
You can pass a list of comma-separated numbers or ranges or a mix of both.
I tried to optimize this project as good as I could. The generator uses a cache and can determine, if a number is present in an already computed sequence. The sequence can be completed this way without further computation.
Additionally multiple threads may be used to generate the sequences.

Example: Generate the aliquot sequences for the first 100 numbers:

```bash
cargo r --release -- 1-100
```

A more complex example: Generate the aliquot sequences for the first 300 numbers, but leave out 276, since it contains large numbers and results in an overflow error:

```bash
cargo r --release -- 1-275,277-300
```

You can avoid the long computation of such numbers like 276 by defining a maximum value for a number in the resulting sequence. If a number exceeds this value, the sequence is declared as unknown.
For example numbers in the sequence should not be greater than four billion:

```bash
cargo r --release -- -m 4000000000 1-300
```

The size of the cache can be set using the CLI switch "-c SIZE". The cache is turned off completely with "-c 0".
Otherwise a default value of 1000000 numbers is used, which allocates 8 Mb of memory.

### Using functionality inside Rust code as a lib
You can generate aliquot sequences in your Rust source using this crate as a lib.
Just use *cargo add* to add the dependency to your project.
The generator is implemented generically and the type of the numbers in a sequence can be u16, u32, u64 or u128.
To determine an aliquot sequence, you just need a few lines of code:

```rust
let mut gener = Generator::<u64>::new();
let aliquot_seq = gener.aliquot_seq(42);
```

The function *aliquot_seq* returns an enum of the type *AliquotSeq*, which differentiates between the following variants:

- *AliquotSeq::PerfectNumber*: The sequence consists of a single perfect number
- *AliquotSeq::PrimeNumber*: The sequence is just a prime number followed by one
- *AliquotSeq::Convergent*: The sequence converges into a prime number followed by one
- *AliquotSeq::AmicableNumber*: The sequence only contains two numbers cycling
- *AliquotSeq::SociableNumber*: A cycling sequence with at least three different numbers
- *AliquotSeq::AspiringNumber*: A Sequence ending with a perfect number
- *AliquotSeq::IntoCycle*: A sequence ending with a cycling sequence like an amicable number (Not found yet - is this actually possible?)
- *AliquotSeq::Unknown*: For this sequence no end has been found due to overflow errors or aborting penalties

You can easily print the sequence and its type using the functions *sequence_string* and *type_str* from the returned enum:

```rust
let n = aliquot_seq.number();
let type_str = aliquot_seq.type_str();
let seq_string = aliquot_seq.seq_string();
println!("{n}: {type_str} {seq_string}");
```
