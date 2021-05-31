use std::collections::VecDeque;

fn main() {
    let mut alphabet = ('a'..='z').collect::<VecDeque<char>>();
    let cyphertext = "v jvfu gb unir cbgngb fnynq sbe qvaare gbqnl";

    let cleartext = loop {
        let cleartext = cyphertext.chars()
            .map(|c| {
                if c == ' ' {
                    return c;
                }
                
                let index = (c as u8 - b'a') as usize;

                alphabet[index]
            })
            .collect::<String>();
        
        if cleartext.contains("dinner") {
            break cleartext;
        }

        alphabet.rotate_left(1);

        if alphabet[0] == 'a' {
            return;
        }
    };

    println!("{}", cleartext);
}
