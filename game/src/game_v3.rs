use std::string;
use lazy_static::lazy_static;

trait Matcher: Send + Sync {
    fn matches(&self, _: u32) -> bool {
        return true;
    }
}

struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {}

struct DivMatcher {
    divisor: u32,
}

impl DivMatcher {
    fn new(divisor: u32) -> Self {
        DivMatcher { divisor }
    }
}

impl Matcher for DivMatcher {
    fn matches(&self, number: u32) -> bool {
        number % self.divisor == 0
    }
}

struct ContainsMatcher {
    digit: u32,
}

impl ContainsMatcher {
    fn new(digit: u32) -> Self {
        ContainsMatcher { digit }
    }
}

impl Matcher for ContainsMatcher {
    fn matches(&self, number: u32) -> bool {
        number
            .to_string()
            .chars()
            .any(|c| c.to_digit(10) == Some(self.digit))
    }
}

trait Action: Send + Sync {
    fn say(&self, number: u32) -> String {
        string::ToString::to_string(&number)
    }
}

struct NumberAction;

impl Action for NumberAction {}

struct StringAction {
    output: String,
}

impl StringAction {
    fn new(output: &str) -> Self {
        StringAction {
            output: output.to_string(),
        }
    }
}

impl Action for StringAction {
    fn say(&self, _: u32) -> String {
        self.output.clone()
    }
}

trait Rule: Send + Sync {
    fn apply(&self, number: u32) -> String;
}

struct AtomRule<M: Matcher, A: Action> {
    matcher: M,
    action: A,
}

impl<M: Matcher, A: Action> AtomRule<M, A> {
    fn new(matcher: M, action: A) -> Self {
        AtomRule { matcher, action }
    }
}

impl<M: Matcher, A: Action> Rule for AtomRule<M, A> {
    fn apply(&self, number: u32) -> String {
        if self.matcher.matches(number) {
            self.action.say(number)
        } else {
            "".to_string()
        }
    }
}

struct AllOfRules<'a> {
    rules: Vec<&'a dyn Rule>,
}

impl<'a> Rule for AllOfRules<'a> {
    fn apply(&self, number: u32) -> String {
        self.rules
            .iter()
            .map(|r| r.apply(number))
            .collect::<String>()
    }
}

struct AnyOfRules<'a> {
    rules: Vec<&'a dyn Rule>,
}

impl<'a> Rule for AnyOfRules<'a> {
    fn apply(&self, number: u32) -> String {
        self.rules
            .iter()
            .map(|rule| rule.apply(number))
            .find(|result| !result.is_empty())
            .unwrap_or_default()
    }
}

pub struct FizzBuzzWhizz<'a> {
    rule: &'a dyn Rule,
}

lazy_static! {
    static ref CONTAINS_3: AtomRule<ContainsMatcher, StringAction> = AtomRule::new(
        ContainsMatcher::new(3),
        StringAction::new("fizz")
    );
    static ref DIV_3: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(3),
        StringAction::new("fizz")
    );
    static ref DIV_5: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(5),
        StringAction::new("buzz")
    );
    static ref DIV_7: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(7),
        StringAction::new("whizz")
    );
    static ref DEFAULT_RULE: AtomRule<AlwaysMatcher, NumberAction> = AtomRule::new(
        AlwaysMatcher,
        NumberAction
    );    
    static ref ALL_OF_RULES: AllOfRules<'static> = AllOfRules {
        rules: vec![&*DIV_3, &*DIV_5, &*DIV_7]
    };
    static ref ANY_OF_RULES: AnyOfRules<'static> = AnyOfRules {
        rules: vec![&*CONTAINS_3, &*ALL_OF_RULES, &*DEFAULT_RULE]
    };
}

impl<'a> FizzBuzzWhizz<'a> {
    fn from_rule(rules: &'a dyn Rule) -> Self {
        FizzBuzzWhizz { rule: rules }
    }

    pub fn new() -> Self {
        Self::from_rule(&*ANY_OF_RULES)
    }
}

use super::Game;

impl<'a> Game for FizzBuzzWhizz<'a> {
    fn apply(&self, number: u32) -> String {
        self.rule.apply(number)
    }
}

#[cfg(test)]
#[test]
fn test_fizz_buzz_whizz() {
    let contain_3 = AtomRule::new(ContainsMatcher::new(3), StringAction::new("fizz"));
    let div_3 = AtomRule::new(DivMatcher::new(3), StringAction::new("fizz"));
    let div_5 = AtomRule::new(DivMatcher::new(5), StringAction::new("buzz"));
    let div_7 = AtomRule::new(DivMatcher::new(7), StringAction::new("whizz"));
    let default_rule = AtomRule::new(AlwaysMatcher, NumberAction);

    let all_of_rules = AllOfRules {
        rules: vec![&div_3, &div_5, &div_7],
    };

    let any_of_rules = AnyOfRules {
        rules: vec![&contain_3, &all_of_rules, &default_rule],
    };

    let game = FizzBuzzWhizz::from_rule(&any_of_rules);

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