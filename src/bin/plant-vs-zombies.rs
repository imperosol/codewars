/// [kyu] 3
///
/// [description]
/// This kata is inspired by Plants vs. Zombies,
/// a tower defense video game developed and originally published by PopCap Games.
///
/// The battlefield is the front lawn and the zombies are coming.
/// Our defenses (consisting of pea-shooters) are in place
/// and we've got the stats of each attacking zombie.
/// Your job is to figure out how long it takes for them to penetrate our defenses.
///
/// # Mechanics
/// The two images below represent the lawn
/// (for the test example below) at separate stages in the game.
/// Left: state at move 3, before shooters fire.
/// Right:state at move 5, after shooters fire. (Moves are 0-based).
///
/// ![](https://i.imgur.com/1wSbnMC.png)
///
/// `Moves`: During a move, new zombies appear and/or existing ones
/// move forward one space to the left.
/// Then the shooters fire. This two-step process repeats.
/// If a zombie reaches a shooter's position, it destroys that shooter.
/// In the example image above, the zombie
/// at [4,4] on the left reaches the shooter at [4,2] and destroys it.
/// The zombie has 1 health point remaining and is
/// eliminated in the same move by the shooter at [4,0].
/// `Numbered shooters`: shoot straight (to the right) a given number
/// of times per move.
/// In the example image,
/// the green numbered shooter at [0,0] fires 2 times per move.
/// `S-shooters` : shoot straight, and diagonally
/// upward and downward (ie. three directions simultaneously) once per move.
/// In the example image, the blue and orange S-shooters
/// can attack zombies in any of the blue and orange squares,
/// respectively (if not blocked by other zombies).
/// At move 3 the blue shooter can only hit the zombie at
/// [1,5] while the orange shooter hits each of the zombies
/// at [1,5], [2,7], and [4,6] once for that move.
/// `Shooting Priority`: The numbered shooters fire all their
/// shots in a cluster, then the S-shooters fire their shots
/// in order from right to left, then top to bottom.
/// Note that once a zombie's health reaches
/// 0 it drops immediately and does not absorb
/// any additional shooter pellets.
/// In the example image, the orange S-shooter fires before
/// the blue one.
///
/// # Input
/// Your function will receive two arguments:
///
/// `Lawn Map`
/// :   An array/list consisting of strings,
///     where each string represents a row of the map.
///     Each string will consist of either
///     " " (space character) which represents empty space,
///     a numeric digit (0-9) representing a numbered shooter,
///     or the letter S representing an S-shooter.
/// `Zombie Stats`
/// :   An array of subarrays representing each zombie, in the following format:
///     [i,row,hp] - where i is the move number (0-based)
///     when it appears, row is the row the zombie walks down,
///     and hp is the initial health point value of the zombie.
///     When new zombies appear, they start at the farthest right column of their row.
///     Input will always be valid.
///
/// # Output
/// Return the number of moves before the first zombie penetrates our defenses
/// (by getting past column 0), or null/None if all zombies are eliminated.


mod pnz {
    use crate::pnz::TowerType::*;
    use itertools::Itertools;
    use std::cmp;

    #[derive(Debug, PartialEq, Clone)]
    struct Coords {
        col: usize,
        row: usize,
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Zombie {
        hp: u8,
        pos: Coords,
    }

    #[derive(Debug)]
    struct ZombieWaves(Vec<Vec<Zombie>>);

    /// A tower can either shoot diagonally or straight n times per turn
    #[derive(Debug, Clone)]
    enum TowerType {
        Straight(u8),
        Diagonal,
    }

    #[derive(Debug)]
    struct Tower {
        tower_type: TowerType,
        pos: Coords,
    }

    impl ZombieWaves {
        /// Return the zombies that will come, wave by wave.
        /// If there is a wave without zombie before the final wave,
        /// this wave will be an empty vec.
        fn new(zombies: &[Vec<usize>], nb_cols: usize) -> Self {
            let max_round = zombies.last().unwrap()[0];
            let mut zombies = zombies.iter().rev();
            let waves = (0..=max_round)
                .rev()
                .map(|i| {
                    zombies
                        .take_while_ref(|z| z[0] == i)
                        .map(|vec| Zombie {
                            hp: vec[2] as u8,
                            pos: Coords {
                                col: nb_cols - 1,
                                row: vec[1],
                            },
                        })
                        .collect_vec()
                })
                .collect();
            Self(waves)
        }
    }

