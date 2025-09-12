// Problem 1: Compile the code by adding the definition for the next method

struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        /* Add code here */
        let current = self.current;
        if current < self.max {
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

pub fn custom_iterator() {
    let mut counter = Counter::new(3);
    assert!(matches!(counter.next(), Some(0)));
    assert!(matches!(counter.next(), Some(1)));
    assert!(matches!(counter.next(), Some(2)));
    assert!(matches!(counter.next(), None));
}

struct Person {
    name: String,
    age: u32,
    occupation: String,
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // let mut vec = Vec::new();
        // vec.push(self.name);
        // vec.push(self.age.to_string());
        // vec.push(self.occupation);
        // vec.into_iter()
        vec![self.name, self.age.to_string(), self.occupation].into_iter()
    }
}

pub fn custom_into_iterator() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        occupation: "Software Engineer".to_string(),
    };

    let mut person_iterator = person.into_iter();

    while let Some(property) = person_iterator.next() {
        println!("{}", property);
    }
}

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8; // this needs to be fixed 
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        /* The function needs to be completed */
        vec![self.r, self.g, self.b].into_iter()
    }
}

pub fn custom_pixel_into_iterator() {
    let pixel = Pixel {
        r: 25,
        g: 55,
        b: 76,
    };
    let mut pixel_iterator = pixel.into_iter();

    while let Some(value) = pixel_iterator.next() {
        println!("{}", value);
    }
}
