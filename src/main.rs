use std::io;

fn main() {

    // UPER

    // UNDERSTAND

        // Bulls and Cows Flow
        //----------------------

        // 1) user1 types in SECRET_WORD

        // 2) user2 will try and GUESS (length is given)
            // if GUESS has letters in SECRET_WORD but NOT in
            // - correct slot COW count is incremented.
            // if GUESS has letters in SECRET_WORD and ARE in
            // -  correct slot BULL count is incremented
            // otherwise neither is incremented

        // 3) if GUESS is SECRET_WORD game is over
    
    // PREPARE

        // How is program capturing user input?
            // using io::stdin() from namespace std::io

            // How does io::stdin() work?
                // io::stdin() has method called read_line()
                // that takes refrence to given String
                // refrence must also match "mutablity" of said string
                // in this case that would be using mut keyword
                // an except method must also be attached to gracfuly crash

        // How is program going to check for "bulls" & "cows"?

            // How is it iterating through given string?
                // program will convert string to byte array
                // done using s.as_bytes()
                // from here .iter() can be called on arr to 
                // - loop through each element
                // also .enumerate() would be used to return 
                // an index and pointer to current item

            // How is it checking for COWS vs BULLS vs nothing?
                // during runtime an array of bytes will be created
                // - from user1's inital input
                // while checking for BULLS iterate current GUESS by
                // - comparing nth element in GUESS to nth element in SECRET_WORD
                // if elements match increment bumber of bulls 
                // than check number of COWS by iterating through GUESS
                // - if the letter in SECRET_WORD than increment COW
                // - only if it is NOT also a BULL

            // Potential issues?
                // inputs of invalid string/int
                // GUESS is string that does not match length
                // - of SECRET_WORD

    // EXECUTE

    // initalize bull and cow count
    let mut bulls = 0;
    let mut cows = 0;

    // get secret word
    let secret = get_input();
    
    // create secret byte arr
    let secret_arr = secret.as_bytes();

    // main loop
    loop {
        // get user guess
        let guess = get_input();
        
        let guess_arr = guess.as_bytes();

        // let mut guess_v = Vec::new();
        // for i in 0..guess_arr.len() {
        //     guess_v.push(guess_arr[i]);
        // }

        // check if guess is the same size as secret
        if guess_arr.len() > secret_arr.len() || guess_arr.len() < secret_arr.len() {
            println!("GUESS NEEDS TO BE THE SAME SIZE AS SECRET");
            continue;
        }

        // BULLS CHECK
        // iterate through secret word
        for i in 0..secret_arr.len() {
            if guess_arr[i] == secret_arr[i] {bulls += 1;}
        }

        // COWS CHECK
        for i in 0..secret_arr.len() {
            for j in 0..guess_arr.len() {
                if guess_arr[i] != secret_arr[i] && guess_arr[j] == secret_arr[i] {
                    cows += 1;
                }
            }
        }

        if bulls == secret_arr.len() {
            println!("YOU WIN");
            break;
        }

        println!("SECRET_WORD - {}GUESS - {}", &secret, &guess);
        println!("BULLS - {} COWS - {}", bulls, cows);

        bulls = 0;
        cows = 0;
    }
}

fn get_input() -> String {

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("CRASSSSSSSSSHHHINGGGGG!");

    make_lowercase(&mut s)
}

fn make_lowercase(s: &mut str) -> String { s.to_lowercase() }

fn is_valid_guess(a1: &[char]) {}
