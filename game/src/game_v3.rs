use std::string;

// 定义 Matcher trait
trait Matcher: Send + Sync {
    fn matches(&self, number: u32) -> bool;
}

// DivMatcher 结构体及其实现
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

// ContainsMatcher 结构体及其实现
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

// AlwaysMatcher 结构体及其实现
struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {
    fn matches(&self, _number: u32) -> bool {
        true
    }
}

// 定义 Action trait
trait Action: Send + Sync {
    fn say(&self, number: u32) -> String;
}

// StringAction 结构体及其实现
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

// NumberAction 结构体及其实现
struct NumberAction;

impl Action for NumberAction {
    fn say(&self, number: u32) -> String {
        number.to_string()
    }
}

// 定义 Rule trait
trait Rule: Send + Sync {
    fn apply(&self, number: u32) -> String;
}

// AtomRule 结构体及其实现
struct AtomRule<'a, M: Matcher, A: Action> {
    matcher: &'a M,
    action: &'a A,
}

impl<'a, M: Matcher, A: Action> AtomRule<'a, M, A> {
    fn new(matcher: &'a M, action: &'a A) -> Self {
        AtomRule { matcher, action }
    }
}

impl<'a, M: Matcher, A: Action> Rule for AtomRule<'a, M, A> {
    fn apply(&self, number: u32) -> String {
        if self.matcher.matches(number) {
            self.action.say(number)
        } else {
            "".to_string()
        }
    }
}

// AllOfRules 结构体及其实现
struct AllOfRules<'a> {
    rules: Vec<&'a dyn Rule>,
}

impl<'a> AllOfRules<'a> {
    fn new(rules: Vec<&'a dyn Rule>) -> Self {
        AllOfRules { rules }
    }
}

impl<'a> Rule for AllOfRules<'a> {
    fn apply(&self, number: u32) -> String {
        self.rules
            .iter()
            .map(|r| r.apply(number))
            .collect::<String>()
    }
}

// AnyOfRules 结构体及其实现
struct AnyOfRules<'a> {
    rules: Vec<&'a dyn Rule>,
}

impl<'a> AnyOfRules<'a> {
    fn new(rules: Vec<&'a dyn Rule>) -> Self {
        AnyOfRules { rules }
    }
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

// FizzBuzzWhizz 结构体及其实现
pub struct FizzBuzzWhizz<'a> {
    // 存储匹配器
    contains_3: ContainsMatcher,
    div_3: DivMatcher,
    div_5: DivMatcher,
    div_7: DivMatcher,
    always: AlwaysMatcher,

    // 存储动作
    action_fizz: StringAction,
    action_buzz: StringAction,
    action_whizz: StringAction,
    action_number: NumberAction,

    // 存储 AtomRule 实例
    atom_contains_3: AtomRule<'a, ContainsMatcher, StringAction>,
    atom_div_3: AtomRule<'a, DivMatcher, StringAction>,
    atom_div_5: AtomRule<'a, DivMatcher, StringAction>,
    atom_div_7: AtomRule<'a, DivMatcher, StringAction>,
    atom_always: AtomRule<'a, AlwaysMatcher, NumberAction>,

    // 存储组合规则
    all_of_rules: AllOfRules<'a>,
    any_of_rules: AnyOfRules<'a>,
}

impl<'a> FizzBuzzWhizz<'a> {
    pub fn new() -> Self {
        // 初始化匹配器
        let contains_3 = ContainsMatcher::new(3);
        let div_3 = DivMatcher::new(3);
        let div_5 = DivMatcher::new(5);
        let div_7 = DivMatcher::new(7);
        let always = AlwaysMatcher;

        // 初始化动作
        let action_fizz = StringAction::new("fizz");
        let action_buzz = StringAction::new("buzz");
        let action_whizz = StringAction::new("whizz");
        let action_number = NumberAction;

        // 初始化 AtomRule
        let atom_contains_3 = AtomRule::new(&contains_3, &action_fizz);
        let atom_div_3 = AtomRule::new(&div_3, &action_fizz);
        let atom_div_5 = AtomRule::new(&div_5, &action_buzz);
        let atom_div_7 = AtomRule::new(&div_7, &action_whizz);
        let atom_always = AtomRule::new(&always, &action_number);

        // 初始化 AllOfRules
        let all_of_rules = AllOfRules::new(vec![&atom_div_3, &atom_div_5, &atom_div_7]);

        // 初始化 AnyOfRules
        let any_of_rules = AnyOfRules::new(vec![&atom_contains_3, &all_of_rules, &atom_always]);

        FizzBuzzWhizz {
            contains_3,
            div_3,
            div_5,
            div_7,
            always,

            action_fizz,
            action_buzz,
            action_whizz,
            action_number,

            atom_contains_3,
            atom_div_3,
            atom_div_5,
            atom_div_7,
            atom_always,

            all_of_rules,
            any_of_rules,
        }
    }
}

use super::Game;

impl<'a> Game for FizzBuzzWhizz<'a> {
    fn apply(&self, number: u32) -> String {
        self.any_of_rules.apply(number)
    }
}
