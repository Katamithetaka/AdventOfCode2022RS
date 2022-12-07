use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

pub enum Action {
    Lose,
    Equal,
    Win,
}

const WIN_SCORE: u32 = 6;
const TIE_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;

impl Move {
    pub fn get_winning_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        }
    }

    pub fn get_losing_move(&self) -> Move {
        // could be optimized by using a match
        // However, it's already fast enough
        self.get_winning_move().get_winning_move()
    }

    pub fn as_u32(&self) -> u32 {
        return self.clone().into();
    }

    pub fn get_score(&self, move2: &Move) -> u32 {
        if self == &move2.get_winning_move() {
            return self.as_u32() + WIN_SCORE;
        } else if self == move2 {
            return self.as_u32() + TIE_SCORE;
        } else {
            return self.as_u32() + LOSE_SCORE;
        }
    }

    pub fn get_score_from_action(&self, action: &Action) -> u32 {
        match action {
            Action::Lose => self.get_losing_move().as_u32() + LOSE_SCORE,
            Action::Equal => self.as_u32() + TIE_SCORE,
            Action::Win => self.get_winning_move().as_u32() + WIN_SCORE,
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = {
            let chars = s.chars().collect::<Vec<_>>();

            if chars.len() != 1 {
                return Err("Not a valid value to be converted into an action".to_owned());
            } else {
                chars[0]
            }
        };

        match char {
            'X' | 'A' => Ok(Move::Rock),
            'Y' | 'B' => Ok(Move::Paper),
            'Z' | 'C' => Ok(Move::Scissor),
            _ => Err("Not a valid move".to_owned()),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = {
            let chars = s.chars().collect::<Vec<_>>();

            if chars.len() != 1 {
                return Err("Not a valid value to be converted into an action".to_owned());
            } else {
                chars[0]
            }
        };

        match char {
            'X' => Ok(Action::Lose),
            'Y' => Ok(Action::Equal),
            'Z' => Ok(Action::Win),
            _ => Err("Not a valid move".to_owned()),
        }
    }
}

impl Into<u32> for &Move {
    fn into(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}
