# 8-Queens Solver
This project is created solely for a semester project of using search algorithms to allow an A.I. to solve the puzzle.

**Note:** _This is only a minimal working version, and has yet been polished, please use at your own risk._

# Implementation
The search algorithm used in this implementation is depth-first search, an uninfomed search that can be implemented fairly trivially, of only using FIFO method on a growable array structure. As mentioned, the depth-first search algorithm is implemented using a stack vector instead of a recursive function call, to reduce the memory size required to store the necessary information.

This 8-Queens solver uses the complete-state formulation, in which the eight pieces are already placed on the board. The solver will search for an end state that no queens can threathen each other. On each action, the solver will move only one of the pieces according to the queen piece movement.

# Usage
The solver will receive an input state by reading the file from the given file location through CLI, or will fallback to the default ./src/init file if none are specified.

The solver currently only support the simple CSV of the coordinates of the eight squares.

An input example of 8 queens being placed on a horizontal line:
```
a1,b1,c1,d1,e1,f1,h1,g1
```