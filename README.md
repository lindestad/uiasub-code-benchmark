# UiA SUB Code Benchmark Challenge

![UiA SUB logo.](assets/sublogo.png)

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

![Greates common divisor picture](assets/gcd.webp)

This challenge involves finding the greatest common divisor (GCD) of two numbers, $a$ and $b$.

$$
a, b \in \mathbb{Z}_{>0}
$$

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

$$
a, b \in \{1, 2, \dots, 20\,000\}
$$

#### Hard Mode

Run:

```sh
cargo run gcd_hard -n 1 --release
```

This version generates 1 million pairs of numbers as input. Additionally the max value is drasticly increased:

$$
a, b \in \{1, 2, \dots, \texttt{u128::MAX}\}
$$

$$
\text{Where } \texttt{u128::MAX} = 340\,282\,366\,920\,938\,463\,463\,374\,607\,431\,768\,211\,455
$$

### Additional Details

- Your executable should be efficient in reading from STDIN until there is no more input and writing the correct output for each pair.
- Make sure your solution handles large inputs both in terms of performance and memory usage.

---

## Challenge 3: Obelisk

![Obelisk view](assets/obelisk2.webp)

This puzzle draws heavy inspiration from an Advent of Code 2024 challenge.

You are taken to an alien world, and must solve a puzzle to be allowed to return home.

You stand atop a plateau, staring down onto a giant field, filled with obelisks. The obelisks have numbers imprinted on them. In front of you is a lever. The puzzle is this:

You are given a starting arrangement of obelisks, like this:
`125 16 12 32`

Each time you pull the lever, the obelisks transform. They follow a strict set of rules.

- If the obelisk has the exact number  `0` it is replaced with `1`: `0 -> 1`.
- If the obelisk has the exact number `7` it *splits* into four obelisks `3 2 3 2`: `7 -> 3 2 3 2`.
- If the obelisk has an *even* number of digits it *splits* in two. The left half of the digits go to the left obelisk, the right half to the right: `22 -> 2 2`. The new obelisks do *not* keep leading zeros: `4000 -> 40 0`, `623002 -> 623 2`.
- If none of these other rules apply, the number on the obelisk is multiplied by $2404$: `151 -> 363004`.

The arrangement of obelisks is always given as if they are on a single straight line.
You will be given a puzzle input in STDIN, that looks something like this:

`5342 17`

After 1 pull the obelisks now look like this:

`53 42 1 7`

After 2 pulls:

`5 3 4 2 2404 3 2 3 2`

After 3 pulls:

`12020 7212 9616 4808 24 4 7212 4808 7212 4808`

After 4 pulls:

`28896080 72 12 96 16 48 8 2 4 9616 72 12 48 8 72 12 48 8`

After 5 pulls:

`2889 6080 7 2 1 2 9 6 1 6 4 8 19232 4808 9616 96 16 7 2 1 2 4 8 19232 7 2 1 2 4 8 19232`

Your goal is to return to STDOUT the *number* of obelisk there are after $n$ pulls of the lever.
In the example above there are `31` obelisks after 5 pulls of the lever.
Expected return to STDOUT:
`31`

### Easy

Given a set of obelisks from STDIN:
How many obelisks are there after $n = 25$ pulls of the lever?
Return the answer to STDOUT. For the example above where the initial input is `5342 17`, the answer is `255042`.

Your executable should be placed in `EXE_FILES_HERE/OBELISK_EASY/`.

Run:

```sh
cargo run obelisk -n 1 --release
```

### Hard

Given a set of obelisks from STDIN:
How many obelisk are there after $n = 100$ pulls of the lever?
Return the answer to STDOUT. For the example above where the initial input is `5342 17`, the answer is `187395285808756316168`.

Your executable should be placed in `EXE_FILES_HERE/OBELISK_HARD/`.

Run:

```sh
cargo run obelisk_hard -n 1 --release
```

---
