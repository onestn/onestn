struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are {} and {} equal? {}",
            self.number, other_number, self.number == other_number);
    }
}

fn main() {
    let item = Item {
        number: 8,
    };

    let reference_item = &item; // &Item Type
    let reference_item_two = &reference_item; // &&Item Type

    item.compare_number(8);
    reference_item.compare_number(8);
    reference_item_two.compare_number(8);
}
