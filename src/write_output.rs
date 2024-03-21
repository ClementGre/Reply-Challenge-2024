use std::io::Write;
use crate::parsing::Game;

pub fn write_output(game: &Game) {
    // Write placed_paths to output file "output.txt" with format path_id x y

    let file = std::fs::File::create("output.txt").expect("Unable to create file");
    let mut writer = std::io::BufWriter::new(file);

    for placed_path in &game.placed_paths {
        writeln!(writer, "{} {} {}", placed_path.id, placed_path.x, placed_path.y).expect("Unable to write data");
    }

    writer.flush().expect("Unable to flush data");

}
