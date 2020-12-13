fn main() {
    let timestamp = 1001612;
    let buses = vec![19,41,37,821,13,17,29,463,23];
    let (bus, departs) = first_bus(&buses, timestamp);
    let wait = departs - timestamp;

    println!("Bus {} departs at {} ({} minute wait).", bus, departs, wait);
    println!("{} * {} = {}", bus, wait, bus * wait);
}

fn departure(bus: usize, timestamp: usize) -> usize {
    let factor = timestamp.div_euclid(bus);
    if timestamp.rem_euclid(bus) == 0 {
        bus * factor
    } else {
        bus * (factor + 1)
    }
}

fn first_bus(buses: &[usize], timestamp: usize) -> (usize, usize) {
    buses.iter()
        .map(|&bus| (bus, departure(bus, timestamp)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_departure() {
        assert_eq!(departure(59, 939), 944);
        assert_eq!(departure(30, 60), 60);
    }

    #[test]
    fn test_part1() {
        let timestamp = 939;
        let buses = vec![7,13,59,31,19];
        let (bus, departs) = first_bus(&buses, timestamp);
        assert_eq!(bus, 59);
        assert_eq!(departs, 944);
    }
}