# Tetris
## Changelog

2026-11-04:
- Initial release
- Add resources flag to writeup

## Introduction
For project 5 and project 6 you will be implementing a networked Tetris game.
If you've never played Tetris against other people try it out! It's a fun game
and regardless, it will be useful context for this project. Two popular
websites for this are [jstris](https://jstris.jezevec10.com/) and
[tetr.io](https://tetr.io/).

At a high level, multiplayer Tetris involves players, each with their own board
attempting to place tetrominoes in a 10 by 20 grid such that the pieces tile
the grid with no gaps. When a row is tiled, that row is removed and blocks
above are moved downwards to replace it.

It's possible for a single block to clear multiple lines. When this happens,
"garbage" is created and sent to the bottom of the other player's board. This
garbage is a row of filled grid cells with one omitted.

The player who last the longest before being forced a to place a block in the
grid higher than row 20 wins.

Project 5 will involve four parts:
1. implementing single player Tetris
2. implementing a bot to play Tetris. 
3. implementing multiplayer Tetris
4. writing a design document

For this project, we want you to implement something multithreaded for you bot
in (2) and have clients on separate threads communicating with a "server"
thread for part (3). We will expand in the corresponding sections.

The rest you may implement as you want, though we will load test your designs
and the most robust teams may receive a small amount of extra credit on this or
next assignment. So, try to write efficient code whenever possible.

### Submission and Logistics
You should *work in groups of 3-4*. Please form groups on gradescope and submit
together for both the checkpoint and final submission. You should have the same
groups throughout this and next project as they will build on each other.

You will submit your project using `cargo package` as previously. Your project
when built should create a binary. We will specify the behavior of that binary
below.

You must also submit a design document describing your design, implementation,
and any optimizations you implement. You should treat this as a proposal as
much as a writeup, and it may make sense to write this before starting your
code (though you will probably want to edit it when you finish). We will
specify what you need to put in this below as well.

(1) will be due Nov 11 as a check-in.
(2), (3), and (4) will be due Nov 18 and will possibly contain edits to your work
submitted for (1).

### AI Policy
As a reminder, you must not use LLMs to generate any part of your submission,
including code, comments, or writing in your design doc. Asking an LLM how to
implement something and using that as inspiration violates this policy.

We discourage, but begrudgingly allow the use of LLMs for seeking out
documentation or descriptions of high level concepts. Though, please seek 
official docs or reach out to a TA through OH or Ed first!

## Part 1: Single Player Tetris
You will implement a binary which simulates a game of Tetris given a list of
inputs describing what a player's action on every frame of gameplay.

You must support the `--simulate-inputs-single-player <inputs-file>` flag. The
flag takes a single argument, a file describing the input format. Upon
receiving a valid file, your binary should print to stdout a serialization of
the board state. You must also support the flag `resources` which specifies a
directory containing JSON files with information such as tetromino shapes,
config, or wall kicks (these will all be explained later). As an example
(assuming you are running your project with `cargo run`):
```
$ cargo run -- --simulate-inputs-single-player input-file.txt --resources resources
...o......
...ooo....
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
......o...
.....oo...
o....oo.o.
ooooooo.oo
oo...o..o.
ooooooooo.
ooooooooo.
ooooooooo.
ooooooooo.
```

We provide further examples with the release code.

### Input Format
The input format is an ascii encoded text file. It describes a single action
per frame of gameplay. The actions can be any of the following
- C: Rotate 90 degrees clockwise
- W: Rotate 90 degrees counter-clockwise
- L: Move piece left
- R: Move piece right
- D: Move piece down
- H: Hard drop piece
- E: No action

Inputs are concatinated into a sequence. For example, an input file describing
rotating clockwise twice, moving a piece left, waiting 3 frames, and then hard
dropping the piece would be:
```
CCLEEEH
```

You should ignore whitespace, so the above input is the same as writing
```
CCL
EEE H
```

### Output Format
The serialized output is a 10x22 grid containing "o" for every cell containing a
part of a tetronmino and "." for every empty cell. You should draw the current
falling piece in addition to any currently filled cells in on the board.

Assuming the piece being placed is a J piece, and the next pieces is an O piece.
the input sequence from above induces the output:
```
....oo....
....oo....
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..ooo.....
....o.....
```

When topped out (the game is over, more precisely defined later), don't show any
next approaching piece.

### Game Behavior
#### Game State
We loosely base our game behavior on the
[Tetris Guidelines](https://tetris.wiki/Tetris_Guideline). Our game will run in
discrete "frames" (which I might also refer to as ticks). Typically there are
60 frames per second, though that is irrelevant for this part of the assignment.

There are many constants used by Tetris which we will define in the provided
`config.json`.

We start by describing the game state at a given frame:
- A 10x20 grid of cells, each cell either filled or empty
- A random piece generator often called a 7-bag
- The current piece on the board
- The position of the current falling piece
- The rotation of the current falling piece
- The number of frames since the current falling piece has been forcefully moved
  down one row
- The gravity: the number of frames it takes for a piece to forcefully move
  down one row
- The number of rotations which have taken place since a piece has been able to
  move down one row

#### Randomizer
A [7-bag](https://tetris.wiki/Random_Generator) is the random generator Tetris
uses for supplying one of 7 pieces to the player to replace a placed piece. It
is analogous to putting each of the seven pieces into a bag and drawing them
randomly without replacement. When the bag is empty, those 7 pieces are placed
back into the bag and drawing continues.

When implementing your randomizer, you *must use the
[rand](https://docs.rs/rand/latest/rand/) and
[rand_xoshiro](https://docs.rs/rand_xoshiro/latest/rand_xoshiro/) crates with
the random seed from `config.json` and the `Xoshiro128PlusPlus` algorithm.*
We provide an implementation matching the staff solution below. Please use it
to be sure you pass tests:
```rs
type RngAlgorithm = Xoshiro128PlusPlus;

/// `BagRandomizer<T>` creates a bag randomizer of items of type T. This can
/// be thought of as a bag of items which are drawn without replacement. Once
/// the bag is empty, the bag is refilled with the items initally drawn. This
/// bounds the time between drawing a duplicate item to the total number of
/// items 2 * (#items - 1).
///
/// This struct provides a `cur()` method to get an item from the randomzier.
/// The same item will be returned until it is removed with a `next()` method.
///
/// ```rs
/// let mut randomizer = BagRandomizer::from_parts(1..=3, [4; 16]);
/// let first_item = *randomizer.cur();
/// let same_item = *randomizer.cur();
/// assert_eq!(first_item, same_item);
///
/// randomizer.next();
/// let different_item = *randomizer.cur();
/// assert_ne!(first_item, different_item);
///
/// randomizer.next();
/// let different_item = *randomizer.cur();
/// assert_ne!(first_item, different_item);
/// ```
pub struct BagRandomizer<T> {
    bag: Vec<T>,
    idx: usize,
    rng: RngAlgorithm,
}

impl<T> BagRandomizer<T> {
    /// Creates a `BagRandomizer` from an interator of items to put in the bag
    /// and a seed for the underlying RNG.
    pub fn from_parts(
        bag: impl Iterator<Item = T>,
        seed: <RngAlgorithm as SeedableRng>::Seed,
    ) -> Self {
        let mut rng = RngAlgorithm::from_seed(seed);

        let mut bag: Vec<T> = bag.collect();
        bag.shuffle(&mut rng);

        Self { idx: 0, bag, rng }
    }

    /// Removes the current exposed value in the bag, cycling to the next
    /// random value.
    pub fn next(&mut self) {
        self.idx += 1;
        if self.idx >= self.bag.len() {
            self.bag.shuffle(&mut self.rng);
            self.idx = 0;
        }
    }

    /// Returns the current exposed value in the bag.
    pub fn cur(&self) -> &T {
        &self.bag[self.idx]
    }
}
```

#### Per frame Sequence
Initially, a piece is drawn from the 7-bag. the position of the piece will be at
column 4, (more generally, (grid width - 1) / 2) and row 20 (more generally
height) both of these 0 indexed. It's rotation will be rotation 0, the spawn
rotation, and the number of frames since the piece has moved down will be 0.

On each frame, the game will:
1. Read the next input from the input file.
2. Perform an action based on the read input.
3. Increment the number of frames since a piece has forcefully moved down a
  row.
  - If this number if greater than the gravity and the piece can be moved down
    a row, move the piece down a row.
  - If this number is greater than the pieces *lock delay*, the number of
    frames before a piece locks into place, fill the cells currently taken up
    by the piece.
  - If any cell of this piece are above row 20, stop reading inputs and
    advancing the game state. The player "topped out" and lost.
4. If any rows are composed entirely of filled with cells, remove the row and
  move each above row down towards the bottom.
5. If a piece was locked, pick a new piece from the 7-bag as the current piece
  and spawn it in column 4 row 20 with rotation 0.

Notice that checks for losses occur before checks for line clears. The wiki
refers to this condition as a [partial lock out](https://tetris.wiki/Top_out).

(2) is underspecified. We explain it in depth in the following sections.

#### Rotations
This is for handling C and W commands.

We use the
[Super Rotation System (SRS)](https://tetris.wiki/Super_Rotation_System). it is
worth reading the linked wiki page (it has useful pictures!) though we will try
and summarize the important parts here.

SRS involves specifying the cells occupied by a tetromino for each rotation,
notated 0 (spawn or no rotations), R (right), 2 (two rotations), L (left).

Concretely, for each rotation we specify a length four list of the cells
occupied by the tetromino. We concatinate, in order, the lists representing
rotations 0, R, 2, and L. For example, we represent J with the following JSON:
```json
{
  "states": [
    [[ -1, 1], [-1,  0], [ 0,  0], [ 1,  0]],
    [[  0, 1], [ 1,  1], [ 0,  0], [ 0,  1]],
    [[ -1, 0], [ 0,  0], [ 1,  0], [ 1,  1]],
    [[  0, 1], [ 0,  0], [ 0,  1], [-1,  1]]
  ],
}
```
Each pair is a horizontal and then vertical offset where right and up are the
positive directions.

When the current piece is rotated, it's possible for it to overlap with filled
cells it didn't before. Because it's more fun, Tetris has a "wall kick" system
where depending on the piece and states being transitioned between, the game
will attempted an ordered list of translations on the piece to remove overlaps.
This can be represented as a compressed table. Each tetromino's rotation is
assigned a list of pairs. The offset is then the element-wise difference of
those lists. For example, the J piece has the following table (not we use "o"
to represent 0 and "t" to represent 2):
```
{
    "o": [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
    "r": [[ 0,  0], [ 1,  0], [ 1, -1], [ 0,  2], [ 1,  2]],
    "t": [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
    "l": [[ 0,  0], [-1,  0], [-1, -1], [ 0,  2], [-1,  2]]
}
```
If the piece is in the 0 state and transitions to the R state, the offsets
tried will be (in order) `(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)`.

J, L, S, Z, T tetrominoes all use the same wall kicks which we call "Normal". I
and O use special wall kick lists which we entitle "I" and "O" respectively.
Theses values can be found in `wall_kicks.json`.

For understanding wall kicks and a visualization I strongly recommend you read
[this](https://tetris.wiki/Super_Rotation_System#Wall_Kicks) and
[this](https://tetris.wiki/Super_Rotation_System#How_Guideline_SRS_Really_Works).
section of the wiki. Though, reading the entire thing might be best.

#### Translations
This is for handling L and R commands.

Translations left and right are straightforward, subtracting from the piece's
column on L and incrementing it on R. If the piece would be translated such
that one of its cells overlaps with another filled cell or is no longer on the
grid, the translation command should be ignored.

#### Drops
This is for handling D and H commands. D commands represent "soft drops" and H
commands represent "hard drops."

A soft drop temporarily decreases the gravity value by a value specified in the
config file. This makes the block fall faster.

A hard drop instantly moves a piece downwards until that piece cannot move down
a row without one of its cells overlapping a fill cell or being below row 0. It
then locks the piece in place. If all of the cells of th piece are above row
20, stop reading inputs and stop updating game state. The player topped out!

There is also a [wiki page on this](https://tetris.wiki/Drop) which is useful to
read, but note they define gravity as the reciprocal of what we do i.e. we use
the unit 1/G for gravity.

### Part 1 Implementation Advice
- We specify a lot of information in JSON files in `resources`. Reading JSON is
  annoying, so recommend you use the dependencies `serde` and `serde_json`. A
  small example is in the docs [here](https://serde.rs/derive.html).
- Writing a CLI from scratch isn't that hard, but there are also nice libraries
  like [clap](https://docs.rs/clap/latest/clap/) (used in lab 6) or
  [argh](https://docs.rs/argh/latest/argh/). Feel free to use either of these.
- It probably make sense to separate the logic for loading JSON files into its
  own struct. Similarly, it likely makes sense to make structs for tetrominoes,
  the game board, and game itself, which calls functions provided by the game
  board.
- Implement board serialization as soon as possible. This will allow you to
  test features as you implement them.
- Implement input format parsing so you can do end to end testing as soon as
  possible.
- Write unit or integration tests for your code as you implement new features.
- When locking a piece in place, remember to reset the counter tracking time
  since a piece has moved down a row due to gravity.

## Other Parts
Currently being worked on. Should be out very soon.

## Testing
We use [runt](https://github.com/rachitnigam/runt) for integration tests. You
can install it with `cargo install runt`. You can then run the integration
tests by building with `cargo build --release` and running `runt cli-tests`.

We encourage you to reading the documentation for `runt` (or simply following)
our example to add your own integration tests.

## Design Document
You may submit your design document as either a TXT (with at most 80 character
lines), Markdown, or PDF file. You should list the name and netids of all
students who contributed to your project on this document.

Your document should include the following sections
1. Architecture: Describe the modules you wrote and any important structs or
  traits. Describe how they interact and any invariants you maintain between
  them.
2. Implementation: Describe any interesting algorithms you use or otherwise
  creative or non-obvious implementation decisions. Explain where you expect
  there may be performance bottlenecks in your code and how you identified
  these spots. In particular, what was your bot's algorithm?
3. Testing: Describe how you tested your design and what parts of your project
  did you devote more or less verification effort towards.
  verification effort towards.
4. Work Credit: If you worked in a group, say who did what.
5. Known Issues: If there is something you know doesn't work, describe it here.
6. AI Usage: If you used any AI tools describe how you used them. We are
  curious how you use the tools, especially given the limiting AI policy.
7. Meta Comments: Write any opinions on the assignment. Some things we are
  curious about are:
  - how long did you spend on the assignment, which parts took the longest?
  - what did you like and dislike about the assignment, why?
  - is there anything you'd like to change about the assignment?

Acknowledgement: The requirements for this document are loosely based off of the
requirements for the design document required by CS 4120's assignments.
