#[derive(Clone)]
struct PlayerStats {
    hp: i8,
    mana_left: u16,
    mana_spent: u16,
    // effect timers
    magic_missile: u8,
    drain: u8,
    shield: u8,
    shield_just_now: bool,
    poison: u8,
    poison_just_now: bool,
    recharge: u8,
    recharge_just_now: bool,
}

fn main() {
    let spells: Vec<&str> = Vec::from(["Magic Missile", "Drain", "Shield", "Poison", "Recharge"]);

    let player: PlayerStats = PlayerStats {
        hp: 50,
        mana_left: 500,
        mana_spent: 0,
        magic_missile: 0,
        drain: 0,
        shield: 0,
        shield_just_now: false,
        poison: 0,
        poison_just_now: false,
        recharge: 0,
        recharge_just_now: false,
    };

    let mut mana_least: u16 = u16::MAX;

    for spell in &spells {
        let mut player: PlayerStats = player.clone();
        match *spell {
            "Magic Missile" => {
                player.magic_missile = 1;
                player.mana_spent += 53;
                player.mana_left -= 53;
            }
            "Drain" => {
                player.drain = 1;
                player.mana_spent += 73;
                player.mana_left -= 73;
            }
            "Shield" => {
                player.shield = 6;
                player.shield_just_now = true;
                player.mana_spent += 113;
                player.mana_left -= 113;
            }
            "Poison" => {
                player.poison = 6;
                player.poison_just_now = true;
                player.mana_spent += 173;
                player.mana_left -= 173;
            }
            "Recharge" => {
                player.recharge = 5;
                player.recharge_just_now = true;
                player.mana_spent += 229;
                player.mana_left -= 229;
            }
            _ => panic!(),
        }
        simulate_duel(player, &spells, 55, &mut mana_least);
    }
    
    println!("FROM MAIN: LEAST MANA SPENT: {}", mana_least);
}

fn simulate_duel(
    mut player: PlayerStats,
    spells: &Vec<&str>,
    mut boss_hp: i8,
    mana_least: &mut u16,
) {
    if !(player.mana_spent < *mana_least) {
        return;
    }
    
    // PLAYER'S TURN // SPELLS CASTED PRIOR
    // =================================================
    // PART 2
    player.hp -= 1;
    if player.hp < 1 {
        return;
    }
    // PART 2
    
    if player.recharge_just_now {
        player.recharge_just_now = false;
    } else if player.recharge > 0 {
        player.mana_left += 101;
        player.recharge -= 1;
    }
    
    if player.poison_just_now {
        player.poison_just_now = false;
    } else if player.poison > 0 {
        boss_hp -= 3;
        player.poison -= 1;
    }

    if player.shield_just_now {
        player.shield_just_now = false;
    } else if player.shield > 0 {
        player.shield -= 1;
    }
    
    if player.magic_missile == 1 {
        boss_hp -= 4;
        player.magic_missile -= 1;
    } else if player.drain == 1 {
        boss_hp -= 2;
        player.hp += 2;
        player.drain -= 1;
    }
    
    if boss_hp < 1 {
        *mana_least = (*mana_least).min(player.mana_spent);
        return;
    }
    
    // ====================================================
    // BOSS'S TURN
    // ====================================================
    if player.recharge > 0 {
        player.mana_left += 101;
        player.recharge -= 1;
    }
    
    if player.poison > 0 {
        boss_hp -= 3;
        player.poison -= 1;
    }
    
    if boss_hp < 1 {
        *mana_least = (*mana_least).min(player.mana_spent);
        return;
    }

    if player.shield == 0 {
        player.hp -= 8;
    } else {
        player.hp -= 1;
        player.shield -= 1;
    }

    if player.hp < 1 {
        return;
    }
    
    // ======================================================
    // PLAYER'S NEXT TURN -- ONLY SPELL CAST
    for spell in spells.iter().filter(|spell| match **spell {
        "Magic Missile" => if player.mana_left > 53 { true } else { false },
        "Drain" => if player.mana_left > 73 { true } else { false },
        "Shield" => if player.mana_left > 113 && player.shield < 2 { true } else { false },
        "Poison" => if player.mana_left > 173 && player.poison < 2 { true } else { false },
        "Recharge" => if player.mana_left > 229 && player.recharge < 2 { true } else { false },
        _ => panic!()
    }) {
        let mut player: PlayerStats = player.clone();
        match *spell {
            "Magic Missile" => {
                player.magic_missile = 1;
                player.mana_spent += 53;
                player.mana_left -= 53;
            }
            "Drain" => {
                player.drain = 1;
                player.mana_spent += 73;
                player.mana_left -= 73;
            }
            "Shield" => {
                if player.shield == 1 {
                    player.shield_just_now = false;
                } else {
                    player.shield_just_now = true;
                }
                player.shield += 6;
                player.mana_spent += 113;
                player.mana_left -= 113;
            }
            "Poison" => {
                if player.poison == 1 {
                    player.poison_just_now = false;
                } else {
                    player.poison_just_now = true;
                }
                player.poison += 6;
                player.mana_spent += 173;
                player.mana_left -= 173;
            }
            "Recharge" => {
                if player.recharge == 1 {
                    player.recharge_just_now = false;
                } else {
                    player.recharge_just_now = true;
                }
                player.recharge += 5;
                player.mana_spent += 229;
                player.mana_left -= 229;
            }
            _ => panic!(),
        }
        simulate_duel(player, &spells, boss_hp, mana_least);
    }
}
