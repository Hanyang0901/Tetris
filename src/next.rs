pub fn next_piece(next: &mut Vec<&str>) {
    println!("{}", " ﹋﹋﹋﹋﹋﹋﹋");
    println!("{}", "NEXT");
    println!();
    next.iter().for_each(|line| println!("{}", line));
    println!();
    println!();
}
