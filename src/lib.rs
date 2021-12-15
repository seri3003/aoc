mod aoc;

#[derive(Debug)]
struct Submarine {
    horizontal_pos: i32,
    depth: i32,
    aim: Option<i32>,
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            horizontal_pos: 0,
            depth: 0,
            aim: None,
        }
    }
    pub fn with_aim(mut self, aim: Option<i32>) -> Self {
        self.aim = aim;
        self
    }
    pub fn move_sub(&mut self, dir: Direction) {
        match dir {
            Direction::Up(y) => {
                self.aim.map_or_else(|| self.depth -= y, |x| self.aim = Some(x - y));
            }
            Direction::Down(y) => {
                self.aim.map_or_else(|| self.depth += y, |x|self.aim = Some(x + y));
            }
            Direction::Forward(x) => {
                self.horizontal_pos += x;
                self.depth += self.aim.map_or(0, |a| a * x);
            }
        }
        println!("Sub {:#?} after {:#?}",self, dir);
    }

    pub fn calc_position(&self) -> i32 {
        self.depth * self.horizontal_pos
    }
}

impl From<&str> for Direction {
    fn from(txt: &str) -> Self {
        let raw_txt = txt.to_lowercase().trim().to_string();

        if let Some(dir) = raw_txt
            .split_once(' ')
            .map(|(dir, val)| (dir, val.parse::<i32>().unwrap_or(0)))
        {
            match dir.0 {
                "up" => Direction::Up(dir.1),
                "down" => Direction::Down(dir.1),
                "forward" => Direction::Forward(dir.1),
                _ => Direction::Up(0),
            }
        } else {
            Direction::Up(0)
        }
    }
}

/// Calculates the changes in depth if depth in increased
pub fn measure_depth(depth: &[u32]) -> u32 {
    let deep = depth.iter();
    let mut counter = 0;
    depth
        .iter()
        .zip(deep.skip(1))
        .map(|(cur, ne)| {
            if cur < ne {
                counter += 1
            }
        })
        .count();
    counter
}

/// Applies a windowed with a windowsize of 3 to the meassured depth
pub fn measure_depth_window(depth: &[u32]) -> u32 {
    let mut summed_val: Vec<u32> = Vec::new();

    depth
        .windows(3)
        .map(|x| summed_val.push(x.iter().sum()))
        .count();

    measure_depth(&summed_val)
}

#[cfg(test)]
mod tests {
    use super::aoc::*;
    use super::*;

    #[test]
    fn test_measure_depth() {
        let simple_test = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let res = measure_depth(&simple_test);
        assert_eq!(res, 7);
    }
    #[test]
    fn test_measure_depth_real() {
        let res = measure_depth(day1::MEASUREMENTS);
        assert_eq!(res, 1448);
    }
    #[test]
    fn test_measure_depth_window() {
        let simple_test = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let res = measure_depth_window(&simple_test);
        assert_eq!(res, 5);
    }
    #[test]
    fn test_measure_depth_window_real() {
        let res = measure_depth_window(day1::MEASUREMENTS);
        assert_eq!(res, 1471);
    }
    #[test]
    fn test_submarine_movement() {
        let movement = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let mut sub: Submarine = Submarine::new();
        movement.iter().map(|&dir| sub.move_sub(dir.into())).count();

        assert_eq!(sub.calc_position(), 150);
    }
    #[test]
    fn test_submarine_movement_with_aim() {
        let movement = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let mut sub: Submarine = Submarine::new().with_aim(Some(0));
        movement.iter().map(|&dir| sub.move_sub(dir.into())).count();

        assert_eq!(sub.calc_position(), 900);
    }
    #[test]
    fn test_submarine_movement_real() {
        let movement = day2::MOVEMENT;
        let mut sub: Submarine = Submarine::new();
        movement.iter().map(|&dir| sub.move_sub(dir.into())).count();

        assert_eq!(sub.calc_position(), 2187380);
    }
    #[test]
    fn test_submarine_movement_with_aim_real() {
        let movement = day2::MOVEMENT;
        let mut sub: Submarine = Submarine::new().with_aim(Some(0));
        movement.iter().map(|&dir| sub.move_sub(dir.into())).count();

        assert_eq!(sub.calc_position(), 2187380);
    }
}
