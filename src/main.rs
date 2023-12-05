mod day1;

fn main() {
    dbg!(day1::p1(&day1::EXAMPLE));
    let in_day1 = include_str!("day1.txt");
    dbg!(day1::p1(&in_day1));
    dbg!(day1::p2(&day1::EXAMPLE2));
    dbg!(day1::p2(&in_day1));
}
