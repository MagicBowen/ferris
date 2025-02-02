use std::string;
use lazy_static::lazy_static;

trait Matcher: Send + Sync {
    fn matches(&self, number: u32) -> bool;
}

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

struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {
    fn matches(&self, _number: u32) -> bool {
        true
    }
}

trait Action: Send + Sync {
    fn say(&self, number: u32) -> String;
}

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
    fn say(&self, _number: u32) -> String {
        self.output.clone()
    }
}

struct NumberAction;

impl Action for NumberAction {
    fn say(&self, number: u32) -> String {
        string::ToString::to_string(&number)
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

impl<M: Matcher + 'static, A: Action + 'static> Rule for AtomRule<M, A> {
    fn apply(&self, number: u32) -> String {
        if self.matcher.matches(number) {
            self.action.say(number)
        } else {
            "".to_string()
        }
    }
}

struct AllOfRules {
    rules: Vec<&'static dyn Rule>,
}

impl Rule for AllOfRules {
    fn apply(&self, number: u32) -> String {
        self.rules
            .iter()
            .map(|r| r.apply(number))
            .collect::<String>()
    }
}

struct AnyOfRules {
    rules: Vec<&'static dyn Rule>,
}

impl Rule for AnyOfRules {
    fn apply(&self, number: u32) -> String {
        self.rules
            .iter()
            .map(|rule| rule.apply(number))
            .find(|result| !result.is_empty())
            .unwrap_or_default()
    }
}

lazy_static! {
    static ref CONTAINS_RULE: AtomRule<ContainsMatcher, StringAction> = AtomRule::new(
        ContainsMatcher::new(3),
        StringAction::new("fizz")
    );
    static ref DIV3_RULE: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(3),
        StringAction::new("fizz")
    );
    static ref DIV5_RULE: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(5),
        StringAction::new("buzz")
    );
    static ref DIV7_RULE: AtomRule<DivMatcher, StringAction> = AtomRule::new(
        DivMatcher::new(7),
        StringAction::new("whizz")
    );
    static ref ALWAYS_RULE: AtomRule<AlwaysMatcher, NumberAction> = AtomRule::new(
        AlwaysMatcher,
        NumberAction
    );
    static ref ALL_OF_RULES: AllOfRules = AllOfRules {
        rules: vec![
            &*DIV3_RULE as &dyn Rule,
            &*DIV5_RULE,
            &*DIV7_RULE,
        ]
    };
    static ref ANY_OF_RULES: AnyOfRules = AnyOfRules {
        rules: vec![
            &*CONTAINS_RULE as &dyn Rule,
            &*ALL_OF_RULES,
            &*ALWAYS_RULE,
        ]
    };
}

pub struct FizzBuzzWhizz {
    rule: &'static dyn Rule,
}

impl FizzBuzzWhizz {
    pub fn new() -> Self {
        FizzBuzzWhizz {
            rule: &*ANY_OF_RULES,
        }
    }
}

use super::Game;

impl Game for FizzBuzzWhizz {
    fn apply(&self, number: u32) -> String {
        self.rule.apply(number)
    }
}