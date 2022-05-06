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

