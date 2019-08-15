extern crate console;
extern crate enigo;

use std::borrow::{Borrow, BorrowMut};
use std::collections::LinkedList;
use std::time;

use console::Term;

use rand::Rng;
use Tetris::next::next_piece;
use Tetris::view::show;

fn main() {
    //The representation of the square and its rotation

    let l = vec!["1     @      1", "1     @      1", "1     @@@    1"];
    let l_r = vec!["1     @      1", "1     @      1", "1   @@@      1"];
    let l_i = vec!["1     @@@    1", "1     @      1", "1     @      1"];
    let l_l = vec!["1   @@@      1", "1     @      1", "1     @      1"];

    let i = vec!["1     @      1", "1     @      1", "1     @      1"];
    let i_r = vec!["1    @@@@    1"];
    let i_i = vec!["1     @      1", "1     @      1", "1     @      1"];
    let i_l = vec!["1    @@@@    1"];

    let one = vec!["1    @@@@    1"];
    let one_r = vec!["1     @      1", "1     @      1", "1     @      1"];
    let one_i = vec!["1    @@@@    1"];
    let one_l = vec!["1     @      1", "1     @      1", "1     @      1"];

    let two = vec!["1     @@@    1", "1     @@@    1"];
    let two_r = vec!["1     @@     1", "1     @@     1", "1     @@     1"];
    let two_i = vec!["1     @@@    1", "1     @@@    1"];
    let two_l = vec!["1     @@     1", "1     @@     1", "1     @@     1"];

    let z = vec!["1   @@@@     1", "1     @      1", "1   @@@@     1"];
    let z_r = vec!["1  @     @   1", "1  @  @  @   1", "1  @     @   1"];
    let z_i = vec!["1   @@@@     1", "1     @      1", "1   @@@@     1"];
    let z_l = vec!["1   @     @   1", "1   @  @  @   1", "1   @     @   1"];

    //Initialization of the data, including the score, the probability of reward, the active part, the fixed part
    let mut score = 0;
    let mut reward_probability = 0;
    let mut active = LinkedList::new();
    let mut stable = LinkedList::new();

    let mut next = vec!["1    @@@@    1"];

    let init_row = "1            1".to_string();

    let mut piece = "one".to_string();
    let mut next_sign = "one".to_string();
    let mut direction = "s".to_string();

    let mut x = 0;
    let mut y = 0;

    while active.len() < 10 {
        active.push_back(init_row.clone());
    }
    while stable.len() < 10 {
        stable.push_back(init_row.clone());
    }

    let mut game_over = false;

    let term = Term::stdout();

    //main run in the game
    while !game_over {
        //Stable processing the line that should be erased

        //If there is a full line in the stable, it will be eliminated.
        for _i in 0..10 {
            let line = stable.pop_back().unwrap();

            if line.contains(' ') {
                //line is not enough
                if line.contains('@')
                    && rand::thread_rng().gen_range(reward_probability, 100) >= 98
                    && reward_probability >= 0
                {
                    //Reward: Directly eliminate one line
                    score += 5;
                    reward_probability += 1;
                } else {
                    stable.push_front(line);
                }
            } else {
                score += 20;
                reward_probability += 5;
            }
        }

        while stable.len() < 10 {
            stable.push_front(init_row.clone());
        }
        //Handling active , including falling and falling down the bottom
        // and synthesizing the new stable and adding new Tetris

        if active
            .iter()
            .any(|line| line.to_string() != init_row.to_string())
        {

            let (last_piece_line_index,last_piece_line)=//获取方块最后一行及最后一行index
                active.iter().enumerate()
                    .filter(|(_index,line)|{**line!=init_row})
                    .max_by_key(|&(a,_b)|{a}).unwrap();
            //Determine if you can continue to fall

            if active.back().unwrap() == &init_row
                && stable
                    .iter()
                    .nth(last_piece_line_index + 1)
                    .unwrap()
                    .chars()
                    .enumerate()
                    .filter(|(_index, character)| character == '@'.borrow())
                    .all(|(index, _character)| last_piece_line.chars().nth(index).unwrap() == ' ')
            {
                //Drop a line

                active.borrow_mut().pop_back();
                active.push_front(init_row.clone());
                y += 1;
            //                println!("{}","")
            } else {
                //Unable to continue falling

                //Combine active and stable into a new stable, clear active

                active.iter().for_each(|active_line| {
                    let mut new_line = String::new();

                    for (index, active_character) in active_line.chars().enumerate() {
                        if active_character == '@' {
                            new_line.push('@');
                        } else {
                            new_line.push(stable.front().unwrap().chars().nth(index).unwrap());
                        };
                    }
                    stable.push_back(new_line);
                    stable.pop_front();
                });

                active.clear();
                while active.len() < 10 {
                    active.push_back(init_row.clone());
                }
            }
        } else {
            //no block
            //add a block
            //            println!("{}","+");

            active.clear();

            next.iter()
                .for_each(|line| active.push_back(line.to_string()));
            direction = 's'.to_string();
            piece = next_sign.clone();
            x = 0;
            y = 0;
            next = match rand::thread_rng().gen_range(1, 6) {
                1 => {
                    next_sign = "one".to_string();
                    one.clone()
                }
                2 => {
                    next_sign = "z".to_string();
                    z.clone()
                }
                3 => {
                    next_sign = "l".to_string();
                    l.clone()
                }
                4 => {
                    next_sign = "i".to_string();
                    i.clone()
                }
                5 => {
                    next_sign = "two".to_string();
                    two.clone()
                }
                _ => {
                    next_sign = "i".to_string();
                    i.clone()
                }
            };
            while active.len() < 10 {
                active.push_back(init_row.clone());
            }
        }

        if stable.front().unwrap().contains('@') {
            game_over = true;
        }

        std::thread::sleep(time::Duration::from_millis(200));
        //get the keyboard act

        use enigo::*;
        let mut enigo = Enigo::new();
        enigo.key_click(Key::Return);

        use std::io::{stdin, stdout, Write};
        let mut key = String::new();

        let _ = stdout().flush();
        stdin().read_line(key.borrow_mut()).expect("");
        //handle the key act

        match key.to_lowercase().chars().nth(0).unwrap() {
            'a' => {
                //left move

                if active.iter().all(|line| line.starts_with("1 ")) {
                    x -= 1;
                    for line in active.iter_mut() {
                        line.replace_range(13.., " 1");
                        line.replace_range(..2, "1");
                    }
                }
            }
            'd' => {
                //right move

                if active.iter().all(|line| line.ends_with(" 1")) {
                    x += 1;
                    for line in active.iter_mut() {
                        line.replace_range(12.., "1");
                        line.replace_range(..1, "1 ");
                    }
                }
            }
            's' => {
                //Accelerate to drop

                if active
                    .iter()
                    .any(|line| line.to_string() != init_row.to_string())
                {
                    //there are still have block

                    let (last_piece_line_index,last_piece_line)=    //
                        active.iter().enumerate()
                            .filter(|(_index,line)|{**line!=init_row})
                            .max_by_key(|&(a,_b)|{a}).unwrap();
                    //determine if it can continue to drop

                    if active.back().unwrap() == &init_row
                        && stable
                            .iter()
                            .nth(last_piece_line_index + 1)
                            .unwrap()
                            .chars()
                            .enumerate()
                            .filter(|(_index, character)| character == '@'.borrow())
                            .all(|(index, _character)| {
                                last_piece_line.chars().nth(index).unwrap() == ' '
                            })
                    {
                        //fall a line

                        active.borrow_mut().pop_back();
                        active.push_front(init_row.clone());

                        y += 1;

                        //                println!("{}","")
                    }

                    let (last_piece_line_index,last_piece_line)=//Get the last and last line of the index
                        active.iter().enumerate()
                            .filter(|(_index,line)|{**line!=init_row})
                            .max_by_key(|&(a,_b)|{a}).unwrap();

                    if active.back().unwrap() == &init_row
                        && stable
                            .iter()
                            .nth(last_piece_line_index + 1)
                            .unwrap()
                            .chars()
                            .enumerate()
                            .filter(|(_index, character)| character == '@'.borrow())
                            .all(|(index, _character)| {
                                last_piece_line.chars().nth(index).unwrap() == ' '
                            })
                    {

                        active.borrow_mut().pop_back();
                        active.push_front(init_row.clone());

                        y += 1;

                        //                println!("{}","")
                    }
                }
            }
            'w' => {
                //rotate
                let a = match piece.as_str() {
                    "one" => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            one_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            one_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            one_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            one.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            one.clone()
                        }
                    },
                    "z" => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            z_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            z_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            z_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            z.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            z.clone()
                        }
                    },
                    "l" => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            l_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            l_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            l_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            l.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            l.clone()
                        }
                    },
                    "i" => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            i_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            i_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            i_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            i.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            i.clone()
                        }
                    },
                    "two" => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            two_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            two_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            two_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            two.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            two.clone()
                        }
                    },
                    _ => match direction.clone().as_str() {
                        "s" => {
                            direction = "r".to_string();
                            i_r.clone()
                        }
                        "r" => {
                            direction = "i".to_string();
                            i_i.clone()
                        }
                        "i" => {
                            direction = "l".to_string();
                            i_l.clone()
                        }
                        "l" => {
                            direction = "s".to_string();
                            i.clone()
                        }
                        _ => {
                            direction = "s".to_string();
                            i.clone()
                        }
                    },
                };

                active.clear();
                a.iter().for_each(|line| active.push_back(line.to_string()));

                while active.len() < 10 {
                    active.push_back(init_row.clone());
                }
                println!("{}", y);

                for _i in 0..y {
                    active.borrow_mut().pop_back();
                    active.push_front(init_row.clone());
                }

                if x > 0 {
                    for _i in 0..x {
                        if active.iter().all(|line| line.ends_with(" 1")) {
                            for line in active.iter_mut() {
                                line.replace_range(12.., "1");
                                line.replace_range(..1, "1 ");
                            }
                        }
                    }
                } else {
                    for _i in x..0 {
                        if active.iter().all(|line| line.starts_with("1 ")) {
                            for line in active.iter_mut() {
                                line.replace_range(13.., " 1");
                                line.replace_range(..2, "1");
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        let m = active.iter().zip(stable.iter());

        //output the game
        show(m);

        next_piece(&mut next);

        println!("SCORE:{}", score);
        println!("Reward Probability:{}", reward_probability);

        std::thread::sleep(time::Duration::from_millis(800));

        term.clear_screen().expect(
            "\
             ",
        );
    }

    println!("{}", "    THE GAME IS OVER");
    println!("  YOUR  SCORE  IS  {}  !", score);
    std::thread::sleep(time::Duration::from_secs(5));
}
