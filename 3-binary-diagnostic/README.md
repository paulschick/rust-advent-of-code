# Day 3 - Binary Submarine Stuff

The data set is a list of binary numbers like this:

```
000011010001
000001110100
111100101010
111001100111
001010100100
```

The first thing is to derive two different numbers. The numbers will be the same length
as each of the provided binary numbers. The first number is the most common number for each row. The
second is the least common number for each row.

This can be thought of as working with matrices. The vertical length would be columns, and horizontally
it would be rows. This works because each binary number has the same number of characters, and it needs to
be evaluated by column.

Gamme Rate: Most common number in each column. 

Epsilon Rate: Least common bit in each column.

After determining these two values, the next step is to convert them into decimal. This means just a normal
number. So converting a string of bits into a number.

When you have a number for each of those two values, then you need to multiply them together.
The product of those numbers is the "power consumption", and the answer to the problem.

## New Things to Try

### Logging

I would like to set up some basic logging. I haven't really explored this in rust yet. What I want to do
is write logs to text file. I would use this instead of a bunch of `println!` statements.

### Console Logging Meaningful Information

This is the design of the final program. It should give you progress updates, and then print out the final
value. That final value can also be written to a text file to be copy and pasted.

Nothing complicated here.

### Error Handling

I've been using just like `unwrap()` and `expect()`. Maybe that's enough? This is something to learn more about.
That might not make it in for day three, though.

## Matrix / Multi-Dimensional Arrays

[Here is a matrix crate](https://docs.rs/matrix/latest/matrix/), that's one option.
An alternative to that would be [array2d](https://docs.rs/array2d/latest/array2d/).
I think the latter is going to be more in line with the appropriate option.

Alternatively, just define an array with multiple dimensions.
I will have to initialize this with a size. I know the size ahead of time, because I have
all of the records in the `vec`.
So that makes it a lot more simple in this case.

Now, for a multiple dimension array to be valuable, I have to split every character in every string.
So one of the nested arrays contains each of the bits in a single binary string.

Width -> equal to the number of characters in one string of bits
Length -> equal to the number of bit strings. the length of the vec created when the file is read.

## Converting a vector of bits into a decimal

This is the last step in terms of things that I have no idea how to do. I need to take the 12
1's and 0's and turn them into a decimal. Then multiply the two numbers and return that value.

