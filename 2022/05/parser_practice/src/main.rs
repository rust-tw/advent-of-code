use std::io::Read;

#[derive(Copy, Clone, Debug)]
struct Item(char);

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<Item>>);

mod parse_regex {
    use itertools::Itertools;
    use once_cell::sync::Lazy;
    use regex::Regex;

    use super::{Item, Stacks};
    static ITEM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[(.)\]").unwrap());

    static DIGITS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    fn parse_item(s: &str) -> anyhow::Result<Item> {
        let item_c = ITEM_REGEX
            .captures(s)
            .and_then(|captures| captures.get(1).map(|m| m.as_str()))
            .and_then(|s| s.chars().exactly_one().ok())
            .ok_or_else(|| anyhow::anyhow!("bad item: {}", s))?;
        Ok(Item(item_c))
    }

    fn parse_stacks(s: Vec<&str>) -> anyhow::Result<Stacks> {
        const SECT_LEN: usize = 4;
        let mut rows_rev = s.into_iter().rev();
        let stacks_cnt = rows_rev
            .next()
            .ok_or_else(|| anyhow::anyhow!("Bad input"))?
            .split_ascii_whitespace()
            .count();
        let mut st = vec![vec![]; stacks_cnt + 1];
        for line in rows_rev {
            for i in 0..stacks_cnt {
                let beg = i * SECT_LEN;
                let s = &line[beg..beg + SECT_LEN - 1].trim();
                if !s.is_empty() {
                    st[i + 1].push(parse_item(s)?);
                }
            }
        }
        Ok(Stacks(st))
    }

    fn parse_moves(s: &str) -> anyhow::Result<(usize, usize, usize)> {
        let bail = || anyhow::anyhow!("Can't extract moves from {s}");
        let (x, y, z) = DIGITS_REGEX
            .find_iter(s)
            .map(|s| s.as_str().parse::<usize>().map_err(|_| bail()))
            .collect_tuple()
            .ok_or_else(&bail)?;
        let x = x?;
        let y = y?;
        let z = z?;
        Ok((x, y, z))
    }

    #[allow(unused)]
    pub(crate) fn parse(s: &str) -> anyhow::Result<(Stacks, Vec<(usize, usize, usize)>)> {
        let mut lines = s.lines();
        let first_section = lines.by_ref().take_while(|s| !s.is_empty()).collect_vec();
        let stacks = parse_stacks(first_section)?;
        let moves: Vec<_> = lines.map(parse_moves).try_collect()?;
        Ok((stacks, moves))
    }
}

mod parse_nom {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char, newline, satisfy, u8 as parse_u8},
        combinator::{all_consuming, map, value},
        multi::{many0_count, separated_list1},
        sequence::{delimited, preceded, separated_pair, tuple},
        IResult,
    };

    use super::{Item, Stacks};

    fn parse_item(s: &str) -> IResult<&str, Item> {
        map(delimited(char('['), anychar, char(']')), Item)(s)
    }

    fn parse_item_optional(s: &str) -> IResult<&str, Option<Item>> {
        alt((value(None, tag("   ")), map(parse_item, Some)))(s)
    }

    fn parse_stack(s: &str) -> IResult<&str, Stacks> {
        fn parse_row(s: &str) -> IResult<&str, Vec<Option<Item>>> {
            separated_list1(char(' '), parse_item_optional)(s)
        }

        // returns the total number of stacks
        fn parse_bottom_row(s: &str) -> IResult<&str, usize> {
            fn parse_stack_tag(s: &str) -> IResult<&str, ()> {
                delimited(
                    char(' '),
                    value((), satisfy(|c| c.is_ascii_digit())), // the tag is unused
                    char(' '),
                )(s)
            }
            map(separated_list1(char(' '), parse_stack_tag), |v| v.len())(s)
        }

        let (input, (rows, len)) = separated_pair(
            separated_list1(newline, parse_row),
            newline,
            parse_bottom_row,
        )(s)?;
        let mut items = vec![vec![]; len + 1];
        for item_list in rows.into_iter().rev() {
            item_list.into_iter().enumerate().for_each(|(i, item)| {
                if let Some(item) = item {
                    items[i + 1].push(item);
                }
            })
        }
        Ok((input, Stacks(items)))
    }

    fn parse_moves(s: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
        fn parse_row(s: &str) -> IResult<&str, (usize, usize, usize)> {
            map(
                tuple((
                    preceded(many0_count(satisfy(|c| !c.is_ascii_digit())), parse_u8),
                    preceded(many0_count(satisfy(|c| !c.is_ascii_digit())), parse_u8),
                    preceded(many0_count(satisfy(|c| !c.is_ascii_digit())), parse_u8),
                )),
                |(x, y, z)| (x as usize, y as usize, z as usize),
            )(s)
        }
        separated_list1(newline, parse_row)(s)
    }

    #[allow(unused)]
    pub(crate) fn parse(s: &str) -> anyhow::Result<(Stacks, Vec<(usize, usize, usize)>)> {
        let s = s.trim_end();
        let (_, out) = all_consuming(separated_pair(parse_stack, newline, parse_moves))(s)
            .map_err(|e| e.to_owned())?;
        Ok(out)
    }
}

impl Stacks {
    fn nth_mut(&mut self, i: usize, j: usize) -> (&mut Vec<Item>, &mut Vec<Item>) {
        assert!(i != j);
        if i < j {
            let (front, back) = self.0.split_at_mut(j);
            let back = back.first_mut().unwrap();
            let (_, front) = front.split_at_mut(i);
            let front = front.first_mut().unwrap();
            (front, back)
        } else {
            let (front, back) = self.0.split_at_mut(i);
            let back = back.first_mut().unwrap();
            let (_, front) = front.split_at_mut(j);
            let front = front.first_mut().unwrap();
            (back, front)
        }
    }

    fn move_items_one_by_one(&mut self, count: usize, src: usize, dst: usize) {
        let (src, dst) = self.nth_mut(src, dst);
        dst.extend(src.drain(src.len() - count..).rev());
    }

    fn move_items_consecutive(&mut self, count: usize, src: usize, dst: usize) {
        let (src, dst) = self.nth_mut(src, dst);
        dst.extend(src.drain(src.len() - count..));
    }

    fn extract_tops(&self) -> String {
        self.0
            .iter()
            .skip(1)
            .map(|x| x.last().map(|item| item.0).unwrap())
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;
    let s = input.trim_end();
    let (stacks, moves) = parse_nom::parse(&s)?;
    println!("q1: {}", q1(stacks.clone(), &moves));
    println!("q2: {}", q2(stacks, &moves));
    Ok(())
}

fn q1(mut s: Stacks, moves: &[(usize, usize, usize)]) -> String {
    for (count, src, dst) in moves.iter().copied() {
        s.move_items_one_by_one(count, src, dst);
    }
    s.extract_tops()
}

fn q2(mut s: Stacks, moves: &[(usize, usize, usize)]) -> String {
    for (count, src, dst) in moves.iter().copied() {
        s.move_items_consecutive(count, src, dst);
    }
    s.extract_tops()
}
