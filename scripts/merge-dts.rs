#!/usr/bin/env rust-script


// I don't know why this is needed, but using "cat" to merge the files didn't work

fn main() {
    let dir = std::env::current_dir().unwrap();
    let dir = dir.to_str().unwrap();
    let filenames = [
        format!("{}/types/additional.d.ts", dir),
        format!("{}/types/generated.d.ts", dir),
    ];

    let mut output = String::new();
    for filename in filenames {
        let contents = std::fs::read_to_string(filename).unwrap();
        output.push_str(&contents);
    }

    std::fs::write(format!("{}/index.d.ts", dir), output).unwrap();
}
