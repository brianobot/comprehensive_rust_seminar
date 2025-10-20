#![allow(unused)]

#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            laps: Vec::new(),
        }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec")
        }
    }

    // exclusive ownership of self
    fn finish(self) {
        let total: i32 = self.laps.iter().sum(); // i think it's weird that the compiler cannot infer type of the output of sum 
        println!(
            "Race {} is finished, Total lap time: {} sec",
            self.name, total
        );
    }
}

// Methods can also be called on Enums too

enum MotivationState {
    Motivated,
    Unmotivated,
    Unbothered,
}

impl MotivationState {
    fn explain(&self) {
        match self {
            Self::Motivated => println!("I am very Motivated Right now"),
            Self::Unmotivated => println!("I am very Unmotivated Right now"),
            Self::Unbothered => println!("I am Unbothered Right now"),
        }
    }

    fn change_mood(self) -> Self {
        Self::Motivated
    }
}

fn main() {
    let mut car_race_1 = CarRace::new("Ibom Grand Prix");
    car_race_1.add_lap(12);
    car_race_1.add_lap(15);
    println!("Race Car 1: {:?}", car_race_1);
    car_race_1.print_laps();
    car_race_1.finish();

    let mood = MotivationState::Motivated;
    mood.explain();

    // NOTES:
    /*
        - the self argument specify the receiver, the objects the methods is called on and acts on
        - methods can be called as associated functions by passing the object in as the first argument
        - self is an abbreviated form of the argument self: Self
    */

    let second_mood = MotivationState::Unbothered;
    MotivationState::explain(&second_mood);

    let bad_mood = MotivationState::Unmotivated;
    let good_mood = bad_mood.change_mood();

    good_mood.explain();
}
