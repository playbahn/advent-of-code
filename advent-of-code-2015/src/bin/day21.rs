use std::cmp::Ordering;

use dinglebit_combinatorics::Combination;

fn main() {
    let weapons: [(u16, u16, u16); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor: [(u16, u16, u16); 6] = [
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings: [(u16, u16, u16); 6] = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let mut ring_combinations: Vec<Vec<usize>> = Vec::new();

    for ring in 1..3 {
        for c in Combination::new(6, ring) {
            ring_combinations.push(c);
        }
    }

    let mut win_cost: u16 = u16::MAX;
    let mut lose_cost: u16 = u16::MIN;

    for weapon in weapons {
        for armor in armor {
            win_cost = win_cost.min(simulate_fight(&[weapon, armor], "win"));
            lose_cost = lose_cost.max(simulate_fight(&[weapon, armor], "lose"));

            for ring_combination in &ring_combinations {
                match ring_combination.len() {
                    1 => {
                        win_cost = win_cost.min(simulate_fight(
                            &[weapon, armor, rings[ring_combination[0]]],
                            "win",
                        ));
                        lose_cost = lose_cost.max(simulate_fight(
                            &[weapon, armor, rings[ring_combination[0]]],
                            "lose",
                        ));
                    }
                    2 => {
                        win_cost = win_cost.min(simulate_fight(
                            &[
                                weapon,
                                armor,
                                rings[ring_combination[0]],
                                rings[ring_combination[1]],
                            ],
                            "win",
                        ));
                        lose_cost = lose_cost.max(simulate_fight(
                            &[
                                weapon,
                                armor,
                                rings[ring_combination[0]],
                                rings[ring_combination[1]],
                            ],
                            "lose",
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{}", win_cost);
    println!("{}", lose_cost);
}

fn simulate_fight(stats: &[(u16, u16, u16)], outcome: &str) -> u16 {
    let mut player_hp: i16 = 100;
    let mut boss_hp: i16 = 100;
    const BOSS_DAMAGE: u16 = 8;
    const BOSS_ARMOR: u16 = 2;

    let (cost, player_damage, player_armor): (u16, u16, u16) = stats
        .iter()
        .fold((0, 0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1, acc.2 + e.2));

    let mut turn: char = 'P';

    while player_hp > 0 && boss_hp > 0 {
        match (
            turn,
            player_damage.cmp(&BOSS_ARMOR),
            BOSS_DAMAGE.cmp(&player_armor),
        ) {
            ('P', Ordering::Greater, _) => {
                boss_hp -= player_damage as i16 - BOSS_ARMOR as i16;
                turn = 'B';
            }
            ('P', _, _) => {
                boss_hp -= 1;
                turn = 'B';
            }
            ('B', _, Ordering::Greater) => {
                player_hp -= BOSS_DAMAGE as i16 - player_armor as i16;
                turn = 'P';
            }
            ('B', _, _) => {
                player_hp -= 1;
                turn = 'P';
            }
            _ => panic!(),
        }
    }

    match (outcome, player_hp.cmp(&boss_hp)) {
        ("win", Ordering::Greater) => cost,
        ("win", Ordering::Less) => u16::MAX,
        ("lose", Ordering::Greater) => u16::MIN,
        ("lose", Ordering::Less) => cost,
        _ => panic!(),
    }
}