    impl Tower {
        fn create_list(lanes: &[&str]) -> Vec<Self> {
            lanes
                .iter()
                .enumerate()
                .flat_map(|(row_ind, &row)| {
                    row.chars()
                        .enumerate()
                        .filter_map(move |(col_ind, c)| match c {
                            'S' => Some(Tower {
                                tower_type: TowerType::Diagonal,
                                pos: Coords {
                                    row: row_ind,
                                    col: col_ind,
                                },
                            }),
                            '1'..='9' => Some(Tower {
                                tower_type: Straight(c.to_digit(10).unwrap() as u8),
                                pos: Coords {
                                    row: row_ind,
                                    col: col_ind,
                                },
                            }),
                            _ => None,
                        })
                })
                .collect()
        }
    }


    #[derive(Debug)]
    struct Game {
        zombies: Vec<Zombie>,
        waves: ZombieWaves,
        towers: Vec<Tower>,
    }

    enum GameOutcome {
        Victory(usize),
        Defeat(usize),
    }

    impl Game {
        fn run(mut self) -> GameOutcome {
            let mut round = 0;
            loop {
                if self.zombies.iter().any(|z| z.pos.col == 0) {
                    return GameOutcome::Defeat(round);
                }
                if self.zombies.is_empty() && self.waves.0.is_empty() {
                    return GameOutcome::Victory(round);
                }
                self.zombies_turn();
                self.towers_turn();
                round += 1;
            }
        }
        fn zombies_turn(&mut self) {
            // zombies that were on the map advance
            self.zombies.iter_mut().for_each(|z| {
                z.pos.col -= 1;
                if let Some(tower_ind) = self.towers.iter().position(|t| t.pos == z.pos) {
                    self.towers.remove(tower_ind);
                }
            });
            // Then new zombies arrive
            if let Some(wave) = self.waves.0.pop() {
                self.zombies.extend(wave);
            }
        }

        fn towers_turn(&mut self) {
            if self.towers.is_empty() || self.zombies.is_empty() {
                return;
            }
            self.straight_towers_shot();
                self.diagonal_towers_shot();
            // remove zombies at 0 hp
            self.zombies.retain(|z| z.hp != 0);
        }

        fn straight_towers_shot(&mut self) {
            let max_row = self.towers.iter().map(|t| t.pos.row).max().unwrap();
            // straight shots first
            let damages_per_row = self
                .towers
                .iter()
                .filter(|t| matches!(t.tower_type, Straight(_)))
                .fold(vec![0; max_row + 1], |mut acc, t| {
                    acc[t.pos.row] += match t.tower_type {
                        Straight(dmg) => dmg,
                        Diagonal => 1,
                    };
                    acc
                });
            // inflict damages
            damages_per_row
                .iter()
                .enumerate()
                .for_each(|(row_ind, dmg)| {
                    let mut remaining = *dmg;
                    self.zombies
                        .iter_mut()
                        .filter(|z| z.pos.row == row_ind)
                        .sorted_by_key(|z| z.pos.col)
                        .for_each(move |z| {
                            let to_substract = cmp::min(z.hp, remaining);
                            z.hp -= to_substract;
                            remaining -= to_substract;
                        });
                });
        }

