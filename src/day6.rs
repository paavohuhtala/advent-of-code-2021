const INPUT: &str = include_str!("./day6.txt");

fn read_input() -> Vec<u8> {
    INPUT.split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

pub fn a() {
    let mut fish = read_input();
    let mut new_fish = Vec::new();

    for _day in 0..80 {
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
            } else {
                *f -= 1;
            }
        }

        fish.append(&mut new_fish);
    }

    println!("Day6a: {}", fish.len());
}

pub fn b() {
    let input = read_input();
    let mut phases: [usize; 9] = [0; 9];

    for i in input {
        phases[i as usize] += 1;
    }

    for _day in 0..256 {
        let mut new_phases = [0; 9];

        for phase in 1..phases.len() {
            new_phases[phase - 1] += phases[phase];
        }

        new_phases[8] += phases[0];
        new_phases[6] += phases[0];

        phases = new_phases;
    }

    let total: usize = phases.iter().sum();

    println!("Day6b: {}", total);
}
