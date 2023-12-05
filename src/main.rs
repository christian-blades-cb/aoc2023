mod day1;
mod day4;

fn main() {
    // dbg!(day1::p1(&day1::EXAMPLE));
    // let in_day1 = include_str!("day1.txt");
    // dbg!(day1::p1(&in_day1));
    // dbg!(day1::p2(&day1::EXAMPLE2));
    // dbg!(day1::p2(&in_day1));

    dbg!(day4::p1(&day4::EXAMPLE1));
    let in_day4 = include_str!("day4.txt");
    dbg!(day4::p1(&in_day4));
    dbg!(day4::p2(&day4::EXAMPLE1));
    dbg!(day4::p2(&in_day4));
}
