use std::collections::HashMap;
use std::io;

/**
script to enable developers to setup their basic java dev environment
*/
fn main() {
    println!("welcome to java environment setup tool.");
    print!("\x1B[2J\x1B[1;1H");

    println!("Showing java versions");
    let mapjava = list_java_versions();
    for (i,value) in mapjava.iter().enumerate() {
        println!("{}. {}-{}",i+1,value.0,value.1);
    }
    let mut userInput = String::new();
    println!("Select a value");
    io::stdin().read_line(&mut userInput).expect("failed to take input from user.");
    let trimInput = userInput.trim();
    let downloadLink = String::new();
    match trimInput.parse::<u32>() {
        Ok(i) => {
            // mapjava.
        },
        Err(e) => println!("Error passing user input {}",trimInput)
    }


}

fn list_java_versions()->HashMap<String,String>{
    let mut javaVersion:HashMap<String,String> = HashMap::new();
    javaVersion.insert(String::from("Java 8"),String::from("download link"));
    javaVersion.insert(String::from("Java 11"),String::from("download link"));
    javaVersion.insert(String::from("Java 17"),String::from("download link"));
    javaVersion
}

