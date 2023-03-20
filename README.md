# 8-Queens Goal Retrace
This project is created solely for a semester project of using search algorithms to allow an A.I. to solve the puzzle.

**Note:** _This program is currently in a working state, but has yet been polished, please use at your own risk._

## Implementation
The search algorithm used in this implementation is A*, an informed search that can be implemented using a priority queue. The heuristic function of the A* algorithm is calculated with the number of queens that is in an unsafe state (not the same as how many attacks can be made), summed by the number of total moves made, up to that point.

This 8-Queens solver uses the complete-state formulation, in which the eight pieces are already placed on the board. The solver will try to move the queens to match with the end state inputted. On each action, the solver will move only one of the pieces according to the queen piece movement.

## Usage
The solver will receive an initial state, and an end state, by reading the files from the given file locations through CLI, or will fallback to the default ./src/states/init, and ./src/states/goal files if not provided. The first argument in the CLI will be used for the file location of the init state, and the second argument will be used for the file location of the goal state.

An example for the basic usage of the CLI program.
```
$ eight_queens init goal
```
or
```
$ eight_queens
```

The solver currently supports the simple CSV of the coordinates of the eight squares, and the Forsyth–Edwards Notation (FEN) input format only.

A CSV input example of 8 queens being placed horizontally on the bottom-most row:
```
a1,b1,c1,d1,e1,f1,h1,g1
```
A FEN input example of 8 queens being placed horizontally on the bottom-most row:
```
8/8/8/8/8/8/8/QQQQQQQQ
```