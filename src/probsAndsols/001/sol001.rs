use std::io;

fn main() {
    let mut words_count = String::new();

    io::stdin()
        .read_line(&mut words_count)
        .expect("Failed to read the line");

    let words_count_int: u32 = words_count.trim().parse().expect("Please type a number!");
    let mut words_array = Vec::new();
    let mut counter: u32 = 1;
    while counter <= words_count_int {
        let mut user_string = String::new();
        io::stdin()
            .read_line(&mut user_string)
            .expect("Failed to read the line");
        words_array.push(user_string);
        counter += 1;
    }

    for word in words_array{
        process_word(word);
    }
}

fn process_word(word:String){
    if word.len() > 10 {
        print!("something happens here!");
    }else{
        print!("{}",word);
    }
}
