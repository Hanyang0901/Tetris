use std::collections::linked_list::Iter;
use std::iter::Zip;

pub fn show(m: Zip<Iter<String>, Iter<String>>) {
    for (active_line, stable_line) in m {
        active_line
            .chars()
            .enumerate()
            .for_each(|(index, character)| {
                if character == '@' {
                    print!("{}", "@");
                } else {
                    print!("{}", stable_line.chars().nth(index).unwrap());
                };
            });

        println!();
    }
}
