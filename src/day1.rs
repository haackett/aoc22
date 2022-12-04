pub fn pt1(input: &str) -> u32 {
    input.split('\n').collect::<Vec<&str>>()
        .group_by(|_, &b| !b.is_empty())
        .map(|x| {
            x.iter().flat_map(|y| y.parse::<u32>()).sum()
        })
        .max().unwrap()
}

pub fn pt2(input: &str) -> u32 {
    let mut elves = input.split('\n').collect::<Vec<&str>>()
        .group_by(|_, &b| !b.is_empty())
        .map(|x| {
            x.iter().flat_map(|y| y.parse::<u32>()).sum()
        })
        .collect::<Vec<u32>>();
    elves.sort();
    elves.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    extern crate test as other_test;
    use other_test::Bencher;
    use super::*;

    const INPUT1: &str = include_str!("../input/1.txt");

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(INPUT1), 69836);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2(INPUT1), 207968);
    }

    #[bench]
    fn bench_pt1(b: &mut Bencher) {
        b.iter(|| {
            other_test::black_box(pt1(INPUT1))
        })
    }

    #[bench]
    fn bench_pt2(b: &mut Bencher) {
        b.iter(|| {
            other_test::black_box(pt2(INPUT1))
        })
    }
}

