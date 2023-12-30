use std::{collections::HashMap, ops::RangeInclusive};

pub struct Day19Part2;

impl crate::days::Day for Day19Part2 {
    fn solve(&self, input: &str) -> String {
        let (workflows, _) = input.split_once("\n\n").expect("input has double linebreak");

        let workflows: HashMap<&str, Workflow> = workflows.lines()
            .map(|line| &line[..line.len() - 1]) // drop } at the end
            .map(|line| line.split_once('{').expect("line has {"))
            .map(|(name, rules)| {
                let (rules, last) = rules.rsplit_once(',').expect("has ,");
                let otherwise = Destination::parse(last);

                let rules = rules.split(',')
                    .map(|rule| {
                        let (rule, dest) = rule.split_once(':').expect("rule has :");
                        let destination = Destination::parse(dest);

                        if let Some((var, value)) = rule.split_once('>') {
                            Rule {
                                var_name: VarName::parse(var),
                                greater_than: true,
                                value: value.parse().expect("is integer"),
                                destination,
                            }
                        } else if let Some((var, value)) = rule.split_once('<') {
                            Rule {
                                var_name: VarName::parse(var),
                                greater_than: false,
                                value: value.parse().expect("is integer"),
                                destination,
                            }
                        } else {
                            panic!("rule needs to contain either > or <");
                        }
                    })
                    .collect();

                (
                    name,
                    Workflow {
                        _name: name,
                        rules,
                        otherwise,
                    }
                )
            })
            .collect();

        let start_input = Input {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        };

        let first_workflow = &workflows["in"];
        first_workflow
            .try_input(start_input, &workflows)
            .to_string()
    }
}

#[derive(Debug, Clone)]
struct Input {
    x: RangeInclusive<u16>,
    m: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
}

impl Input {
    fn count(self) -> u64 {
        u64::try_from(self.x.count()).unwrap() *
        u64::try_from(self.m.count()).unwrap() *
        u64::try_from(self.a.count()).unwrap() *
        u64::try_from(self.s.count()).unwrap()
    }
}

#[derive(Debug)]
enum VarName {
    X,
    M,
    A,
    S,
}

impl VarName {
    fn parse(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid variable name"),
        }
    }
}

#[derive(Debug)]
struct Workflow<'input> {
    _name: &'input str,
    rules: Vec<Rule<'input>>,
    otherwise: Destination<'input>,
}

impl<'input> Workflow<'input> {
    fn try_input(&self, input: Input, workflows: &HashMap<&str, Workflow>) -> u64 {
        let mut sum = 0;

        let mut rest_input = input;
        for rule in &self.rules {
            let (maybe_matched, unmatched) = rule.applies_for(rest_input);
            rest_input = unmatched;

            if let Some(matched) = maybe_matched {
                match rule.destination {
                    Destination::Workflow(wf) => {
                        sum += workflows[wf].try_input(matched, workflows);
                    }
                    Destination::Accepted => sum += matched.count(),
                    Destination::Rejected => {}
                }
            }
        }

        match self.otherwise {
            Destination::Workflow(wf) => {
                sum += workflows[wf].try_input(rest_input, workflows);
            }
            Destination::Accepted => sum += rest_input.count(),
            Destination::Rejected => {}
        }

        sum
    }
}

#[derive(Debug)]
struct Rule<'input> {
    var_name: VarName,
    greater_than: bool,
    value: u16,
    destination: Destination<'input>,
}

impl<'input> Rule<'input> {
    fn applies_for(&self, mut input: Input) -> (Option<Input>, Input) {
        let input_range = match self.var_name {
            VarName::X => &input.x,
            VarName::M => &input.m,
            VarName::A => &input.a,
            VarName::S => &input.s,
        };

        let (matched_range, rest_range) = if self.greater_than {
            if input_range.end() <= &self.value {
                return (None, input);
            }

            (
                self.value + 1..=*input_range.end(),
                *input_range.start()..=self.value
            )
        } else {
            if input_range.start() >= &self.value {
                return (None, input);
            }

            (
                *input_range.start()..=self.value - 1,
                self.value..=*input_range.end()
            )
        };

        let mut result = input.clone();
        match self.var_name {
            VarName::X => result.x = matched_range,
            VarName::M => result.m = matched_range,
            VarName::A => result.a = matched_range,
            VarName::S => result.s = matched_range,
        }

        match self.var_name {
            VarName::X => input.x = rest_range,
            VarName::M => input.m = rest_range,
            VarName::A => input.a = rest_range,
            VarName::S => input.s = rest_range,
        }

        (Some(result), input)
    }
}

#[derive(Debug)]
enum Destination<'input> {
    Workflow(&'input str),
    Accepted,
    Rejected,
}

impl<'input> Destination<'input> {
    fn parse(s: &'input str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Workflow(s),
        }
    }
}
