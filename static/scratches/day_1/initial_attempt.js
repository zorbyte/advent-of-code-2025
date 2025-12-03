const EXAMPLE_INPUT = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`;

const INITIAL_DIAL_VALUE = 50;

function findPassword(dial = INITIAL_DIAL_VALUE, input = EXAMPLE_INPUT) {
    let part1Count = 0, part2Count = 0;
    let previousSign = 0; // no sign yet.
    for (const instruction of input.trim().split("\n")) {
        const currentSign = instruction.startsWith("R") ? 1 : -1;
        const delta = Number.parseInt(instruction.slice(1));

        // NOTE FROM FUTURE: Explainer is better in the actual rust module.

        // we are measuring change here, not neccessarily numbers,
        // so what we really will look at here is the following scenario:
        // imagine a series of numbers from 0-99, infinitely re-ocurring.
        // we'll pick say (a hypothetical middle) of this line
        // and call it the origin and make a number line:
        //
        // ... | 1, 2, 3, ..., 98, 99 | 1, 2, 3, ..., 98, 99 | ... ad. inf.
        //     0           L          0 (origin)  R          0
        //       [-------------- number line --------------]
        // if we were to fold it over the origin (0), we perform the same operations,
        // while still maintaining the same distance from the origin.
        //
        // we achieve this by doing dial = 100 - dial, whenever we want to fold
        // our number line like this.
        //
        // we will have our approach and the naive approach
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
        // was the same as last time as we don't need to fold our numberline.
        // to demonstrate this, we'll do R10 next for example:
        //   naive: (30 + 10) % 100 = 40 => (40 + 100) % 100 = 40 => 40
        //   swap:  (30) = 30 => 30 + 10 = 40 => 40 % 100 = 40 => 40
        //
        // the exception to the rule when flipping sign is that we don't do it when the dial is already
        // at the origin point, as this is pointless. the reason for that is because as mentioned,
        // we only are measuring the distance from the origin, so adding the delta (the non-directional)
        // piece of the instruction will just be the same thing as opoerating on the "left-side" of the line.
        //
        // if you did, you would do 100 - 0, meaning
        // the dial position you would be working with is 100, an impossibility that should
        // instead be folded onto 0.
        //
        // in speedrunning communities, this method of folding a re-occurring number line like this
        // is called parallel universes.
        dial = previousSign !== currentSign && dial !== 0 ? 100 - dial : dial;
        dial += delta;
        part2Count += Math.trunc(dial / 100); // find out how many times 100 was crossed e.g. 256 -> 2.56 -> 2;
        dial %= 100; // remove any hundreds, e.g. 256 -> 56.
        if (dial === 0) part1Count++;
        previousSign = currentSign;
    }

    console.log(`Part 1: ${part1Count}. Part 2: ${part2Count}`);
}

findPassword();
