#[derive(Debug)]
struct Game {
    player_idx: usize,
    nb_game: usize,
}

trait NextLine {
    fn next_line(&mut self) -> Result<String>;
}

impl<B: std::io::BufRead> NextLine for std::io::Lines<B> {
    fn next_line(&mut self) -> Result<String> {
        let val = self.next().ok_or("no input left")??;
        eprintln!("input was '{val:?}'");
        Ok(val)
    }
}
#[derive(Debug, Default, Copy, Clone)]
struct PlayerInfo {
    final_score: usize,
    gold: usize,
    silver: usize,
    bronze: usize,
}

#[derive(Debug, Default, Copy, Clone)]
struct HurdleRacePlayer {
    pos: usize,
    stunt: usize,
}

#[derive(Debug, Default, Copy, Clone)]
struct DivingPlayer {
    points: usize,
    combo: usize,
}

#[derive(Debug, Default, Copy, Clone)]
struct RollerPlayer {
    pos: usize,
    stunt: usize,
    risk: usize,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
struct Vec2d<T> {
    x: T,
    y: T,
}

#[derive(Debug, Default)]
enum GameInfo {
    #[default]
    Unused,
    Diving {
        gpu: Vec<char>,
        players: [DivingPlayer; 3],
    },
    HurdleRace {
        gpu: Vec<char>,
        players: [HurdleRacePlayer; 3],
    },
    Archery {
        gpu: Vec<usize>,
        players: [Vec2d<i32>; 3],
    },
    Roller {
        gpu: Vec<char>,
        players: [RollerPlayer; 3],
        turns_left: usize,
    },
}
impl GameInfo {
    fn fill_info(&mut self) -> Result<()> {
        let mut io = std::io::stdin().lines();
        match self {
            Self::Unused => {}
            Self::Diving { gpu, players } => {
                let line = io.next_line()?;
                let mut line = line.split_whitespace();
                *gpu = line
                    .next()
                    .ok_or("no field!")?
                    .chars()
                    .collect::<Vec<char>>();
                for i in 0..3 {
                    players[i].points = line.next().ok_or("no field!")?.parse()?;
                }
                for i in 0..3 {
                    players[i].combo = line.next().ok_or("no field!")?.parse()?;
                }
            }
            Self::Roller {
                gpu,
                players,
                turns_left,
            } => {
                let line = io.next_line()?;
                let mut line = line.split_whitespace();
                *gpu = line
                    .next()
                    .ok_or("no field!")?
                    .chars()
                    .collect::<Vec<char>>();
                for i in 0..3 {
                    players[i].pos = line.next().ok_or("no field!")?.parse()?;
                }
                for i in 0..3 {
                    let risk_stunt = line.next().ok_or("no field!")?.parse::<i32>()?;
                    if risk_stunt < 0 {
                        players[i].stunt = risk_stunt.abs() as usize;
                        players[i].risk = 0;
                    } else {
                        players[i].stunt = 0;
                        players[i].risk = risk_stunt.abs() as usize;
                    }
                }
                *turns_left = line.next().ok_or("no field!")?.parse()?;
            }
            Self::HurdleRace { gpu, players } => {
                let line = io.next_line()?;
                let mut line = line.split_whitespace();
                *gpu = line
                    .next()
                    .ok_or("no field!")?
                    .chars()
                    .collect::<Vec<char>>();
                players[0].pos = line.next().ok_or("no field!")?.parse()?;
                players[1].pos = line.next().ok_or("no field!")?.parse()?;
                players[2].pos = line.next().ok_or("no field!")?.parse()?;

                players[0].stunt = line.next().ok_or("no field!")?.parse()?;
                players[1].stunt = line.next().ok_or("no field!")?.parse()?;
                players[2].stunt = line.next().ok_or("no field!")?.parse()?;
            }
            Self::Archery { gpu, players } => {
                let line = io.next_line()?;
                let mut line = line.split_whitespace();
                let gpu_tmp = line.next().ok_or("no field!")?;
                *gpu = if gpu_tmp == "GAME_OVER" {
                    Vec::new()
                } else {
                    gpu_tmp
                        .chars()
                        .map(|c| c as usize - b'0' as usize)
                        .collect()
                };
                players[0].x = line.next().ok_or("no field!")?.parse()?;
                players[1].x = line.next().ok_or("no field!")?.parse()?;
                players[2].x = line.next().ok_or("no field!")?.parse()?;

                players[0].y = line.next().ok_or("no field!")?.parse()?;
                players[1].y = line.next().ok_or("no field!")?.parse()?;
                players[2].y = line.next().ok_or("no field!")?.parse()?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct TurnInfo {
    players: [PlayerInfo; 3],
    games: [GameInfo; 4],
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Game {
    fn parse() -> Result<Game> {
        let mut game = Game {
            player_idx: 0,
            nb_game: 0,
        };
        let mut io = std::io::stdin().lines();
        game.player_idx = io.next_line()?.parse()?;
        game.nb_game = io.next_line()?.parse()?;
        Ok(game)
    }

    fn blank_turninfo(&self) -> Result<TurnInfo> {
        Ok(TurnInfo {
            players: Default::default(),
            games: {
                const UNUSED: GameInfo = GameInfo::Unused;
                let mut arr = [UNUSED; 4];
                arr[0] = GameInfo::HurdleRace {
                    gpu: Vec::new(),
                    players: [Default::default(); 3],
                };
                arr[1] = GameInfo::Archery {
                    gpu: Vec::new(),
                    players: [Default::default(); 3],
                };
                arr[2] = GameInfo::Roller {
                    gpu: Vec::new(),
                    players: [Default::default(); 3],
                    turns_left: 0,
                };
                arr[3] = GameInfo::Diving {
                    gpu: Vec::new(),
                    players: [Default::default(); 3],
                };
                arr
            },
        })
    }

    fn get_turn(&mut self) -> Result<TurnInfo> {
        let mut info = self.blank_turninfo()?;
        let mut io = std::io::stdin().lines();
        for player in &mut info.players {
            let line = io.next_line()?;
            let mut line = line.split_whitespace();
            player.final_score = line.next().ok_or("no field!")?.parse()?;
            player.gold = line.next().ok_or("no field!")?.parse()?;
            player.silver = line.next().ok_or("no field!")?.parse()?;
            player.bronze = line.next().ok_or("no field!")?.parse()?;
        }
        drop(io);
        for g in &mut info.games {
            g.fill_info()?;
        }
        Ok(info)
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(usize)]
enum Input {
    Left = 0,
    Right,
    Down,
    Up,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Up => "UP",
                Self::Down => "DOWN",
                Self::Left => "LEFT",
                Self::Right => "RIGHT",
            }
        )
    }
}

fn main() -> Result<()> {
    let mut game = Game::parse()?;
    let mut votes = Vec::<Input>::with_capacity(4);
    loop {
        let mut turn = game.get_turn()?;
        let player_info = &turn.players[game.player_idx];
        votes.clear();
        for game_info in &turn.games {
            match game_info {
                GameInfo::Unused => panic!("wtf"),
                GameInfo::HurdleRace { gpu, players, .. } => {
                    if "GAME_OVER"
                        .chars()
                        .zip(gpu.iter().copied())
                        .all(|(l, r)| l == r)
                    {
                        continue;
                    }
                    let player = &players[game.player_idx];
                    if player.stunt != 0 {
                        continue;
                    }
                    let next_fence = gpu
                        .iter()
                        .copied()
                        .enumerate()
                        .skip(player.pos + 1)
                        .find(|(_, c)| *c == '#')
                        .map(|(i, _)| i)
                        .unwrap_or(usize::MAX);
                    votes.push(match next_fence.saturating_sub(player.pos) {
                        0 | 1 => Input::Up,
                        2 => Input::Left,
                        3 => Input::Down,
                        4.. => Input::Right,
                        _ => Input::Right,
                    })
                }
                _ => {
                    eprintln!("minigame not handled!");
                }
            }
        }
        const NUM_TO_INPUT: [Input; 4] = {
            let mut arr = [Input::Up; 4];
            arr[Input::Up as usize] = Input::Up;
            arr[Input::Left as usize] = Input::Left;
            arr[Input::Right as usize] = Input::Right;
            arr[Input::Down as usize] = Input::Down;
            arr
        };
        let mut counts = [0; 4];
        votes.iter().for_each(|&v| counts[v as usize] += 1);
        eprintln!("{counts:?}");
        let res = if counts[Input::Up as usize] > 0 {
            Input::Up
        } else {
            counts
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, c)| c != 0)
                .max_by_key(|&(_, c)| c)
                .map(|(i, _)| NUM_TO_INPUT[i])
                .unwrap_or(Input::Right)
        };
        println!("{res}");
        eprintln!("{res:?}");
    }
}
