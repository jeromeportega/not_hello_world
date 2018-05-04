fn main() {
    show_alphabet();
    count();
    println!("The State Of Illinois");
}

fn count() {
    for x in 1..101 {
        println!("{}", x); // x: i32
    }
}

fn show_alphabet() {
    const ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    for letter in ALPHABET.iter() {
        println!("{}", letter);
    }
}
