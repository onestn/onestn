#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Not it's {self:?}");
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}


fn main() {
    let mut new_animal = Animal::new();

    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}
