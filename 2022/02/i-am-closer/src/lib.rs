pub fn challenge_02(lines: &Vec<&str>) -> Option<(u64, u64)> {
    let total_scores = lines
        .iter()
        .map(|&s| {
            let pair = s.split(' ').collect::<Vec<_>>();
            let opponent = pair[0].parse::<Shape>().unwrap();
            let mine_p1 = pair[1].parse::<Shape>().unwrap();
            let mine_p2 = opponent.find_match(pair[1]).unwrap();

            let p1 = mine_p1.score() + mine_p1.compete_points(&opponent);
            let p2 = mine_p2.score() + mine_p2.compete_points(&opponent);
            (p1, p2)
        })
        .fold((0, 0), |(accu_p1, accu_p2), (score_p1, score_p2)| {
            (accu_p1 + score_p1, accu_p2 + score_p2)
        });

    Some(total_scores)
}

#[derive(Debug, PartialEq)]
enum ParseShapeError {
    InvalidValue,
}

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl std::str::FromStr for Shape {
    type Err = ParseShapeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseShapeError::InvalidValue),
        }
    }
}

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn compete_points(&self, opponent: &Self) -> u64 {
        match self {
            Self::Rock => match opponent {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6,
            },
            Self::Paper => match opponent {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0,
            },
            Self::Scissors => match opponent {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3,
            },
        }
    }

    fn find_match(&self, expected: &str) -> Option<Self> {
        match expected {
            "X" => match self {
                // "You need to lose"
                Self::Rock => Some(Self::Scissors),
                Self::Paper => Some(Self::Rock),
                Self::Scissors => Some(Self::Paper),
            },
            "Y" => match self {
                // "You need to draw"
                Self::Rock => Some(Self::Rock),
                Self::Paper => Some(Self::Paper),
                Self::Scissors => Some(Self::Scissors),
            },
            "Z" => match self {
                // "You need to win"
                Self::Rock => Some(Self::Paper),
                Self::Paper => Some(Self::Scissors),
                Self::Scissors => Some(Self::Rock),
            },
            _ => None,
        }
    }
}

#[test]
fn test_parses() {
    let test_fn = |s: &str| s.parse::<Shape>();
    assert_eq!(test_fn("A"), Ok(Shape::Rock));
    assert_eq!(test_fn("B"), Ok(Shape::Paper));
    assert_eq!(test_fn("C"), Ok(Shape::Scissors));
    assert_eq!(test_fn("X"), Ok(Shape::Rock));
    assert_eq!(test_fn("Y"), Ok(Shape::Paper));
    assert_eq!(test_fn("Z"), Ok(Shape::Scissors));
    assert_eq!(test_fn("D"), Err(ParseShapeError::InvalidValue));
    assert_eq!(test_fn("W"), Err(ParseShapeError::InvalidValue));
}

#[test]
fn test_scores() {
    assert_eq!(Shape::Rock.score(), 1);
    assert_eq!(Shape::Paper.score(), 2);
    assert_eq!(Shape::Scissors.score(), 3);
}

#[test]
fn test_lost_cases() {
    assert_eq!(Shape::Rock.compete_points(&Shape::Paper), 0);
    assert_eq!(Shape::Paper.compete_points(&Shape::Scissors), 0);
    assert_eq!(Shape::Scissors.compete_points(&Shape::Rock), 0);
}

#[test]
fn test_draw_cases() {
    assert_eq!(Shape::Rock.compete_points(&Shape::Rock), 3);
    assert_eq!(Shape::Paper.compete_points(&Shape::Paper), 3);
    assert_eq!(Shape::Scissors.compete_points(&Shape::Scissors), 3);
}

#[test]
fn test_win_cases() {
    assert_eq!(Shape::Rock.compete_points(&Shape::Scissors), 6);
    assert_eq!(Shape::Paper.compete_points(&Shape::Rock), 6);
    assert_eq!(Shape::Scissors.compete_points(&Shape::Paper), 6);
}

#[test]
fn test_find_match() {
    assert_eq!(Shape::Rock.find_match("X"), Some(Shape::Scissors));
    assert_eq!(Shape::Paper.find_match("Y"), Some(Shape::Paper));
    assert_eq!(Shape::Scissors.find_match("Z"), Some(Shape::Rock));
    assert_eq!(Shape::Scissors.find_match("W"), None);
}
