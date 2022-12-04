const BEATS: [[usize; 3]; 3] = 
    [[3,  0, 6],
     [6,  3, 0],
     [0,  6, 3]];

#[derive(Clone,Copy)]
enum Throw {
    Rock, 
    Paper,
    Scissors
}

impl Throw {
    fn from_index(idx: usize) -> Throw {
        match idx {
            0 => Throw::Rock, 
            1 => Throw::Paper,
            2 => Throw::Scissors, 
            _ => panic!(),
        }
    }

    fn to_score(self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn to_index(self) -> usize {
        self.to_score() - 1
    }
}

struct Game {
    id: usize,
    left: Throw,
    right: Throw,
}

impl Game {
    fn from(line: &str, id: usize) -> Game {
        let mut pair = line.split(' ');

        let left = match pair.next()
            .unwrap_or_else(|| panic!("Bad input {} at position 0 in Game {}", line, id))
        {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            &_ => panic!("Unexpected input in line {} in Game {}", line, id),
        };
        let right = match pair.next()
            .unwrap_or_else(|| panic!("Bad input {} at position 1 in Game {}", line, id))
        {
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissors,
            &_ => panic!("Unexpected input in line {} in Game {}", line, id),
        };

        Game {
            id,
            left,
            right,
        }
    }

    fn play(self) -> usize {
        let outcome = BEATS[self.right.to_index()][self.left.to_index()];
        outcome + self.right.to_score()
    }
}

pub fn pt1(input: &str) -> i32 {
    input
        .split('\n')
        .flat_map(|line| {
            if line.is_empty() {
                return None
            } 
            Some(line)
        })
        .enumerate()
        .map(|(i,line)| Game::from(line, i).play() as i32)
        .sum()
}

pub fn pt2(input: &str) -> i32 {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(id,line)| {
            if line.is_empty() {
                return None
            } 
            let mut pair = line.split(' ');

            let left = match pair.next()
                .unwrap_or_else(|| panic!("Bad input {} at position 0 in Game {}", line, id))
            {
                "A" => Throw::Rock,
                "B" => Throw::Paper,
                "C" => Throw::Scissors,
                &_ => panic!("Unexpected input in line {} in Game {}", line, id),
            };

            let right = match pair.next()
                .unwrap_or_else(|| panic!("Bad input {} at position 1 in Game {}", line, id))
            {
                "X" => Throw::from_index(BEATS[left.to_index()].iter().position(|x| x == &6).unwrap()),
                "Y" => Throw::from_index(BEATS[left.to_index()].iter().position(|x| x == &3).unwrap()),
                "Z" => Throw::from_index(BEATS[left.to_index()].iter().position(|x| x == &0).unwrap()),
                &_ => panic!("Unexpected input in line {} in Game {}", line, id),
            };

            Some(Game { id, left, right })
        })
        .map(|g| g.play() as i32)
        .sum()
}

mod test {

    #[test]
    fn test_pt1() {

    }

    #[test]
    fn test_pt2() {

    }
}
