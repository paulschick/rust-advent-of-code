# Day 3.2 - Binary Diagnostic Part 2

[Link to challenge](https://adventofcode.com/2021/day/3#part2)

## Overview

Two more values introduced:

- oxygen generator rating
- CO2 scrubber rating

Finding these two values involves filtering out values until only one remains.
Start with the full list of binary numbers.

### Oxygen Generator Rating

This one involves using the most common numbers in a pattern.
You have the list of binary numbers. Each "row" is going to be considered
one binary number. The goal is to get to a single binary number that
fits the criteria, then convert this into a decimal.

So, first, you want to look at the first position. This is the first bit
in every row. Same as was done to get the gamma number. Say that first bit
is a 1. Now, you discard every row that does not start with a one; all remaining
numbers have a 1 in the first position.

Now find the most common number in the second position. Let's say that it's 0.
So discard every row where the first two numbers are not 10. Then continue doing
this for every position until you get a single unique number. Convert to decimal,
and you have the answer for the oxygen generator rating.

### CO2 Scrubber Rating

This is the same but using the *least* common number.

### Considerations

When there is an equal number of 0 and 1 bits, and you're solving for oxygen generator,
then you would use a 1 as the value for that position.
When there is an equal number and you're solving for the CO2 scrubber rating, then you're
going to use a 0 for that position.

I should be able to just flip the oxygen generator rating to get the CO2 scrubber
rating.

### Submitting the Solution

After getting the two numbers, you multiply them together like usual. Submit the
product of those two values.

