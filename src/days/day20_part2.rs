use std::collections::VecDeque;

pub struct Day20Part2;

impl crate::days::Day for Day20Part2 {
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

        let mut last_module = None;

        let mut modules: Vec<Module> = modules.iter()
            .enumerate()
            .map(|(curr_module_index, (_, module, str_destinations))| {
                let destinations = str_destinations.into_iter()
                    .filter_map(|dest| {
                        if let Some((idx, _)) = modules.iter()
                            .enumerate()
                            .find(|(_, (name, _, _))| *name == *dest)
                        {
                            Some(idx)
                        } else {
                            if *dest == "rx" {
                                last_module = Some(curr_module_index);
                            }
                            None
                        }
                    });

                let mut module = module.clone();
                module.destinations.extend(destinations);

                module
            })
            .collect();

        let last_module = last_module.expect("one module points to rx");

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
        let mut pulses = VecDeque::with_capacity(100);
        let mut first_pulses = vec![(None, None); modules.len()];
        let mut leftover_modules = 2 * first_pulses.len();
        'outer: for idx in 1.. {
            pulses.push_back((broadcast_idx, broadcast_idx, Pulse::Low));

            while let Some((from, to, pulse)) = pulses.pop_front() {
                let (low, high) = &mut first_pulses[from];
                if pulse == Pulse::Low && low.is_none() {
                    leftover_modules -= 1;
                    *low = Some(idx);
                } else if pulse == Pulse::High && high.is_none() {
                    leftover_modules -= 1;
                    *high = Some(idx);
                }

                // broadcaster will never send high,
                // plus the last step will need to be calculated
                if leftover_modules == 3 {
                    break 'outer;
                }

                if let Some(to_module) = modules.get_mut(to) {
                    to_module.handle_pulse(&mut pulses, pulse, from, to);
                };
            }
        }

        modules.iter()
            .enumerate()
            .filter(|(_, m)| m.destinations.contains(&last_module))
            .map(|(idx, _)| first_pulses[idx].1)
            .map(Option::unwrap)
            .reduce(lcm)
            .unwrap()
            .to_string()
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
