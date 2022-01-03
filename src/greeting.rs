// Greet new user.
// Calculate user's generation by knowing their age.
// Announce what generation the user is from.

pub fn run() {
    // Variables.
    let user_name = String::from("Christopher");
    let user_age: i32 = 28;
    let mut current_year: i32 = 2022;
    let birth_year: i32 = current_year - user_age;
    let generation = ["are a Baby Boomer!.", "are a Gen X baby!", "are a Millennial.", "are a Zennial."];

    // Greets the user and tells them it will calculate.

    println!("Hello, {}, I see you're {} years old.", user_name, user_age);

    println!("{}, let me calculate what generation you are from.", user_name);

    // Getting user's generation by subtracting current year from user's age.
    if birth_year <= 1964 && birth_year >= 1946 {
        println!("{}, you {}", user_name, generation[0]);

    } else if birth_year <= 1980 && birth_year >= 1965 {
        println!("{}, you {}", user_name, generation[1]);

    } else if birth_year <= 1996 && birth_year >= 1981 {
        println!("{}, you {}", user_name, generation[2]);

    } else if birth_year <= 2012 && birth_year >= 1997 {
        println!("{}, you {}", user_name, generation[3]);
    } else {
        println!("I can't calculate that far it seems.");
    }

}