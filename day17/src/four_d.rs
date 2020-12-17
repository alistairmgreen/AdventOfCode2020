use std::collections::HashSet;
pub type Coordinate = (i32, i32, i32, i32);

pub fn read_cubes(cubes: &str) -> HashSet<Coordinate> {
    let mut grid = HashSet::with_capacity(cubes.len());

    for (y, line) in cubes.lines().enumerate() {
        for (x, cube) in line.trim().chars().enumerate() {
            if cube == '#' {
                grid.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    grid
}

fn neighbours(position: Coordinate, grid: &HashSet<Coordinate>) -> usize {
    let (pos_x, pos_y, pos_z, pos_w) = position;
    let mut count = 0;

    for x in (pos_x - 1)..=(pos_x + 1) {
        for y in (pos_y - 1)..=(pos_y + 1) {
            for z in (pos_z - 1)..=(pos_z + 1) {
                for w in (pos_w - 1)..=(pos_w + 1) {
                    if (x, y, z, w) == (pos_x, pos_y, pos_z, pos_w) {
                        continue;
                    }

                    if grid.contains(&(x, y, z, w)) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn bounds(grid: &HashSet<Coordinate>) -> (Coordinate, Coordinate) {
    let mut min = (0, 0, 0, 0);
    let mut max = (0, 0, 0, 0);

    for &(x, y, z, w) in grid.iter() {
        if x < min.0 {
            min.0 = x;
        }

        if x > max.0 {
            max.0 = x;
        }

        if y < min.1 {
            min.1 = y;
        }

        if y > max.1 {
            max.1 = y;
        }

        if z < min.2 {
            min.2 = z;
        }

        if z > max.2 {
            max.2 = z;
        }

        if w < min.3 {
            min.3 = w;
        }

        if w > max.3 {
            max.3 = w;
        }
    }

    (min, max)
}

pub fn next_generation(previous: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut next = previous.clone();

    next.retain(|&coord| matches!(neighbours(coord, &previous), 2 | 3));

    let (min, max) = bounds(&previous);

    for x in (min.0 - 1)..=(max.0 + 1) {
        for y in (min.1 - 1)..=(max.1 + 1) {
            for z in (min.2 - 1)..=(max.2 + 1) {
                for w in (min.3 - 1)..=(max.3 + 1) {
                    if neighbours((x, y, z, w), &previous) == 3 {
                        next.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    next
}
