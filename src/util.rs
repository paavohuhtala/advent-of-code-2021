pub fn parse_lines(str: &str) -> Vec<i32> {
    str.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
