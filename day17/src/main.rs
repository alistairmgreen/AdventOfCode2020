use day17::{three_d, four_d};

fn main() {
    let layout = ".##..#.#
    ##.#...#
    ##.#.##.
    ..#..###
    ####.#..
    ...##..#
    #.#####.
    #.#.##.#";

    let cycles = 6;


    let mut grid = three_d::read_cubes(layout);  
    for _ in 0..cycles {
        let next = three_d::next_generation(&grid);
        grid = next;
    }

    println!(
        "3D: After {} cycles there are {} active cubes.",
        cycles,
        grid.len()
    );

    let mut grid = four_d::read_cubes(layout);
    for _ in 0..cycles {
        let next = four_d::next_generation(&grid);
        grid = next;
    }

    println!(
        "4D: After {} cycles there are {} active cubes.",
        cycles,
        grid.len()
    );
}