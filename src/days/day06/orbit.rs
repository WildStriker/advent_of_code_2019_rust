use std::collections;
use std::error;
use std::io;

pub struct Mass {
    orbit: Option<usize>,
    orbiters: Vec<usize>,
}

impl Mass {
    fn new() -> Mass {
        Mass {
            orbit: None,
            orbiters: Vec::new(),
        }
    }
}

pub struct Orbits {
    masses: collections::HashMap<usize, Mass>,
    ids: collections::HashMap<String, usize>,
}

impl Orbits {
    /// Count total number of orbits starting from this mass
    fn orbit_count(&self, mass: &Mass) -> usize {
        let mut count = mass.orbiters.len();
        for orbiter in &mass.orbiters {
            count += self.orbit_count(self.masses.get(orbiter).unwrap());
        }
        count
    }

    /// Count of all orbits in the system
    pub fn total_orbits(&self) -> usize {
        let mut total = 0;
        for mass in self.masses.values() {
            total += self.orbit_count(&mass);
        }
        total
    }

    fn new() -> Self {
        Self {
            masses: collections::HashMap::new(),
            ids: collections::HashMap::new(),
        }
    }

    fn insert(&mut self, orbit: String, orbiter: String) {
        let count = self.ids.len();
        let orbit_id = *self.ids.entry(orbit).or_insert(count);

        let count = self.ids.len();
        let orbiter_id = *self.ids.entry(orbiter).or_insert(count);

        let orbit_mass = self.masses.entry(orbit_id).or_insert_with(|| Mass::new());
        orbit_mass.orbiters.push(orbiter_id);

        let orbiter_mass = self.masses.entry(orbiter_id).or_insert_with(|| Mass::new());
        orbiter_mass.orbit = Some(orbit_id);
    }

    /// get mass id from string
    fn get_mass_id(&self, name: &str) -> Result<&usize, String> {
        self.ids
            .get(name)
            .ok_or(format!("Unable to find mass {}", name))
    }

    /// Caculate distance between two bodies in orbit
    /// the calculation is inclusive (includes start and end)
    /// when calculating "orbital transfers" just remove 2
    pub fn calculate_distance(&self, start: &str, end: &str) -> Result<usize, String> {
        let start_id = self.get_mass_id(start)?;
        let end_id = self.get_mass_id(end)?;

        let mut traveled: collections::HashMap<&usize, usize> = collections::HashMap::new();
        let mut next = collections::VecDeque::new();

        traveled.insert(start_id, 0);
        next.push_back(start_id);

        while let Some(mass_id) = next.pop_front() {
            let mass = self.masses.get(mass_id).unwrap();
            let distance = *traveled.get(mass_id).unwrap() + 1;

            if let Some(orbit) = &mass.orbit {
                if !traveled.contains_key(orbit) {
                    if orbit == end_id {
                        return Ok(distance);
                    }
                    traveled.insert(orbit, distance);
                    next.push_back(orbit);
                }
            }
            for orbiter in &mass.orbiters {
                if !traveled.contains_key(orbiter) {
                    if orbiter == end_id {
                        return Ok(distance);
                    }
                    traveled.insert(orbiter, distance);
                    next.push_back(orbiter);
                }
            }
        }

        Err(format!("Unable to find a path from {} to {}", start, end))
    }
}

/// Parse reader and return the initialized Orbits
pub fn parse_orbits<T>(reader: T) -> Result<Orbits, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut orbits = Orbits::new();

    for res in reader.lines() {
        let line = res?;

        let mut masses = line.split(")");

        let orbit = masses.next().ok_or_else(|| "Unable to parse orbit")?;
        let orbiter = masses.next().ok_or_else(|| "Unable to parse orbiter")?;

        orbits.insert(orbit.to_owned(), orbiter.to_owned());
    }
    Ok(orbits)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_orbits() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
        .as_bytes();

        let orbits = parse_orbits(input).unwrap();
        let count = orbits.total_orbits();

        assert_eq!(count, 42);
    }

    #[test]
    fn test_calculate_distance() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            .as_bytes();

        let orbits = parse_orbits(input).unwrap();
        let distance = orbits.calculate_distance("YOU", "SAN").unwrap();

        assert_eq!(distance, 6);
    }
}
