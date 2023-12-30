use std::collections::VecDeque;

pub struct Day20Part1;

impl crate::days::Day for Day20Part1 {
    fn solve(&self, input: &str) -> String {
        let modules: Vec<(&str, Module, Vec<&str>)> = input.lines()
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

                let str_destinations: Vec<_> = destinations.split(',')
                    .map(|m| m.trim())
                    .collect();

                (
                    name,
                    Module {
                        kind,
                        destinations: Vec::new(),
                    },
                    str_destinations
                )
            })
            .collect();

        // output modules don't really exist.
        // we use indices that come after real modules for these,
        // and remember them for later
        let mut output_index = modules.len();
        let mut output_modules = Vec::new();

        let mut modules: Vec<Module> = modules.iter()
            .map(|(_, module, str_destinations)| {
                let destinations = str_destinations.into_iter()
                    .map(|dest| {
                        if let Some((idx, _)) = modules.iter()
                            .enumerate()
                            .find(|(_, (name, _, _))| *name == *dest)
                        {
                            idx
                        }
                        else
                        {
                            output_index += 1;
                            output_modules.push((dest, output_index));
                            output_index
                        }
                    });

                let mut module = module.clone();
                module.destinations.extend(destinations);

                module
            })
            .collect();

        // initialize memory for conjunction modules
        let conjunctions: Vec<_> = modules.iter()
            .enumerate()
            .filter(|(_, m)| {
                if let ModuleKind::Conjunction(_) = m.kind {
                    true
                } else {
                    false
                }
            })
            .map(|(idx, _)| idx)
            .collect();

        for conj in conjunctions {
            let pointing_to_conj: Vec<_> = modules.iter()
                .enumerate()
                .filter(|(_, m)| m.destinations.contains(&conj))
                .map(|(idx, _)| idx)
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

        let broadcast_idx = modules.iter()
            .enumerate()
            .find(|(_, m)| m.kind == ModuleKind::Broadcast)
            .map(|(idx, _)| idx)
            .expect("broadcast exists");

        // done initializing
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        let mut pulses = VecDeque::with_capacity(100);
        for _ in 0..1000 {
            pulses.push_back((broadcast_idx, broadcast_idx, Pulse::Low));

            while let Some((from, to, pulse)) = pulses.pop_front() {
                if pulse == Pulse::High {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }

                if let Some(to_module) = modules.get_mut(to) {
                    to_module.handle_pulse(&mut pulses, pulse, from, to);
                };
            }
        }

        (low_pulses * high_pulses).to_string()
    }
}

#[derive(Debug, Clone)]
struct Module {
    kind: ModuleKind,
    destinations: Vec<usize>,
}

impl Module {
    fn handle_pulse(&mut self, pulses: &mut VecDeque<(usize, usize, Pulse)>, pulse: Pulse, from: usize, to: usize) {
        match self.kind {
            ModuleKind::Broadcast => {
                for d in &self.destinations {
                    pulses.push_back((to, *d, pulse));
                }
            },
            ModuleKind::FlipFlop(ref mut on) => {
                if pulse != Pulse::Low {
                    return;
                }

                *on = !*on;

                let pulse = if *on {
                    Pulse::High
                } else {
                    Pulse::Low
                };

                for d in &self.destinations {
                    pulses.push_back((to, *d, pulse));
                }
            }
            ModuleKind::Conjunction(ref mut others) => {
                let Some((_, ref mut other_pulse)) = others.iter_mut().find(|(m, _)| *m == from) else {
                    unreachable!();
                };

                *other_pulse = pulse;

                let out_pulse = if others.iter().all(|(_, p)| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for d in &self.destinations {
                    pulses.push_back((to, *d, out_pulse));
                }
            }
            ModuleKind::Untyped => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(usize, Pulse)>),
    Untyped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}
