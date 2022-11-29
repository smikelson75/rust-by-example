fn main() {
    runloop();
    whileloop();
    forloop();
    forlooparray();
}

fn runloop() {
    let mut counter = 0;

    // rust infinite loop
    loop {
        counter += 1;

        if counter == 5 {
            // continue stops execution of the loop and returns
            // to the beginning of the loop
            continue;            
        }
        
        println!("{}", counter);

        if counter == 10 {
            // break stops execution of the loop and exits
            // the loop entirely;
            break;
        }
    }
}

fn whileloop() {
    let mut counter = 0;

    // while loop
    while counter <= 10 {
        counter += 1;
        println!("{}", counter);
    }
}

fn forloop() {
    // exclusive range 1 to 11 (11 not included)
    for index in 1..11 {
        println!("{}", index);
    }

    // inclusive range 1 to 10 (10 is included)
    for index in 1..=10 {
        println!("{}", index);
    }
}

fn forlooparray() {
    let duck_aircraft = [ "Boeing 737" , "Boeing 767", "Boeing 787", "Airbus 319", "Airbus 320"];

    for aircraft in duck_aircraft.iter() {
        println!("{}", aircraft);
    }
}