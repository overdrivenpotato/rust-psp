psp::module!("rust_std_hello_world", 1, 1);

fn main() {
    psp::enable_home_button();

    let yeet = String::from("Yeeteth! I am inside a String!");
    psp::dprintln!("{}", yeet);

    let people = vec!["sajattack", "overdrivenpotato", "iridescence"];
    for person in people {
        let x = format!(
            "Hello, {}! I'm coming to you live from the standard library!\n",
            person
        );
        psp::dprint!("{}", x);
    }
}
