use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    // get()으로 가져올 경우 None을 반환하여 패닉을 방지함
    println!("{:?}", city_hashmap["Bielefeld"]);      // "Germany"
    println!("{:?}", city_hashmap.get("Bielefeld"));  // Some("Germany")
    println!("{:?}", city_hashmap.get("Bielefeldd")); // None
}
