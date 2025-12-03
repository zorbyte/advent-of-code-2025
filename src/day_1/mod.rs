mod instructions;

use crate::{
    Result,
    stopwatch::{FinishedStopwatch, Stopwatch},
    tui,
};
use instructions::{Direction, Instruction, generate_instructions};

const INITIAL_DIAL_POSITION: u64 = 50;

pub fn run(input: Option<String>) -> Result<FinishedStopwatch> {
    let input = input.expect("input to solve should be available for day 1");

    let sw = Stopwatch::start();
    let instructions = generate_instructions(input);
    let (part_1, part_2) = find_password(instructions);

    let sw = sw.stop();

    tui::title_banner("Day 1 Answers");
    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(sw)
}

// We are measuring change here, not necessarily numbers,
// so what we really will look at here is the following scenario:
// imagine a series of numbers from 0-99, infinitely re-occurring.
// We'll pick a (hypothetical) middle of this line
// and call it the origin and make a number line with two sides
// a left side and a right side:
//
// ..., 99 | 1, 2, 3, ..., 98, 99 | 1, 2, 3, ..., 98, 99 | 1, ..., ad. inf.
//         0*          L          0 (origin)  R          0
//          [-------------- number line --------------]
// N.B. Zero with Star (0*) is marked as such for a discussion further down.
//
// If we were to fold this piece of the line over the origin (0),
// we perform the same operations, while still maintaining the same distance
// from the origin.
//
// We achieve this by doing dial = 100 - dial, whenever we want to fold
// our number line like this.
//
// We will have our approach (swap) and the naive approach
// demonstrated by example using: dial = 50 and instruction = L68
//
//  naive: (50 - 68) % 100 = -18 => (-18 + 100) % 100 = 82 => 82
//   swap: (100 - 50) = 50 => 50 + 68 = 118 => 118 % 100 = 18 => 18
//         ^ = (100 - dial): performed this time since we don't know what the
//                           last sign was.
//
// notice how both 82 and 18 both share the same distance from the origin
// (that is: 0 or 100; where 100 becomes 0 anyway).
//
// if we were to do R48 next:
//   naive: (82 + 48) % 100 = 30 => (30 + 100) % 100 = 30 => 30
//   swap:  (100 - 18) = 82 => 82 + 48 = 130 => 130 % 100 = 30 => 30
//
// when swapping do a different side, you can see how the numbers
// automatically fix themselves on the swap approach, since we are using
// the universal property of distance from the origin as opposed to an
// actual physical position the dial points to.
//
// as mentioned, the swap method doesn't do (100 - dial) if the direction
// was the same as last time as we don't need to fold our number-line.
// to demonstrate this, we'll do R10 next for example:
//   naive: (30 + 10) % 100 = 40 => (40 + 100) % 100 = 40 => 40
//   swap:  (30) = 30 => 30 + 10 = 40 => 40 % 100 = 40 => 40
//
// The exception to the rule when moving to the other side of the origin
// is that we don't do it when the dial is already at the origin point (dial = 0),
// as this is pointless. The reason for that is because as mentioned,
// we only are measuring the distance from the origin, so adding the magnitude component of
// the instruction will just be the same thing as moving our origin point
// to the this location on the infinite number line, i.e. the origin point will move from
// our current frame of reference to the origin point on the left (notated as 0*).
// The number-line in use is still exactly the same, but we have chosen a new origin point
// along the line of infinitely re-occurring numbers.if
//
// if you did, you would do 100 - 0, meaning
// the dial position you would be working with is 100, an impossibility that should
// instead be folded onto 0.
//
// in speed-running communities, this method of folding a re-occurring number line like this
// is called parallel universes.
fn find_password(instructions: Vec<Instruction>) -> (u64, u64) {
    let (mut part_1, mut part_2) = (0, 0);

    let mut dial = INITIAL_DIAL_POSITION;
    let mut previous_direction = Direction::None;

    for Instruction(direction, magnitude) in instructions {
        // ignore the case of flipping direction when dial already equals 0,
        // since that would just make the dial equal 100, which should be 0 anyway
        dial = if previous_direction != direction && dial != 0 {
            100 - dial
        } else {
            dial
        };

        dial += magnitude;

        part_2 += dial / 100; // find out how many times 100 was crossed e.g. 256 -> 2.56 -> 2;
        dial %= 100; // remove any hundreds, e.g. 256 -> 56.
        if dial == 0 {
            part_1 += 1;
        }

        previous_direction = direction;
    }

    (part_1, part_2)
}
