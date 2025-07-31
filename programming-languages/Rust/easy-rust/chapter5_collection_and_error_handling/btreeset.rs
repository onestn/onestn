use std::collections::BTreeSet;

fn main() {
    let many_numbers = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
    ];

    let mut number_btreeset = BTreeSet::new();

    for number in many_numbers {
        number_btreeset.insert(number);
    }

    for entry in number_btreeset {
        print!("{entry} -> ");
    }
}
