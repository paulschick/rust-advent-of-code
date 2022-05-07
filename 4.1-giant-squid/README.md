# Day 4

Two types of data in the input file:

- Numbers to be drawn
- Bingo boards

The first line is the numbers to be drawn. After a new line, there is the first bingo board.
The bingo boards are broken up into squares. So each line of a board is on a new line.
To distinguish between boards, there needs to be an empty new line. That goes for the numbers
drawn too.
You could also technically read down 5 lines for each board, then skip the next. Whatever works.
They're all the same size.

## Rules

- A win must be through all numbers selected on a horizontal or vertical line.
There are no diagonals.
- You need to be able to find the first board to win.
- You must calculate the score of the winning board and submit that.
By winning board, that means the first one to win out of the set.

### Score Calculation

Sum all of the **unmarked numbers** on the board. Then, multiply that sum by
the number that was called when the board won.

- Sum all numbers that were not part of the winning set on the board
- Multiply the sum by the winning number.

So their example is that the sum is 188, and the board won when a 24 was drawn,
so the solution is `188 * 24 = 4512`.

