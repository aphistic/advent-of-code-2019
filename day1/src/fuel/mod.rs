use libm;

fn for_base_mass(mass: u32) -> u32 {
    match (libm::floor((mass as f64) / 3.) - 2.) as i32 {
        m if m < 0 => 0,
        m => m as u32,
    }
}

fn for_mass(mass: u32) -> u32 {
    let mut total: u32 = 0;

    let mut base_mass: u32 = mass;
    loop {
        match for_base_mass(base_mass) {
            0 => break,
            next_mass => {
                total += next_mass;
                base_mass = next_mass;
            }
        }
    }

    total
}

pub struct Summary {
    masses: Vec<u32>
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            masses: Vec::new(),
        }
    }

    pub fn add_mass(&mut self, mass: u32) {
        self.masses.push(mass)
    }

    pub fn sum(&self) -> u32 {
        self.masses.iter()
            .fold(
                0,
                |acc, mass| acc + for_mass(*mass),
            )
    }
}


#[cfg(test)]
mod tests {
    mod calculation {
        use super::super::*;

        #[test]
        fn for_base_mass_examples() {
            assert_eq!(2, for_base_mass(12));
            assert_eq!(2, for_base_mass(14));
            assert_eq!(654, for_base_mass(1969));
            assert_eq!(33583, for_base_mass(100756));
        }

        #[test]
        fn for_base_mass_negative() {
            assert_eq!(0, for_base_mass(8));
            assert_eq!(0, for_base_mass(2));
        }
    }

    mod summary {
        use super::super::*;

        #[test]
        fn add_mass() {
            let mut s = Summary::new();
            s.add_mass(12);
            s.add_mass(14);
            assert_eq!(s.masses, vec![12, 14])
        }

        #[test]
        fn sum() {
            let mut s = Summary::new();
            s.add_mass(12);
            s.add_mass(14);
            assert_eq!(s.sum(), 4);
        }
    }
}
