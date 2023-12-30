use std::collections::HashMap;

pub struct Day19Part1;

impl crate::days::Day for Day19Part1 {
    fn solve(&self, input: &str) -> String {
        let (workflows, inputs) = input.split_once("\n\n").expect("input has double linebreak");

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

        let inputs: Vec<Input> = inputs.lines()
            .map(|line| &line[1..line.len() - 1]) // trim { and }
            .map(|line| {
                let vals: Vec<_> = line.split(',')
                    .map(|s| s[2..].parse().expect("is integer"))
                    .collect();

                assert_eq!(vals.len(), 4);
                Input {
                    x: vals[0],
                    m: vals[1],
                    a: vals[2],
                    s: vals[3],
                }
            })
            .collect();

        
        inputs.into_iter()
            .filter(|input| input.run_workflow("in", &workflows))
            .map(|input| input.x as u32 + input.m as u32 + input.a as u32 + input.s as u32)
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug)]
struct Input {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Input {
    fn run_workflow(&self, current_workflow: &str, workflows: &HashMap<&str, Workflow>) -> bool {
        let workflow = &workflows[current_workflow];

        let mut destination = &workflow.otherwise;
        for rule in &workflow.rules {
            if let Some(dest) = rule.apply_on(self) {
                destination = dest;
                break;
            }
        }

        match destination {
            Destination::Workflow(name) => self.run_workflow(name, workflows),
            Destination::Accepted => true,
            Destination::Rejected => false,
        }
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

#[derive(Debug)]
struct Rule<'input> {
    var_name: VarName,
    greater_than: bool,
    value: u16,
    destination: Destination<'input>,
}

impl<'input> Rule<'input> {
    fn apply_on(&self, input: &Input) -> Option<&Destination> {
        let input_value = match self.var_name {
            VarName::X => input.x,
            VarName::M => input.m,
            VarName::A => input.a,
            VarName::S => input.s,
        };

        if self.greater_than && input_value > self.value {
            Some(&self.destination)
        } else if !self.greater_than && input_value < self.value {
            Some(&self.destination)
        } else {
            None
        }
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
