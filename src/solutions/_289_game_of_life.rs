struct Solution;

#[derive(Copy, Clone)]
enum State {
    Dead = 0,
    Live = 1,
    LiveToDead = 2,
    DeadToLive = 3,
    LiveToLive = 4,
    DeadToDead = 5,
    Unknown = 6,
}

impl State {
    fn from_i32(x: i32) -> State {
        match x {
            0 => State::Dead,
            1 => State::Live,
            2 => State::LiveToDead,
            3 => State::DeadToLive,
            4 => State::LiveToLive,
            5 => State::DeadToDead,
            _ => State::Unknown,
        }
    }

    fn to_i32(self) -> i32 {
        self as i32
    }

    fn to_live(self) -> i32 {
        match self {
            State::LiveToDead | State::Live | State::LiveToLive => 1,
            _ => 0,
        }
    }

    fn next(self, neighbors: i32) -> State {
        match self {
            State::Live => match neighbors {
                0 | 1 => State::LiveToDead,
                2 | 3 => State::LiveToLive,
                _ => State::LiveToDead,
            },
            State::Dead => match neighbors {
                3 => State::DeadToLive,
                _ => State::DeadToDead,
            },
            State::LiveToDead => State::Dead,
            State::DeadToLive => State::Live,
            State::LiveToLive => State::Live,
            State::DeadToDead => State::Dead,
            State::Unknown => State::Unknown,
        }
    }
}

impl Solution {
    fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                let mut neighbors = 0;
                if i > 0 {
                    neighbors += State::from_i32(board[i - 1][j]).to_live();
                }
                if j > 0 {
                    neighbors += State::from_i32(board[i][j - 1]).to_live();
                }
                if i + 1 < n {
                    neighbors += State::from_i32(board[i + 1][j]).to_live();
                }
                if j + 1 < m {
                    neighbors += State::from_i32(board[i][j + 1]).to_live();
                }
                if i > 0 && j > 0 {
                    neighbors += State::from_i32(board[i - 1][j - 1]).to_live();
                }
                if i + 1 < n && j > 0 {
                    neighbors += State::from_i32(board[i + 1][j - 1]).to_live();
                }
                if i + 1 < n && j + 1 < m {
                    neighbors += State::from_i32(board[i + 1][j + 1]).to_live();
                }
                if i > 0 && j + 1 < m {
                    neighbors += State::from_i32(board[i - 1][j + 1]).to_live();
                }
                let current: State = State::from_i32(board[i][j]);
                board[i][j] = current.next(neighbors).to_i32();
            }
        }
        for i in 0..n {
            for j in 0..m {
                board[i][j] = State::from_i32(board[i][j]).next(0).to_i32()
            }
        }
    }
}

#[test]
fn test() {
    let mut board: Vec<Vec<i32>> = vec_vec_i32![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, res);
}
