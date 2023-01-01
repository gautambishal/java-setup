use std::collections::HashMap;
/**
script to enable developers to setup their basic java dev environment
*/
fn main() {
    println!("welcome to java environment setup tool.");
    print!("\x1B[2J\x1B[1;1H");

    println!("showing java versions");
    for listJavaVersion in listJavaVersions() {

    }

}

fn listJavaVersions()->HashMap<String,String>{
    let mut javaVersion:HashMap<String,String> = HashMap::new();
    javaVersion.insert(String::from("Java 8"),String::from("download link"));
    javaVersion.insert(String::from("Java 11"),String::from("download link"));
    javaVersion.insert(String::from("Java 17"),String::from("download link"));
    javaVersion
}