        fn diagonal_towers_shot(&mut self) {
            self.towers
                .iter()
                .filter(|t| matches!(t.tower_type, Diagonal))
                .sorted_by_key(|t| t.pos.row)
                .rev()
                .sorted_by_key(|t| t.pos.col)
                .rev()
                .for_each(|t| {
                    // zombies upfront
                    if let Some(z) = self
                        .zombies
                        .iter_mut()
                        .filter(|z| z.hp > 0)
                        .filter(|z| z.pos.row == t.pos.row && z.pos.col > t.pos.col)
                        .min_by_key(|z| z.pos.col)
                    {
                        z.hp -= 1
                    }
                    // zombies up
                    if let Some(z) = self
                        .zombies
                        .iter_mut()
                        .filter(|z| z.hp > 0)
                        .filter(|z| z.pos.row > t.pos.row && z.pos.col > t.pos.col)
                        .filter(|z| z.pos.row - t.pos.row == z.pos.col - t.pos.col)
                        .min_by_key(|z| z.pos.col)
                    {
                        z.hp -= 1
                    }
                    // zombies bottom
                    if let Some(z) = self
                        .zombies
                        .iter_mut()
                        .filter(|z| z.hp > 0)
                        .filter(|z| z.pos.row < t.pos.row && z.pos.col > t.pos.col)
                        .filter(|z| t.pos.row - z.pos.row == z.pos.col - t.pos.col)
                        .min_by_key(|z| z.pos.col)
                    {
                        z.hp -= 1
                    }
                });
        }
    }

    pub fn plants_and_zombies(lawn: &[&str], zombies: &[Vec<usize>]) -> usize {
        let game = Game {
            zombies: Vec::new(),
            waves: ZombieWaves::new(zombies, lawn[0].len()),
            towers: Tower::create_list(lawn),
        };
        match game.run() {
            GameOutcome::Victory(_) => 0,
            GameOutcome::Defeat(i) => i,
        }
    }
}

fn main() {
    let example_tests = vec![
        (
            vec!["2       ", "  S     ", "21  S   ", "13      ", "2 3     "],
            vec![
                vec![0, 4, 28],
                vec![1, 1, 6],
                vec![2, 0, 10],
                vec![2, 4, 15],
                vec![3, 2, 16],
                vec![3, 3, 13],
            ],
            10,
        ),
        (
            vec!["11      ", " 2S     ", "11S     ", "3       ", "13      "],
            vec![
                vec![0, 3, 16],
                vec![2, 2, 15],
                vec![2, 1, 16],
                vec![4, 4, 30],
                vec![4, 2, 12],
                vec![5, 0, 14],
                vec![7, 3, 16],
                vec![7, 0, 13],
            ],
            12,
        ),
        (
            vec![
                "12        ",
                "3S        ",
                "2S        ",
                "1S        ",
                "2         ",
                "3         ",
            ],
            vec![
                vec![0, 0, 18],
                vec![2, 3, 12],
                vec![2, 5, 25],
                vec![4, 2, 21],
                vec![6, 1, 35],
                vec![6, 4, 9],
                vec![8, 0, 22],
                vec![8, 1, 8],
                vec![8, 2, 17],
                vec![10, 3, 18],
                vec![11, 0, 15],
                vec![12, 4, 21],
            ],
            20,
        ),
        (
            vec!["12      ", "2S      ", "1S      ", "2S      ", "3       "],
            vec![
                vec![0, 0, 15],
                vec![1, 1, 18],
                vec![2, 2, 14],
                vec![3, 3, 15],
                vec![4, 4, 13],
                vec![5, 0, 12],
                vec![6, 1, 19],
                vec![7, 2, 11],
                vec![8, 3, 17],
                vec![9, 4, 18],
                vec![10, 0, 15],
                vec![11, 4, 14],
            ],
            19,
        ),
        (
            vec![
                "1         ",
                "SS        ",
                "SSS       ",
                "SSS       ",
                "SS        ",
                "1         ",
            ],
            vec![
                vec![0, 2, 16],
                vec![1, 3, 19],
                vec![2, 0, 18],
                vec![4, 2, 21],
                vec![6, 3, 20],
                vec![7, 5, 17],
                vec![8, 1, 21],
                vec![8, 2, 11],
                vec![9, 0, 10],
                vec![11, 4, 23],
                vec![12, 1, 15],
                vec![13, 3, 22],
            ],
            0,
        ),
    ];

    example_tests
        .into_iter()
        .for_each(|(grid, zqueue, sol)| assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol));
}
