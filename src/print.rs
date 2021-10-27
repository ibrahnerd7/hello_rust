pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    //Basic formatting
    println!("{} is from {}", "Brad", "Mass");

    //Positional args
    println!(
        "{0} if rom {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    //Named args
    println!("{name} likes to play {activity}",name="John", activity="baseball");

    //Placeholder traits
    println!("Binary:{:b} Hex:{:x} Octal: {:o}",10,10,10);

    //Placeholder for debug traits
    println!("{:?}",(12, true,"hello"));

    //Basic math
    println!("10 + 10 ={}",10+10);
}
