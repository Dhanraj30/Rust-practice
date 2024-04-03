// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavors {
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavors,
    fluid_oz: f64,
}

fn print_drink(drinks: Drink) {
    match drinks.flavor {
        Flavors::Sweet => println!("sweet"),
        Flavors::Fruity => println!("fruity"),
    }
    println!("oz: {:?}", drinks.fluid_oz);
}
fn main() {
    let sweet = Drink {
        flavor: Flavors::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);
    let fruity = Drink {
        flavor: Flavors::Fruity,
        fluid_oz: 6.0,
    };
    print_drink(fruity);
}
