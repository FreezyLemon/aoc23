use std::collections::HashMap;

pub struct Day20Part1;

impl crate::days::Day for Day20Part1 {
    fn solve(&self, input: &str) -> String {
        let mut modules: HashMap<&str, Module> = input.lines()
            .map(|line| line.split_once(" -> ").expect("line has ->"))
            .map(|(module, destinations)| {
                let kind = match &module[..1] {
                    "%" => ModuleKind::FlipFlop(false),
                    "&" => ModuleKind::Conjunction(Vec::new()),
                    _ => ModuleKind::Untyped,
                };

                let name = if kind == ModuleKind::Untyped {
                    module
                } else {
                    &module[1..]
                };

                let kind = if name == "broadcaster" {
                    ModuleKind::Broadcast
                } else {
                    kind
                };

                let destinations = destinations.split(',')
                    .map(|m| m.trim())
                    .collect();

                (
                    name,
                    Module {
                        name,
                        kind,
                        destinations,
                    }
                )
            })
            .collect();

        // initialize memory for conjunction modules
        let conjunctions: Vec<_> = modules.values()
            .filter(|m| {
                if let ModuleKind::Conjunction(_) = m.kind {
                    true
                } else {
                    false
                }
            })
            .map(|m| m.name)
            .collect();

        for conj in conjunctions {
            let pointing_to_conj: Vec<_> = modules.values()
                .filter(|m| m.destinations.contains(&conj))
                .map(|m| m.name)
                .collect();

            for other in pointing_to_conj {
                let Some(module) = modules.get_mut(conj) else {
                    unreachable!()
                };

                let ModuleKind::Conjunction(ref mut others) = module.kind else {
                    unreachable!()
                };

                others.push((other, Pulse::Low));
            }
        }

        // done initializing
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        for _ in 0..1000 {
            let mut pulses = vec![("button", "broadcaster", Pulse::Low)];
            'button_push: loop {
                let mut next_pulses = Vec::new();
                for (from, to, pulse) in pulses {
                    if pulse == Pulse::High {
                        high_pulses += 1;
                    } else {
                        low_pulses += 1;
                    }

                    if let Some(to_module) = modules.get_mut(to) {
                        next_pulses.extend(to_module.handle_pulse(pulse, from));
                    };
                }
    
                if next_pulses.is_empty() {
                    break 'button_push;
                }
    
                pulses = next_pulses;
            }

        }

        (low_pulses * high_pulses).to_string()
    }
}

#[derive(Debug)]
struct Module<'input> {
    name: &'input str,
    kind: ModuleKind<'input>,
    destinations: Vec<&'input str>,
}

impl<'input> Module<'input> {
    fn handle_pulse(&mut self, pulse: Pulse, from: &'input str) -> Vec<(&'input str, &'input str, Pulse)> {
        match self.kind {
            ModuleKind::Broadcast => self.destinations.iter().map(|d| (self.name, *d, pulse)).collect(),
            ModuleKind::FlipFlop(ref mut on) => {
                if pulse != Pulse::Low {
                    return vec![];
                }

                *on = !*on;

                self.destinations.iter().map(|d| {
                    (
                        self.name,
                        *d,
                        if *on {
                            Pulse::High
                        } else {
                            Pulse::Low
                        }
                    )
                })
                .collect()
            }
            ModuleKind::Conjunction(ref mut others) => {
                let Some((_, ref mut other_pulse)) = others.iter_mut().find(|(m, _)| *m == from) else {
                    unreachable!()
                };

                *other_pulse = pulse;

                let out_pulse = if others.iter().all(|(_, p)| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                self.destinations.iter()
                    .map(|d| (self.name, *d, out_pulse))
                    .collect()
            }
            ModuleKind::Untyped => vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
enum ModuleKind<'input> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(&'input str, Pulse)>),
    Untyped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}
