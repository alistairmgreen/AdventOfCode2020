fn main() {
    let mut joltage: Vec<usize> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    joltage.push(0); // the charging outlet
    joltage.sort_unstable();

    let adapter = joltage.last().unwrap() + 3;
    joltage.push(adapter);

    let mut differences = vec![0; 4];

    for pair in joltage.windows(2) {
        let difference = pair[1] - pair[0];
        assert!(difference <= 3);
        differences[difference] += 1;
    }

    println!(
        "{} differences of 1 jolt, {} differences of 3 jolts; product = {}",
        differences[1],
        differences[3],
        differences[1] * differences[3]
    );
}
