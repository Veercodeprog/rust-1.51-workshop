struct City {
    description: String,
    residents: u64,
is_coastal: bool
    // 👉 TODO add a field here for is_coastal: bool
    //
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
}

fn new_city(description:String, residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!(" descripiton: {} ,{} residents", description, residents),
            residents,
is_coastal
        }
    } else {
  City {
            description: format!("a *non-coastal* city of approximately {} residents", residents),
            residents,
is_coastal
        }
        // panic!(
        //     "👉 TODO return a `City` described as a *non-coastal* city of approximately {} residents"
        // );
    }
}

fn main() {
    let rustville: City = new_city("very big non-coastal city".to_string(), 1_000_000, true);

    println!("This city can be described as: 👉 TODO print rustville's `description` here.");

    if rustville.is_coastal {
        println!("It is a coastal city.{}", rustville.description);
    } else {
        println!("It is not a coastal city.{}", rustville.description);
    }
}
