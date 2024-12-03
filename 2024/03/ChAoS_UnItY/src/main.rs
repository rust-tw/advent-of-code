use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::is_digit,
    IResult,
};

fn main() {
    let content = include_str!("../Day03.txt");
    part1(content);
    part2(content);
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    let (input, num) = take_while_m_n(0, 3, |c| is_digit(c as u8))(input)?;

    Ok((input, num.parse::<u32>().unwrap()))
}

fn parse_mul(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("mul(")(input)?;
    let (input, lhs) = parse_num(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, rhs) = parse_num(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, lhs * rhs))
}

fn part1(mut content: &str) {
    let mut answer = 0;

    while !content.is_empty() {
        if let Ok((remain, multiplied)) = parse_mul(content) {
            answer += multiplied;
            content = remain;
        } else {
            content = &content[1..];
        }
    }

    println!("Part 1: {answer}");
}

fn part2(mut content: &str) {
    const DO_OP: &'static str = "do()";
    const DONT_OP: &'static str = "don't()";

    fn parse_conditional(input: &str) -> IResult<&str, &str> {
        alt((tag(DO_OP), tag(DONT_OP)))(input)
    }

    let mut answer = 0;
    let mut disabled = false;

    while !content.is_empty() {
        if !disabled {
            if let Ok((remain, multiplied)) = parse_mul(content) {
                answer += multiplied;
                content = remain;
                continue;
            }
        }

        if let Ok((remain, cond_op)) = parse_conditional(content) {
            match cond_op {
                DO_OP => disabled = false,
                DONT_OP => disabled = true,
                _ => unreachable!(),
            }

            content = remain;
        } else {
            content = &content[1..];
        }
    }

    println!("Part 2: {answer}");
}
