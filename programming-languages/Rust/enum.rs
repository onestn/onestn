fn main() {
    // Enum
    #[allow(dead_code)]
    #[derive(Debug)] // derive Debug trait, to print the enumk
    
    enum Languages {
        Python,
        Rust,
        Javascript,
        Go,
    }

    impl Languages {
        fn echo(&self) {
            println!("{:?}", &self)
        }
    }
    
    let language = Languages::Rust;
    language.echo();

    // match
    match language {
        Languages::Python => println!("I love Python"),
        Languages::Go => println!("I love Go"),
        Languages::Javascript => println!("I love Javascript"),
        _ => println!("I love Rust"),
    }
}
