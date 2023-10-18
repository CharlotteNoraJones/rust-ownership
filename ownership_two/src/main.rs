fn main() {
    let s1 = gives_ownership();

    let s2: String = String::from("hello");

    let s3: String = takes_and_gives_back(s2);

    let s4: String = String::from("hello");

    let (s5, len) = calculate_length(s4);

    println!("The length of {} is {}.", s5, len);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
