use std::string;

trait Matcher {
    fn matches(&self, number: u32) -> bool;
}

struct DivMatcher {
    divisor: u32,
}

impl DivMatcher {
    fn new(divisor: u32) -> Self {
        DivMatcher{divisor}
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
        ContainsMatcher{digit}
    }
}

impl Matcher for ContainsMatcher {
    fn matches(&self, number: u32) -> bool {
        let mut n = number;
        while n > 0 {
            if n % 10 == self.digit {
                return true;
            }
            n /= 10;
        }
        false
    }
}

struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {
    fn matches(&self, _number: u32) -> bool {
        true
    }
}

trait Action {
    fn say(&self, number: u32) -> String;
}

struct StringAction {
    output: String,
}

impl StringAction{
    fn new(output: &str) -> Self {
        StringAction{output: output.to_string()}
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

trait Rule {
    fn apply(&self, number: u32) -> String;
}

struct AtomRule<M : Matcher, A : Action> {
    matcher : M,
    action : A,
}

impl<M: Matcher, A: Action>  AtomRule<M, A> {
    fn new(matcher: M, action: A) -> Self {
        AtomRule{matcher, action}
    }
}

impl<M: Matcher, A: Action> Rule for AtomRule<M, A> {
    fn apply(&self, number: u32) -> String {
        if self.matcher.matches(number) {
            self.action.say(number)
        } else {
            String::new()
        }
    }
}

struct AllOfRules<'a> {
    rules : Vec<&'a dyn Rule>,
}

impl<'a> Rule for AllOfRules<'a> {
    fn apply(&self, number: u32) -> String {
        self.rules.iter().fold("".to_string(), |acc, r| {
            acc + &r.apply(number)
        })
    }
}

struct AnyOfRules<'a> {
    rules : Vec<&'a dyn Rule>,
}

impl<'a> Rule for AnyOfRules<'a> {
    fn apply(&self, number: u32) -> String {
        for r in &self.rules {
            let result = r.apply(number);
            if !result.is_empty() {
                return result
            }
        }
        String::new()
    }
}

fn main() {
    let fizz = AtomRule::new(DivMatcher::new(3), StringAction::new("fizz"));
    let buzz = AtomRule::new(DivMatcher::new(5), StringAction::new("buzz"));
    let whizz = AtomRule::new(DivMatcher::new(7), StringAction::new("whizz"));

    let contains= AtomRule::new(ContainsMatcher::new(3), StringAction::new("whizz"));
    let default = AtomRule{matcher : AlwaysMatcher, action : NumberAction};

    let all_of = AllOfRules{rules : vec![&fizz, &buzz, &whizz]};
    let any_of = AnyOfRules{rules : vec![&contains, &all_of, &default]};

    let game = &any_of;

    for i in 1..110 {
        println!("number of {} student say {}", i, game.apply(i));
    }
}
