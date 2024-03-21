use std::fmt::Display;
use std::ptr::write;
use crate::parsing::Game;

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Display game dashboard with golden points, silver points and paths

        write!(f, "Game: {}x{}\n", self.w, self.h)?;
        write!(f, "Game With paths: {}x{}\n  ", self.w, self.h)?;
        for j in 0..self.w {
            write!(f, " {} ", j)?;
        }
        write!(f, "\n")?;
        for i in 0..self.h {
            write!(f, "{} ", i)?;
            for j in 0..self.w {
                let mut found = false;
                for golden_point in &self.golden_points {
                    if golden_point.x == j && golden_point.y == i {
                        write!(f, " G ")?;
                        found = true;
                        break;
                    }
                }
                if !found {
                    for silver_point in &self.silver_points {
                        if silver_point.x == j && silver_point.y == i {
                            write!(f, "{}", silver_point.points)?;
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    write!(f, " . ")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "Game With paths: {}x{}\n  ", self.w, self.h)?;
        for j in 0..self.w {
            write!(f, " {} ", j)?;
        }
        write!(f, "\n")?;
        for i in 0..self.h {
            write!(f, "{} ", i)?;
            for j in 0..self.w {
                let mut found = false;
                for golden_point in &self.golden_points {
                    if golden_point.x == j && golden_point.y == i {
                        write!(f, " G ")?;
                        found = true;
                        break;
                    }
                }
                for path in &self.placed_paths {
                    if path.x == j && path.y == i {
                        write!(f, " {} ", path.id)?;
                        found = true;
                        break;
                    }
                }
                if !found {
                    write!(f, " . ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
