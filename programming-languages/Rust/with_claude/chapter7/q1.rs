mod kitchen {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peach"),
            }
        }
    }

    fn secret_recipe() -> String {
        String::from("butter and jam")
    }
}

fn main() {
    let mut meal = kitchen::Breakfast::summer("호밀");

    meal.toast = String::from("밀");
    meal.fruit = String::from("딸기");

    let recipe = kitchen::secret_recipe();

    println!("토스트: {}", meal.toast);
}

