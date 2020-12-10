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

    let mut total_arrangements: i64 = 1;
    for p in Partitions::of(&joltage) {
        if p.len() > 2 {
            let mut number_legal = 0;
            for mut arrangement in arrangements(&p[1..p.len()-1]) {
                let mut v = Vec::with_capacity(p.len());
                v.push(p[0]);
                v.append(&mut arrangement);
                v.push(p[p.len() - 1]);
                if is_legal(&v) {
                    number_legal += 1;
                }
            }
            total_arrangements *= number_legal;
        }
    }
    println!("There are {} possible arrangements.", total_arrangements);
}

struct Partitions<'a> {
    index: usize,
    numbers: &'a [usize],
}

impl<'a> Partitions<'a> {
    pub fn of(numbers: &[usize]) -> Partitions {
        Partitions { index: 0, numbers }
    }
}

impl<'a> Iterator for Partitions<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.numbers.len() {
            return None;
        }

        let mut partition = Vec::new();
        let mut previous = self.numbers[self.index];
        partition.push(previous);
        self.index += 1;
        for &number in &self.numbers[self.index..] {
            if (number - previous) >= 3 {
                break;
            }
            partition.push(number);
            previous = number;
            self.index += 1;
        }

        Some(partition)
    }
}

fn is_legal(numbers: &[usize]) -> bool {
    !numbers
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .any(|difference| difference > 3)
}

fn arrangements(numbers: &[usize]) -> Vec<Vec<usize>> {
    let mut a = Vec::new();
    if numbers.len() > 1 {
        for mut arrangement in arrangements(&numbers[1..]) {
            a.push(arrangement.clone());
            let mut b = Vec::with_capacity(arrangement.len() + 1);
            b.push(numbers[0]);
            b.append(&mut arrangement);
            a.push(b);
        }
    } else {
        a.push(vec![numbers[0]]);
        a.push(Vec::new());
    }

    a
}
