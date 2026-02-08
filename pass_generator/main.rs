use rand::{rng, Rng};
fn main(){
    let symbols="qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM!@#$%^*+01234567890";
    println!("Enter lenght of pass");
    let mut lenght = String::new();
    std::io::stdin().read_line(&mut lenght).unwrap();
    let lenght : u8 = lenght.trim().parse().unwrap();
    let mut password = String::new();
    for _ in 0..lenght {
        let random = rng().gen_range(0..symbols.len());
        let ch=symbols.chars().nth(random).unwrap();
        password.push(ch);
    }
    println!("Password: {}", password);

}