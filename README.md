# UiA SUB Code Benchmark Challenge

This repository features some coding challenges:

## Challenge 1: Reverse a String

So simple yet so hard. Create an executable (file that ends in `.exe` on Windows) and place it in `EXE_FILES_HERE/REVERSE_STRING/`.

Your executable will be passed a large list of words (through STDIN), separated by a space, like this:

```text
horse cart truck donut conch
```

You need to reverse each word and pass back a single large string to STDOUT:

```text
esroh trac kcurt tunod hcnoc
```

There are two versions of the challenge:

### Easy Mode

Run (replace `20` with the number of runs you want; `--release` can be omitted if you want a non-optimized build):

```sh
cargo run reverse -n 20 --release
```

This version gives a string of 20,000 words (20K words) as input.

### Hard(er) Mode

Run:

```sh
cargo run reverse_large -n 1 --release
```

This version gives a string of 20 million words (that's ~200MB of raw text). Good luck!

---

## Challenge 2: Greatest Common Divisor

This challenge involves finding the greatest common divisor (GCD) of two numbers, $a$ and $b$.

\[
a, b \in \mathbb{Z}_{>0}
\]

($a$ and $b$ are positive integers).

Your executable should be placed in `EXE_FILES_HERE/GREATEST_COMMON_DIVISOR/`.

### Input Format

Your program will be provided with input from STDIN in the form of multiple lines. Each line contains two positive integers separated by a space. For example:

```text
12 20
14 3
442 86
```

### Output Format

For each input line, compute the greatest common divisor (GCD) of the two numbers and output a single line containing the result. For the sample input above, the output should be:

```text
4
1
2
```

### Running the Challenge

There are two versions of the challenge:

#### Easy Mode

Run (replace `20` with the number of runs you want):

```sh
cargo run gcd -n 20 --release
```

This version generates 20,000 pairs of numbers as input.

Constraints:

\[
a, b \in \{1, 2, \dots, 20\,000\}
\]

#### Hard Mode

Run:

```sh
cargo run gcd_hard -n 1 --release
```

This version generates 1 million pairs of numbers as input. Additionally the max value is drasticly increased:

\[
a, b \in \{1, 2, \dots, \texttt{u128::MAX}\}
\]

\[
\text{Where } \texttt{u128::MAX} = 340\,282\,366\,920\,938\,463\,463\,374\,607\,431\,768\,211\,455
\]

### Additional Details

- Your executable should be efficient in reading from STDIN until there is no more input and writing the correct output for each pair.
- Make sure your solution handles large inputs both in terms of performance and memory usage.

---

Good luck, and have fun with the challenges!
