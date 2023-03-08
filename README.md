# 8-Queens Solver
This project is created solely for a semester project of using search algorithms to allow an A.I. to solve the puzzle.

**Note:** _This is only a minimal working version, and has yet been polished, please use at your own risk._

## Implementation
The search algorithm used in this implementation is depth-first search, an uninfomed search that can be implemented fairly trivially, of only using FIFO method on a growable array structure. As mentioned, the depth-first search algorithm is implemented iteratively using a stack vector instead of a recursive function call, to reduce the memory size required to store the necessary information.

This 8-Queens solver uses the complete-state formulation, in which the eight pieces are already placed on the board. The solver will try to move the queens to match with the end state inputted. On each action, the solver will move only one of the pieces according to the queen piece movement.

# Usage
The solver will receive an initial state, and an end state, by reading the files from the given file locations through CLI, or will fallback to the default ./src/init, and ./src/goal files if not provided. The first argument in the CLI will be used for the file location of the init state, and the second argument will be used for the file location of the goal state.

An example for the basic usage of the CLI program.
```
$ eight_queens ./src/init ./src/goal
```

The solver currently supports the simple CSV of the coordinates of the eight squares, and the Forsyth–Edwards Notation (FEN) input format only.

An CSV input example of 8 queens being placed horizontally on the bottom-most row:
```
a1,b1,c1,d1,e1,f1,h1,g1
```
A FEN input example of 8 queens being placed horizontally on the bottom-most row:
```
8/8/8/8/8/8/8/QQQQQQQQ
```