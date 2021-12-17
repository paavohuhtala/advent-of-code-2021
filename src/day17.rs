use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("./day17.txt");

#[derive(Debug)]
struct Area {
    x: (i32, i32),
    y: (i32, i32),
}

#[derive(Debug, Clone)]
struct State {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl State {
    fn new_with_velocity(velocity: (i32, i32)) -> Self {
        State {
            position: (0, 0),
            velocity,
        }
    }
}

fn simulate_step(state: &State) -> State {
    let (x, y) = state.position;
    let (vx, vy) = state.velocity;

    let new_x = x + vx;
    let new_y = y + vy;

    let new_vx = if vx < 0 {
        vx + 1
    } else if vx > 0 {
        vx - 1
    } else {
        0
    };

    let new_vy = vy - 1;

    State {
        position: (new_x, new_y),
        velocity: (new_vx, new_vy),
    }
}

fn read_input() -> Area {
    let (x_min, x_max, y_min, y_max) = scan_fmt!(
        INPUT,
        "target area: x={d}..{d}, y={d}..{d}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();

    Area {
        x: (x_min, x_max),
        y: (y_min, y_max),
    }
}

// Draws the trajectory of the probe.
// Target area is marked with 'T'
// The first position of the probe is marked with 'S'
// The remaining positions are marked with '#'
// The rest is marked with '.'
#[allow(dead_code)]
fn draw_trajectory(positions: &[(i32, i32)], target_area: &Area) {
    let (area_x_min, area_x_max) = target_area.x;
    let (area_y_min, area_y_max) = target_area.y;

    let x_min = area_x_min.min(positions.iter().map(|(x, _)| *x).min().unwrap());
    let x_max = area_x_max.max(positions.iter().map(|(x, _)| *x).max().unwrap());

    let y_min = area_y_min.min(positions.iter().map(|(_, y)| *y).min().unwrap());
    let y_max = area_y_max.max(positions.iter().map(|(_, y)| *y).max().unwrap());

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            let position = (x, y);
            if position == positions[0] {
                print!("S");
            } else if positions.contains(&position) {
                print!("#");
            } else if x >= area_x_min && x <= area_x_max && y >= area_y_min && y <= area_y_max {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn is_inside_area((x, y): (i32, i32), area: &Area) -> bool {
    let (x_min, x_max) = area.x;
    let (y_min, y_max) = area.y;

    x >= x_min && x <= x_max && y >= y_min && y <= y_max
}

pub fn ab() {
    let target_area = read_input();
    let mut max_y = 0;

    let mut hits = 0;

    for initial_vel_y in -500..10000 {
        'find_vel: for initial_vel_x in 1..500 {
            let mut state = State::new_with_velocity((initial_vel_x, initial_vel_y));
            let mut positions = vec![state.position];
            let mut try_max_y = 0;

            for _ in 0..10000 {
                state = simulate_step(&state);
                positions.push(state.position);
                try_max_y = state.position.1.max(try_max_y);

                if is_inside_area(state.position, &target_area) {
                    max_y = max_y.max(try_max_y);
                    /*println!(
                        "initial_vel_y: {}, initial_vel_x: {}, max_y: {}, intersection: ({}, {})",
                        initial_vel_y, initial_vel_x, max_y, state.position.0, state.position.1
                    );
                    draw_trajectory(&positions, &target_area);
                    println!();*/

                    hits += 1;

                    continue 'find_vel;
                }

                // no x velocity and x not inside target area
                if state.velocity.0 == 0
                    && (state.position.0 < target_area.x.0 || state.position.0 > target_area.x.1)
                {
                    continue 'find_vel;
                }

                // went past in x direction
                if state.position.0 > target_area.x.1 {
                    continue 'find_vel;
                }

                // went past in y direction
                if state.position.1 < target_area.y.0 {
                    continue 'find_vel;
                }
            }
        }
    }

    println!("Day17a: {}", max_y);
    println!("Day17b: {}", hits);
}
