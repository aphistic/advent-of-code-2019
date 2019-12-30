use std::fs;

mod fuel;
mod mass;

fn main() {
    match fs::File::open("data/input.txt") {
        Ok(f) => {
            let mut fuel_summary = fuel::Summary::new();

            let mass_reader = mass::Reader::new(&f);
            for mass_result in mass_reader {
                match mass_result {
                    Ok(mass) => {
                        fuel_summary.add_mass(mass);
                    }
                    Err(error) => panic!("couldn't read mass: {}", error)
                }
            }

            println!("total fuel: {}", fuel_summary.sum())
        }
        Err(error) => panic!("couldn't open file: {}", error)
    }
}
