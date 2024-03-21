use std::fmt::{Display, Formatter};
use crate::{Direction, PlacedPath};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub w: u16,
    pub h: u16,
    pub golden_points: Vec<GoldenPoint>,
    pub silver_points: Vec<SilverPoint>,
    pub paths: Vec<Path>,
    pub placed_paths: Vec<PlacedPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoldenPoint {
    pub x: u16,
    pub y: u16,
    pub is_linked: bool,
}

impl GoldenPoint {
    pub fn distance(&self, x: u16, y: u16) -> u16 {
        ((self.x as i16 - x as i16).abs() + (self.y as i16 - y as i16).abs()) as u16
    }
    pub fn distance_to(&self, other: &GoldenPoint) -> u16 {
        ((self.x as i16 - other.x as i16).abs() + (self.y as i16 - other.y as i16).abs()) as u16
    }
    pub fn coordinates(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SilverPoint {
    pub x: u16,
    pub y: u16,
    pub points: u16,
    pub linked: bool
}

impl SilverPoint {
    pub fn coordinates(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl From<&SilverPoint> for Point {
    fn from(value: &SilverPoint) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<&GoldenPoint> for Point {
    fn from(value: &GoldenPoint) -> Self {
        Point {
            x: value.x,
            y: value.y,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PathType {
    Z3,
    Z5,
    Z6,
    Z7,
    Z9,
    Z96,
    A,
    A5,
    B,
    C,
    C3,
    D,
    E,
    F,
}

impl PathType {
    pub fn get_connections(&self) -> Vec<Vec<Direction>> { // (up, right, down, left)
        match self {
            PathType::Z3 => vec![vec![Direction::Left, Direction::Right]],
            PathType::Z5 => vec![vec![Direction::Down, Direction::Right]],
            PathType::Z6 => vec![vec![Direction::Down, Direction::Left]],
            PathType::Z7 => vec![vec![Direction::Down, Direction::Right, Direction::Left]],
            PathType::Z9 => vec![vec![Direction::Up, Direction::Right]],
            PathType::Z96 => vec![vec![Direction::Down, Direction::Left],vec![Direction::Up, Direction::Right]],
            PathType::A => vec![vec![Direction::Up, Direction::Left]],
            PathType::A5 => vec![vec![Direction::Down, Direction::Right],vec![Direction::Up, Direction::Left]],
            PathType::B => vec![vec![Direction::Up, Direction::Right, Direction::Left]],
            PathType::C => vec![vec![Direction::Down, Direction::Up]],
            PathType::C3 => vec![vec![Direction::Down, Direction::Up],vec![Direction::Left, Direction::Right]],
            PathType::D => vec![vec![Direction::Down, Direction::Right, Direction::Up]],
            PathType::E => vec![vec![Direction::Down, Direction::Left, Direction::Up]],
            PathType::F => vec![vec![Direction::Down, Direction::Right, Direction::Left, Direction::Up]],
        }
    }
}
impl Display for PathType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathType::Z3 => write!(f, "3"),
            PathType::Z5 => write!(f, "5"),
            PathType::Z6 => write!(f, "6"),
            PathType::Z7 => write!(f, "7"),
            PathType::Z9 => write!(f, "9"),
            PathType::Z96 => write!(f, "96"),
            PathType::A => write!(f, "A"),
            PathType::A5 => write!(f, "A5"),
            PathType::B => write!(f, "B"),
            PathType::C => write!(f, "C"),
            PathType::C3 => write!(f, "C3"),
            PathType::D => write!(f, "D"),
            PathType::E => write!(f, "E"),
            PathType::F => write!(f, "F"),

        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub id: PathType,
    pub cost: u8,
    pub available: u16,
}


pub fn parse_game(name: &str) -> Game {
    let content_str = std::fs::read_to_string(format!("data/{}.txt", name))
        .expect("Something went wrong reading the file");

    let mut contents = content_str.split("\r\n").collect::<Vec<&str>>();

    if contents[0].starts_with("\u{feff}") {
        contents[0] = &contents[0][3..];
    }

    let first_line = contents[0].split(' ').collect::<Vec<&str>>();

    let w = first_line[0].parse::<u16>().unwrap();
    let h = first_line[1].parse::<u16>().unwrap();
    let nb_golden_points = first_line[2].parse::<u16>().unwrap();
    let nb_silver_points = first_line[3].parse::<u16>().unwrap();

    let mut golden_points = Vec::new();
    let mut silver_points = Vec::new();
    let mut available_paths = Vec::new();

    for i in 1..nb_golden_points + 1 {
        let line = contents[i as usize].split(' ').collect::<Vec<&str>>();
        golden_points.push(GoldenPoint {
            x: line[0].parse::<u16>().unwrap(),
            y: line[1].parse::<u16>().unwrap(),
            is_linked: false
        });
    }

    for i in nb_golden_points + 1..nb_golden_points + nb_silver_points + 1 {
        let line = contents[i as usize].split(' ').collect::<Vec<&str>>();
        silver_points.push(SilverPoint {
            x: line[0].parse::<u16>().unwrap(),
            y: line[1].parse::<u16>().unwrap(),
            points: line[2].parse::<u16>().unwrap(),
            linked: false
        });
    }

    for i in (nb_golden_points + nb_silver_points + 1) as usize..contents.len() {
        let line = contents[i].split(' ').collect::<Vec<&str>>();

        if line.len() < 3 {
            continue;
        }
        let id = match line[0] {
            "3" => Some(PathType::Z3),
            "5" => Some(PathType::Z5),
            "6" => Some(PathType::Z6),
            "7" => Some(PathType::Z7),
            "9" => Some(PathType::Z9),
            "96" => Some(PathType::Z96),
            "A" => Some(PathType::A),
            "A5" => Some(PathType::A5),
            "B" => Some(PathType::B),
            "C" => Some(PathType::C),
            "C3" => Some(PathType::C3),
            "D" => Some(PathType::D),
            "E" => Some(PathType::E),
            "F" => Some(PathType::F),
            _ => None,
        }.unwrap();
        available_paths.push(Path {
            id,
            cost: line[1].parse::<u8>().unwrap(),
            available: line[2].parse::<u16>().unwrap(),
        });
    }

    Game {
        w,
        h,
        golden_points,
        silver_points,
        paths: available_paths,
        placed_paths: vec![],
    }
}
