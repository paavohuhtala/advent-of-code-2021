const INPUT: &str = include_str!("./day2.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Forward
}

struct Command(Direction, i32);

fn get_commands() -> Vec<Command> {
  INPUT.lines().map(|line| {
    let (direction, units) = line.split_once(' ').unwrap();
    let direction = match direction {
      "up" => Direction::Up,
      "down" => Direction::Down,
      "forward" => Direction::Forward,
      _ => panic!(),
    };

    let units = units.parse().unwrap();

    Command(direction, units)
  }).collect()
}

fn day2a() {
  let commands = get_commands().into_iter().fold((0, 0), |(x, y), command| {
    match command.0 {
      Direction::Up => (x, y - command.1),
      Direction::Down => (x, y + command.1),
      Direction::Forward => (x + command.1, y),
    }
  });

  let (horizontal, vertical) = commands;

  println!("Day2a: {}", horizontal * vertical);
}

fn day2b() {
  let commands = get_commands().into_iter().fold((0, 0, 0), |(x, y, aim), command| {
    match command.0 {
      Direction::Down => (x, y, aim + command.1),
      Direction::Up => (x, y, aim - command.1),
      Direction::Forward => (x + command.1, y + (aim * command.1), aim),
    }
  });

  let (horizontal, vertical, _aim) = commands;

  println!("Day2b: {}", horizontal * vertical);
}

pub fn day2() {
    day2a();
    day2b();
}
