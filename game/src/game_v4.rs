use std::sync::OnceLock;

fn div_matcher(divisor: u32) -> impl Fn(u32) -> bool + Sync {
    move |number| number % divisor == 0
}

fn contains_matcher(digit: u32) -> impl Fn(u32) -> bool + Sync {
    move |number| number.to_string().contains(&digit.to_string())
}

fn always_matcher() -> impl Fn(u32) -> bool + Sync {
    move |_number| true
}

fn string_action(output: &'static str) -> impl Fn(u32) -> String + Sync {
    move |_number| output.to_string()
}

fn number_action() -> impl Fn(u32) -> String + Sync {
    move |number| number.to_string()
}

fn atom_rule<M, A>(matcher: M, action: A) -> impl Fn(u32) -> String + Sync
where
    M: Fn(u32) -> bool + Sync,
    A: Fn(u32) -> String + Sync,
{
    move |number| {
        if matcher(number) {
            action(number)
        } else {
            String::new()
        }
    }
}

fn all_of_rule<'a>(
    rules: &'a [&(dyn Fn(u32) -> String + Sync)],
) -> impl Fn(u32) -> String + Sync + 'a {
    move |number| {
        let mut result = String::new();
        for rule in rules.iter() {
            result.push_str(&rule(number));
        }
        result
    }
}

fn any_of_rule<'a>(
    rules: &'a [&(dyn Fn(u32) -> String + Sync)],
) -> impl Fn(u32) -> String + Sync + 'a {
    move |number| {
        for rule in rules.iter() {
            let res = rule(number);
            if !res.is_empty() {
                return res;
            }
        }
        String::new()
    }
}

pub struct FizzBuzzWhizz<'a> {
    rule: &'a (dyn Fn(u32) -> String + Sync),
}

static RULE_CONTAINS: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static RULE_DIV3: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static RULE_DIV5: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static RULE_DIV7: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static RULE_DEFAULT: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static DIV_RULES: OnceLock<Vec<&'static (dyn Fn(u32) -> String + Sync)>> = OnceLock::new();
static RULE_DIVISIBLE: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();
static COMPOSITE_RULES: OnceLock<Vec<&'static (dyn Fn(u32) -> String + Sync)>> = OnceLock::new();
static RULE_GAME: OnceLock<Box<dyn Fn(u32) -> String + Send + Sync>> = OnceLock::new();

impl<'a> FizzBuzzWhizz<'a> {
    pub fn from_rule(rule: &'a (dyn Fn(u32) -> String + Sync)) -> Self {
        FizzBuzzWhizz { rule }
    }

    pub fn new() -> Self {
        RULE_CONTAINS.get_or_init(|| {
            Box::new(atom_rule(contains_matcher(3), string_action("Fizz")))
        });
        RULE_DIV3.get_or_init(|| {
            Box::new(atom_rule(div_matcher(3), string_action("Fizz")))
        });
        RULE_DIV5.get_or_init(|| {
            Box::new(atom_rule(div_matcher(5), string_action("Buzz")))
        });
        RULE_DIV7.get_or_init(|| {
            Box::new(atom_rule(div_matcher(7), string_action("Whizz")))
        });
        RULE_DEFAULT.get_or_init(|| {
            Box::new(atom_rule(always_matcher(), number_action()))
        });
    
        DIV_RULES.get_or_init(|| {
            vec![
                RULE_DIV3.get().unwrap().as_ref(),
                RULE_DIV5.get().unwrap().as_ref(),
                RULE_DIV7.get().unwrap().as_ref(),
            ]
        });
    
        RULE_DIVISIBLE.get_or_init(|| {
            Box::new(all_of_rule(DIV_RULES.get().unwrap().as_slice()))
        });
    
        COMPOSITE_RULES.get_or_init(|| {
            vec![
                RULE_CONTAINS.get().unwrap().as_ref(),
                RULE_DIVISIBLE.get().unwrap().as_ref(),
                RULE_DEFAULT.get().unwrap().as_ref(),
            ]
        });
    
        RULE_GAME.get_or_init(|| {
            Box::new(any_of_rule(COMPOSITE_RULES.get().unwrap().as_slice()))
        });

        Self::from_rule(RULE_GAME.get().unwrap().as_ref())
    }
}

use super::Game;

impl<'a> Game for FizzBuzzWhizz<'a> {
    fn apply(&self, number: u32) -> String {
        (self.rule)(number)
    }
}

#[cfg(test)]
#[test]
fn test_fizz_buzz_whizz() {
    let rule_contains_3 = atom_rule(contains_matcher(3), string_action("fizz"));
    let rule_div_3 = atom_rule(div_matcher(3), string_action("fizz"));
    let rule_div_5 = atom_rule(div_matcher(5), string_action("buzz"));
    let rule_div_7 = atom_rule(div_matcher(7), string_action("whizz"));
    let default_rule = atom_rule(always_matcher(), number_action());

    let div_rules: [&(dyn Fn(u32) -> String + Sync); 3] = [&rule_div_3, &rule_div_5, &rule_div_7];
    let all_rules = all_of_rule(&div_rules);

    let any_rules: [&(dyn Fn(u32) -> String + Sync); 3] =
        [&rule_contains_3, &all_rules, &default_rule];
    let game_rule = any_of_rule(&any_rules);

    let game = FizzBuzzWhizz::from_rule(&game_rule);

    let test_cases = vec![
        (1, "1"),
        (2, "2"),        
        (3, "fizz"),
        (5, "buzz"),
        (7, "whizz"),
        (13, "fizz"),
        (15, "fizzbuzz"),
        (21, "fizzwhizz"),
        (31, "fizz"),
        (35, "fizz"),
        (70, "buzzwhizz"),
        (105, "fizzbuzzwhizz"),
    ];

    for (input, expected_output) in test_cases {
        let result = game.apply(input);
        assert_eq!(result, expected_output, "For input {}, expected {} but got {}", input, expected_output, result);
    }
} 