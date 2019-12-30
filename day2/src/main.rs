mod intcode;

fn main() {
    match intcode::Program::load_from_file("data/input.txt") {
        Ok(mut p) => {
            for noun in 0..=99 {
                for verb in 0..=99 {
                    match p.call(noun, verb) {
                        Ok(output) => match output {
                            19690720 => {
                                println!("output: {}", 100 * noun + verb);
                                return
                            }
                            _ => continue,
                        }
                        Err(e) => println!("error calling program: {}", e),
                    }

                }
            }
        }
        Err(e) => println!("couldn't load file: {}", e)
    }
    println!("not found")
}
