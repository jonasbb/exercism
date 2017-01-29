# Luhn

Write a program that can take a number and determine whether or not it is valid per the Luhn formula.

The [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) is
a simple checksum formula used to validate a variety of identification
numbers, such as credit card numbers and Canadian Social Insurance
Numbers.

The task is to write a function that checks if a given string is valid.

Validating a Number
------
Strings of length 1 or less are not valid. Spaces are allowed in the input,
but they should be stripped before checking. All other non-digit characters
are disallowed.

As an example of a valid string, here is a fictitious Canadian Social Insurance
Number.

```
046 454 286
```

The first step of the Luhn algorithm is to double every second digit,
starting from the right. We will be doubling

```
_4_ 4_4 _8_
```

If doubling the number results in a number greater than 9 then subtract 9
from the product. The results of our doubling:

```
086 858 276
```

Then sum all of the digits

```
0+8+6+8+5+8+2+7+6 = 50
```

If the sum is evenly divisible by 10, then the number is valid. This number is valid!

An example of an invalid Canadian SIN where we've changed the final digit

```
046 454 287
```

Double the second digits, starting from the right

```
086 858 277
```

Sum the digits

```
0+8+6+8+5+8+2+7+7 = 51
```

51 is not evenly divisible by 10, so this number is not valid.

----

An example of an invalid credit card account

```
8273 1232 7352 0569
```

Double the second digits, starting from the right

```
7253 2262 5312 0539
```

Sum the digits

```
7+2+5+3+2+2+5+2+5+3+1+2+0+5+3+9 = 57
```

57 is not evenly divisible by 10, so this number is not valid.

## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Crates and Modules](https://doc.rust-lang.org/stable/book/crates-and-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/xrust](https://github.com/exercism/xrust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/x-common/blob/master/CONTRIBUTING.md).

[help-page]: http://exercism.io/languages/rust
[crates-and-modules]: http://doc.rust-lang.org/stable/book/crates-and-modules.html

## Source

The Luhn Algorithm on Wikipedia [http://en.wikipedia.org/wiki/Luhn_algorithm](http://en.wikipedia.org/wiki/Luhn_algorithm)

## Submitting Incomplete Problems
It's possible to submit an incomplete solution so you can see how others have completed the exercise.

