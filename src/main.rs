use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
};

#[derive(Debug)]
struct Game {
    player_idx: usize,
    nb_game: usize,
}

#[derive(Debug, Clone, Default)]
struct VoteManager {
    votes: HashMap<Discriminant<GameInfo>, [f64; 4]>,
}

impl VoteManager {
    fn clear(&mut self) {
        self.votes.clear();
    }

    fn vote(&mut self, game: &GameInfo, votes: [f64; 4]) {
        if matches!(game, GameInfo::Unused) {
            return;
        }
        let key = discriminant(game);
        self.votes.insert(key, votes);
    }

    fn get_result(&mut self) -> Input {
        let mut results = self
            .votes
            .values()
            .copied()
            .map(|a| {
                let mut i = 0;
                a.map(|w| {
                    (
                        {
                            let temp = i;
                            i += 1;
                            NUM_TO_INPUT[temp]
                        },
                        w,
                    )
                })
            })
            .collect::<Vec<_>>();
        loop {
            let mut clone = results.clone();
            let total = clone.iter().flatten().map(|(_, w)| *w).sum::<f64>();
            if total == 0.0 {
                break Input::Up;
            }
            clone.iter_mut().flatten().for_each(|(_, w)| *w /= total);
            let mut res = HashMap::new();
            clone
                .iter()
                .flatten()
                .filter(|(_, w)| *w != 0.0)
                .for_each(|(i, w)| *res.entry(*i).or_insert(0f64) += *w);
            if let Some((i, _)) = res.iter().find(|(_, w)| **w >= 0.5) {
                break *i;
            };
            let min = *(res
                .iter()
                .min_by(|(_, r), (_, l)| r.partial_cmp(l).unwrap())
                .unwrap()
                .0);
            results.iter_mut().for_each(|v| v[min as usize].1 = 0.0f64);
        }
    }
}

trait NextLine {
    fn next_line(&mut self) -> Result<String>;
    fn pnext_line(&mut self) -> Result<String> {
        match self.next_line() {
            Ok(s) => {
                eprintln!("input was '{s:?}'");
                Ok(s)
            }
            Err(e) => Err(e),
        }
    }
}

impl<B: std::io::BufRead> NextLine for std::io::Lines<B> {
    fn next_line(&mut self) -> Result<String> {
        let val = self.next().ok_or("no input left")??;
        Ok(val)
    }
}
#[derive(Debug, Default, Copy, Clone)]
struct Score {
    gold: usize,
    silver: usize,
    bronze: usize,
}

#[derive(Debug, Default, Copy, Clone)]
struct PlayerInfo {
    final_score: usize,
    scores: [Score; 4],
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

impl Vec2d<i32> {
    fn mag(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
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
                let line = io.pnext_line()?;
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
                let line = io.pnext_line()?;
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
                let line = io.pnext_line()?;
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
                let line = io.pnext_line()?;
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
            for i in 0..4 {
                player.scores[i].gold = line.next().ok_or("no field!")?.parse()?;
                player.scores[i].silver = line.next().ok_or("no field!")?.parse()?;
                player.scores[i].bronze = line.next().ok_or("no field!")?.parse()?;
            }
        }
        drop(io);
        for g in &mut info.games {
            g.fill_info()?;
        }
        Ok(info)
    }
}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
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

const NUM_TO_INPUT: [Input; 4] = {
    let mut arr = [Input::Up; 4];
    arr[Input::Up as usize] = Input::Up;
    arr[Input::Left as usize] = Input::Left;
    arr[Input::Right as usize] = Input::Right;
    arr[Input::Down as usize] = Input::Down;
    arr
};

macro_rules! cast_vote {
    {
        ($n1:ident: $w1:literal),
        ($n2:ident: $w2:literal),
        ($n3:ident: $w3:literal),
        ($n4:ident: $w4:literal)$(,)?
    } => {
        {
            let mut arr = [0.0; 4];
            arr[Input::$n1 as usize] = Input::$w1;
            arr[Input::$n2 as usize] = Input::$w2;
            arr[Input::$n3 as usize] = Input::$w3;
            arr[Input::$n4 as usize] = Input::$w4;
            arr
        }
    };
    {
        ($n1:ident: $w1:literal),
        ($n2:ident: $w2:literal),
        ($n3:ident: $w3:literal)$(,)?
    } => {
        {
            let mut arr = [0.0f64; 4];
            arr[Input::$n1 as usize] = $w1;
            arr[Input::$n2 as usize] = $w2;
            arr[Input::$n3 as usize] = $w3;
            arr
        }
    };
    {
        ($n1:ident: $w1:literal),
        ($n2:ident: $w2:literal)$(,)?
    } => {
        {
            let mut arr = [0.0f64; 4];
            arr[Input::$n1 as usize] = $w1;
            arr[Input::$n2 as usize] = $w2;
            arr
        }
    };
    {
        ($n1:ident: $w1:literal)$(,)?
    } => {
        {
            let mut arr = [0.0f64; 4];
            arr[Input::$n1 as usize] = $w1;
            arr
        }
    };
    {
    } => {
        {
            let mut arr = [0.0f64; 4];
            arr
        }
    };
    {$($name:ident),*$(,)?} => {
        {
            let mut arr = [0.0f64; 4];
            let mut w = 1.0f64;
            #[allow(unused)]
            {
            $(
                arr[Input::$name as usize] = w;
                w += 1.0;
            )*
            }
            arr
        }
    };
}

fn main() -> Result<()> {
    let mut game = Game::parse()?;
    let mut votes = VoteManager::default();
    loop {
        let turn = game.get_turn()?;
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

                    // votes.vote(
                    //     game_info,
                    //     match next_fence.saturating_sub(player.pos) {
                    //         0 | 1 => cast_vote![Up, Right, Down, Left],
                    //         2 => cast_vote![Left, Up, Right, Down],
                    //         3 => cast_vote![Down, Up, Right, Left],
                    //         _ => cast_vote![Right, Down, Up, Left],
                    //     },
                    // )
                }
                GameInfo::Archery { gpu, players } => {
                    if gpu.is_empty() {
                        continue;
                    }
                    let pos = players[game.player_idx];
                    let wind = *(gpu.first().unwrap()) as i32;

                    let mut locvotes = [
                        (
                            Input::Up,
                            Vec2d {
                                x: pos.x,
                                y: pos.y + wind,
                            },
                        ),
                        (
                            Input::Down,
                            Vec2d {
                                x: pos.x,
                                y: pos.y - wind,
                            },
                        ),
                        (
                            Input::Left,
                            Vec2d {
                                x: pos.x + wind,
                                y: pos.y,
                            },
                        ),
                        (
                            Input::Right,
                            Vec2d {
                                x: pos.x - wind,
                                y: pos.y,
                            },
                        ),
                    ];
                    locvotes.sort_by(|l, r| l.1.mag().partial_cmp(&r.1.mag()).unwrap());
                    votes.vote(game_info, {
                        let mut arr = [0.0f64; 4];
                        locvotes
                            .iter()
                            .enumerate()
                            .for_each(|(idx, (input, _))| arr[*input as usize] = idx as f64);
                        arr
                    })
                }
                _ => {
                    eprintln!("minigame not handled!");
                }
            }
        }
        println!("{}", votes.get_result());
    }
}
