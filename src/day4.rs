#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(self, r: &Range) -> bool {
        self.start <= r.start && r.end <= self.end
    }

    fn overlaps(self, r: &Range) -> bool {
        if self.start < r.start {
            self.end >= r.start
        } else {
            self.start <= r.end
        }
    }

    fn from(s: &str) -> Self {
        let segments = s.split('-').collect::<Vec<&str>>();

        Range {
            start: segments[0].parse().unwrap(),
            end: segments[1].parse().unwrap(),
        }
    }
}

pub fn pt1(input: &str) -> usize {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<&str>>();
            (Range::from(ranges[0]), Range::from(ranges[1]))
        })
        .filter(|(r1, r2)| {
            r1.contains(r2) || r2.contains(r1)
        })
        .count()
}

pub fn pt2(input: &str) -> usize {
    input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<&str>>();
            (Range::from(ranges[0]), Range::from(ranges[1]))
        })
        .filter(|(r1, r2)| {
            r1.overlaps(r2)
        })
        .count()
}

mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/4.txt");

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(EXAMPLE), 2);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2(EXAMPLE), 4);

    }
}
