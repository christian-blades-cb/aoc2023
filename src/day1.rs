use regex::Regex;

pub static EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

pub static EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

pub fn p1(raw: &str) -> usize {
    raw.lines()
        .map(|l| {
            let mut digits = l.chars().filter(|x| x.is_digit(10));
            let first = digits.next().unwrap();
            let mut last = first;
            for x in digits {
                last = x;
            }
            let n = format!("{}{}", first, last).parse::<usize>().unwrap();
            // dbg!(&n);
            n
        })
        .sum()
}

pub fn p2(raw: &str) -> usize {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    raw.lines()
        .map(|ln| {
            let first = {
                let mut digits = re.captures_iter(ln).map(|cap| cap[0].to_string());
                digits.next().unwrap()
            };
            let length = ln.len();
            let last = (0..length)
                .skip(1)
                .find_map(|i| {
                    let snippet = &ln[length - i..length];
                    re.find(snippet).map(|x| x.as_str().to_owned())
                })
                .unwrap_or_else(|| first.clone());
            let n = format!("{}{}", digit_to_usize(&first), digit_to_usize(&last))
                .parse::<usize>()
                .unwrap();
            dbg!(&n);
            n
        })
        .sum()
}

fn digit_to_usize(raw: &str) -> usize {
    if let Ok(i) = raw.parse::<usize>() {
        return i;
    }
    match raw {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("not a digit!"),
    }
}
