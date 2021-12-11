use crate::array2d::Array2D;

const INPUT: &str = include_str!("./day11.txt");

#[derive(Debug, Clone, Copy)]
enum OctopusState {
    Charging(u8),
    Flashed,
}

fn flash_octopus(world: &mut Array2D<OctopusState>, i: usize, flashes: &mut usize) {
    let octopus = &mut world.data[i];

    match octopus {
        &mut OctopusState::Flashed => {
            return;
        }
        _ => {}
    }

    *flashes += 1;

    *octopus = OctopusState::Flashed;

    let mut adjacent = Vec::new();
    world.adjacent_mut(i, &mut adjacent);

    let mut flashed = Vec::new();

    for (i, octopus) in adjacent {
        match octopus {
            OctopusState::Charging(n) => {
                *n += 1;

                if *n > 9 {
                    flashed.push(i);
                }
            }
            OctopusState::Flashed => {}
        }
    }

    for i in flashed {
        flash_octopus(world, i, flashes);
    }
}

pub fn a() {
    let mut input = Array2D::from_string(INPUT).map(OctopusState::Charging);
    let mut steps = 0;
    let mut flashes = 0;

    while steps < 100 {
        for i in 0..input.data.len() {
            let octopus = &mut input.data[i];

            match octopus {
                OctopusState::Charging(n) => {
                    *n += 1;

                    if *n > 9 {
                        flash_octopus(&mut input, i, &mut flashes);
                    }
                }
                _ => {}
            }
        }

        for i in 0..input.data.len() {
            let octopus = &mut input.data[i];

            match octopus {
                OctopusState::Flashed => {
                    *octopus = OctopusState::Charging(0);
                }
                _ => {}
            }
        }

        steps += 1;
    }

    println!("Day11a: {}", flashes);
}

pub fn b() {
    let mut input = Array2D::from_string(INPUT).map(OctopusState::Charging);
    let mut steps = 0;

    loop {
        for i in 0..input.data.len() {
            let octopus = &mut input.data[i];

            match octopus {
                OctopusState::Charging(n) => {
                    *n += 1;

                    if *n > 9 {
                        flash_octopus(&mut input, i, &mut 0);
                    }
                }
                _ => {}
            }
        }

        let mut all_flashed = true;

        for i in 0..input.data.len() {
            let octopus = &mut input.data[i];

            match octopus {
                OctopusState::Flashed => {
                    *octopus = OctopusState::Charging(0);
                }
                OctopusState::Charging(_) => {
                    all_flashed = false;
                }
            }
        }

        steps += 1;

        if all_flashed {
            break;
        }
    }

    println!("Day11b: {}", steps);
}
