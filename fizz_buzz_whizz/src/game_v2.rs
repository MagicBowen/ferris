type Matcher = Box<dyn Fn(u32) -> bool>;

fn div_matcher(divisor: u32) -> Matcher {
    Box::new(move |number| number % divisor == 0)
}

fn contains_matcher(digit: u32) -> Matcher {
    Box::new(move |number| number.to_string().contains(&digit.to_string()))
}

fn always_matcher() -> Matcher {
    Box::new(|_number| true)
}

type Action = Box<dyn Fn(u32) -> String>;

fn string_action(output: &'static str) -> Action {
    Box::new(move |_number| output.to_string())
}

fn number_action() -> Action {
    Box::new(|number| number.to_string())
}

type Rule = Box<dyn Fn(u32) -> String>;

fn atom_rule(matcher: Matcher, action: Action) -> Rule {
    Box::new(move |number| {
        if matcher(number) {
            action(number)
        } else {
            "".to_string()
        }
    })
}

fn all_of_rule(rules: Vec<Rule>) -> Rule {
    Box::new(move |number| {
        rules.iter().map(|rule| rule(number)).collect()
    })
}

fn any_of_rule(rules: Vec<Rule>) -> Rule {
    Box::new(move |number| {
        rules.iter()
             .map(|rule| rule(number))
             .find(|result| !result.is_empty())
             .unwrap_or_default()
    })
}

use super::Game;

pub struct FizzBuzzWhizz {
    rule: Rule,
}

impl FizzBuzzWhizz {
    const FIZZ: &'static str = "Fizz";
    const BUZZ: &'static str = "Buzz";
    const WHIZZ: &'static str = "Whizz";

    pub fn new() -> Self {
        FizzBuzzWhizz {
            rule: any_of_rule(vec![
                atom_rule(contains_matcher(3), string_action(Self::FIZZ)),
                all_of_rule(vec![
                    atom_rule(div_matcher(3), string_action(Self::FIZZ)),
                    atom_rule(div_matcher(5), string_action(Self::BUZZ)),
                    atom_rule(div_matcher(7), string_action(Self::WHIZZ)),
                ]),
                atom_rule(always_matcher(), number_action()),
            ]),
        }
    }
}

impl Game for FizzBuzzWhizz {
    fn apply(&self, number: u32) -> String {
        (self.rule)(number)
    }
}
