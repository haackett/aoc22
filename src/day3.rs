fn char_to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

pub fn pt1(input: &str) -> u32 {
    input.split('\n')
        .flat_map(|line| {
            if line.is_empty() {
                None
            } else {
                let left = &line[0..line.len() / 2];
                let right = &line[line.len() / 2 .. line.len()];
                let common_char = left.chars().find(|c| right.contains(*c)).unwrap();
                Some(char_to_priority(common_char))
            }
        })
        .sum()
}

pub fn pt2(input: &str) -> u32 {
    input.split('\n')
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .map(|x| {
            let common_char = x[0].chars().find(|c| {
                x[1].contains(*c) && x[2].contains(*c)
            }).unwrap();
            char_to_priority(common_char)
        })
        .sum()
}

mod test {
    use super::*;

    const EXAMPLE3: &str = include_str!("../examples/3.txt");

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(EXAMPLE3), 157);
    }

    #[test]
    fn test_pt2() {

    }
}
