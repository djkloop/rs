#[derive(Debug)]
enum Flavor {
    Sweet,
    Sour,
    Salty,
}

#[derive(Debug)]
struct Drink {
    flavor: Flavor,
    fluid_oz: f32,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("Sweet"),
        Flavor::Sour => println!("Sour"),
        Flavor::Salty => println!("Salty"),
    }
    println!("{}oz", drink.fluid_oz);
}

pub fn struct_main() {
        let drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 12.0,
    };
    
    let drink2 = Drink {
        flavor: Flavor::Sour,
        fluid_oz: 10.0,
    };
    
    let drink3 = Drink {
        flavor: Flavor::Salty,
        fluid_oz: 15.0,
    };  

    println!("drink1: {:?}", drink);
    print_drink(drink);
    println!("drink2: {:?}", drink2);
    print_drink(drink2);
    println!("drink3: {:?}", drink3);
    print_drink(drink3);
}

