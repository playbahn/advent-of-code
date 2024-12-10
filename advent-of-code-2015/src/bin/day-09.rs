use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let routes: String = fs::read_to_string("input/day9.txt").unwrap();
    let mut distances: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut places: HashSet<&str> = HashSet::new();
    let mut shortest: u32 = u32::MAX;
    let mut longest: u32 = u32::MIN;

    for route in routes.lines() {
        let route: Vec<&str> = route.split_ascii_whitespace().collect();
        places.insert(route[0]);
        places.insert(route[2]);

        distances
            .entry(route[0])
            .and_modify(|sub_route: &mut HashMap<&str, u32>| {
                sub_route
                    .entry(route[2])
                    .or_insert(route[4].parse().unwrap());
            })
            .or_insert(HashMap::from([(route[2], route[4].parse().unwrap())]));

        distances
            .entry(route[2])
            .and_modify(|sub_route: &mut HashMap<&str, u32>| {
                sub_route
                    .entry(route[0])
                    .or_insert(route[4].parse().unwrap());
            })
            .or_insert(HashMap::from([(route[0], route[4].parse().unwrap())]));
    }

    println!("{:#?}", distances);

    for place in &places {
        let mut places: HashSet<&str> = places.clone();
        places.remove(place);
        shortest = shortest.min(calc_shortest(places, place, &distances, 0));
    }

    println!("{:#?}", shortest);

    for place in &places {
        let mut places: HashSet<&str> = places.clone();
        places.remove(place);
        longest = longest.max(calc_longest(places, place, &distances, 0));
    }

    println!("{:#?}", longest);
}

fn calc_shortest(
    unvisited: HashSet<&str>,
    last: &str,
    distances: &HashMap<&str, HashMap<&str, u32>>,
    visited_cost: u32,
) -> u32 {
    if unvisited.len() == 1 {
        visited_cost + distances[last][unvisited.iter().next().unwrap()]
    } else {
        let mut shortest: u32 = u32::MAX;
        for next in &unvisited {
            let mut unvisited: HashSet<&str> = unvisited.clone();
            unvisited.remove(next);
            shortest = shortest.min(calc_shortest(
                unvisited,
                next,
                distances,
                visited_cost + distances[last][next],
            ));
        }

        shortest
    }
}

fn calc_longest(
    unvisited: HashSet<&str>,
    last: &str,
    distances: &HashMap<&str, HashMap<&str, u32>>,
    visited_cost: u32,
) -> u32 {
    if unvisited.len() == 1 {
        visited_cost + distances[last][unvisited.iter().next().unwrap()]
    } else {
        let mut longest: u32 = u32::MIN;
        for next in &unvisited {
            let mut unvisited: HashSet<&str> = unvisited.clone();
            unvisited.remove(next);
            longest = longest.max(calc_longest(
                unvisited,
                next,
                distances,
                visited_cost + distances[last][next],
            ));
        }

        longest
    }
}
