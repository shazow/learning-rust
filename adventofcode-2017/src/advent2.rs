pub fn advent2part1(input: &str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        let parsed: Vec<u32> = line.split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect();
        let min = parsed.iter().min().expect("no values");
        let max = parsed.iter().max().expect("no values");
        acc += max - min;
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::error::Error;

    fn load_input(path: &str) -> Result<String, Box<Error>> {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        Ok(buf)
    }

    #[test]
    fn test_advent2() {
        let contents = load_input("src/advent2.input").expect("load input failed");
        let res = advent2part1(&contents);
        assert_eq!(30994, res);
    }
}
