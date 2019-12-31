mod wires;

fn main() {
    match wires::Grid::parse_file("data/input.txt") {
        Ok(grid) => match grid.shortest_steps() {
            Some(steps) => println!("shortest steps: {}", steps),
            None => println!("no intersection found"),
        }
        Err(e) => println!("error parsing grid: {}", e)
    }
}
