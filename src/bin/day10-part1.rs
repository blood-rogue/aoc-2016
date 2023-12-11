use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy)]
enum Recipient {
    Bot(usize),
    Output(usize),
}

impl Recipient {
    fn parse(s: &str, v: usize) -> Self {
        match s {
            "bot" => Self::Bot(v),
            "output" => Self::Output(v),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bot {
    low: Option<usize>,
    high: Option<usize>,
}

impl Bot {
    const fn new(value: usize) -> Self {
        Self {
            low: Some(value),
            high: None,
        }
    }

    const fn full(self) -> bool {
        self.low.is_some() && self.high.is_some()
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    low_to: Recipient,
    high_to: Recipient,
}

fn main() {
    let input_pattern = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let output_pattern =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")
            .unwrap();

    let mut bots = HashMap::new();
    let mut instructions = HashMap::new();

    for line in include_str!(r"..\..\input\day10.txt").lines() {
        if let Some(captures) = input_pattern.captures(line) {
            let id = captures[2].parse::<usize>().unwrap();
            let input = captures[1].parse::<usize>().unwrap();

            bots.entry(id)
                .and_modify(|bot: &mut Bot| {
                    if bot.low <= Some(input) {
                        bot.high = Some(input);
                    } else {
                        bot.high = bot.low;
                        bot.low = Some(input);
                    }
                })
                .or_insert_with(|| Bot::new(input));
        }

        if let Some(captures) = output_pattern.captures(line) {
            let id = captures[1].parse::<usize>().unwrap();

            let low_to = Recipient::parse(&captures[2], captures[3].parse().unwrap());
            let high_to = Recipient::parse(&captures[4], captures[5].parse().unwrap());

            instructions.insert(id, Instruction { low_to, high_to });
        }
    }

    while !bots.is_empty() {
        let bots_clone = bots.clone();
        let (id, bot) = bots_clone.iter().find(|(_, bot)| bot.full()).unwrap();

        if *bot
            == (Bot {
                low: Some(17),
                high: Some(61),
            })
        {
            dbg!(id);
            break;
        }

        if let Some(instructions) = instructions.get(id) {
            if let Recipient::Bot(id) = instructions.low_to {
                let low_value = bot.low.unwrap();

                bots.entry(id)
                    .and_modify(|bot: &mut Bot| {
                        if bot.low <= Some(low_value) {
                            bot.high = Some(low_value);
                        } else {
                            bot.high = bot.low;
                            bot.low = Some(low_value);
                        }
                    })
                    .or_insert_with(|| Bot::new(low_value));
            }

            if let Recipient::Bot(id) = instructions.high_to {
                let high_value = bot.high.unwrap();

                bots.entry(id)
                    .and_modify(|bot: &mut Bot| {
                        if bot.low <= Some(high_value) {
                            bot.high = Some(high_value);
                        } else {
                            bot.high = bot.low;
                            bot.low = Some(high_value);
                        }
                    })
                    .or_insert_with(|| Bot::new(high_value));
            }
        }

        bots.remove(id);
    }
}
