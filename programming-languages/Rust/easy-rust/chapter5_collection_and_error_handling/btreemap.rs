use std::collections::BTreeMap; // 순서가 있음

struct City {
    name: String,
    population: BTreeMap<u32, u32>,
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }
}

