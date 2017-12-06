pub fn advent2part1(input:&str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        let parsed: Vec<u32> = line.split_whitespace().filter_map(|w| w.parse().ok()).collect();
        let min = parsed.iter().min().expect("no values");
        let max = parsed.iter().max().expect("no values");
        acc += max-min;
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_advent2() {
        let mut f = File::open("src/advent2.input").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        let res = advent2part1(&contents);
        assert_eq!(30994, res);
    }
}
