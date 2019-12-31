mod security;

fn main() {
    let mut possible = 0;
    for pass in 197487..=673251 {
        if security::possible_password(&pass.to_string()) {
            println!("{}", pass);
            possible += 1;
        }
    }

    println!("possible passwords: {}", possible);
}
