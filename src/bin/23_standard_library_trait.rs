fn main() {
    // comparison traits
    // PartialEq and Eq
    // this traits implements 2 methods, one required eq method and an option ne method
    // when we use the == and != operator, the call the methods under the hood
    // PartialEq can be implemented for different types, Eq can not be
    // In practise it is common to derive these traits, but uncommon to define them

    #[allow(dead_code)]
    struct Key {
        id: u32,
        metadata: Option<String>
    }

    impl Key {
        fn new(id: u32) -> Self  {
            Self {
                id,
                metadata: None
            }
        }
    }

    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    // for a type to implement Eq it has to inherit the PartialEq trait
    impl Eq for Key {

    }


    let my_key = Key::new(23);
    let other_key = Key::new(345);

    let is_equal = my_key == other_key; // this translates to &my_key.eq(&other_key)
    println!("{is_equal}");

    // PartialOrd and Ord
    // this traits defines partial ordering with a partial_cmp method
    // it is used to implement the >, < >=, <= operators

    use std::cmp::Ordering;

    #[allow(dead_code)]
    #[derive(Eq, PartialEq)]
    struct Citation {
        author: String,
        year: u32,
    }

    impl PartialOrd for Citation {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.author.partial_cmp(&other.author) {
                Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
                author_ord => author_ord,
            }
        }
    }

    let my_citation = Citation { author: String::from("Brian"), year: 2025 };
    let enstein_citation = Citation { author: String::from("Enstein"), year: 1940 };

    let is_newer = enstein_citation > my_citation;
    println!("Is Newer: {is_newer}");

    // Ord is implemented with the cmp method
    // this returns a Ordering type, unlike the PartialOrd which returns the Option<Ordering>
    // When comparing references to things, the references are not compared, but the actual things being pointed to



}