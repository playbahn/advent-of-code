use std::fs;

fn main() {
    let config: String = fs::read_to_string("input/day18.txt").unwrap();
    let config: Vec<Vec<char>> = config
        .lines()
        .map(|light_row: &str| light_row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut config_old: [[char; 100]; 100] = [['\0'; 100]; 100];
    let mut config_new: [[char; 100]; 100] = [['\0'; 100]; 100];

    for (x, light_row) in config.iter().enumerate() {
        for (y, light) in light_row.iter().enumerate() {
            config_old[x][y] = *light;
        }
    }

    let mut neighbor_lights_on: u8;

    for _ in 0..100 {
        for x in 0..100 {
            for y in 0..100 {
                // neighboring lights check
                neighbor_lights_on = 0;

                for xx in 1.max(x) - 1..100.min(x + 2) {
                    for yy in (1.max(y) - 1..100.min(y + 2)).filter(|yy| (xx, *yy) != (x, y)) {
                        if config_old[xx][yy] == '#' {
                            neighbor_lights_on += 1;
                        }
                    }
                }

                match (config_old[x][y], neighbor_lights_on) {
                    ('#', 2|3) => config_new[x][y] = '#',
                    ('#',  _ ) => config_new[x][y] = '.',
                    ('.',   3) => config_new[x][y] = '#',
                    ('.',  _ ) => config_new[x][y] = '.',
                    _ => {},
                }
            }
        }

        // PART 2
        config_new[00][00] = '#';
        config_new[00][99] = '#';
        config_new[99][00] = '#';
        config_new[99][99] = '#';
        // PART 2

        config_old = config_new;
    }

    let mut total_lights_on: u16 = 0;

    for x in 0..100 {
        for y in 0..100 {
            if config_new[x][y] == '#' {
                total_lights_on += 1;
            }
        }
    }

    println!("{}", total_lights_on);
}
