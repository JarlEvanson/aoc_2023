use std::collections::VecDeque;

use fxhash::FxHashMap;

use crate::util::math::lcm_single;

pub fn solve(input: &str) -> (usize, usize) {
    let mut modules = input
        .lines()
        .map(|val| {
            let (kind, targets) = val.split_once("->").unwrap();

            let targets = targets
                .split(",")
                .map(|target| target.trim())
                .collect::<Vec<_>>();

            let (name, kind) = if kind.starts_with("broadcaster") {
                ("broadcaster", ModuleType::BroadCaster)
            } else if kind.starts_with("%") {
                (
                    kind.trim_start_matches('%').trim(),
                    ModuleType::FlipFlop(false),
                )
            } else if kind.starts_with("&") {
                (
                    kind.trim_start_matches('&').trim(),
                    ModuleType::Conjunction(FxHashMap::default()),
                )
            } else {
                unreachable!()
            };

            (name, (kind, targets))
        })
        .collect::<FxHashMap<_, _>>();

    let mut prev_rx = "";

    {
        let mut conjunctions = FxHashMap::default();

        for (&name, (kind, _)) in modules.iter() {
            if let ModuleType::Conjunction(_) = kind {
                conjunctions.insert(name, FxHashMap::default());
            }
        }

        for (&name, (_, targets)) in modules.iter() {
            for &target in targets {
                if target == "rx" {
                    prev_rx = name;
                }

                if let Some(ModuleType::Conjunction(_)) = modules.get(target).map(|l| &l.0) {
                    conjunctions.get_mut(target).unwrap().insert(name, false);
                }
            }
        }

        for (conjunction_name, connections) in conjunctions {
            modules.get_mut(conjunction_name).unwrap().0 = ModuleType::Conjunction(connections);
        }
    }

    let mut cycles = FxHashMap::default();
    let mut pulse_counts = [0, 0];

    let mut button_presses = 0;
    let mut processing = VecDeque::new();

    if let ModuleType::Conjunction(values) = &modules.get(prev_rx).unwrap().0 {
        for &sender in values.iter().map(|(name, _)| name) {
            cycles.insert(sender, None);
        }
    }

    loop {
        button_presses += 1;

        processing.push_back(("button", false, "broadcaster"));

        while let Some((sender, pulse, module)) = processing.pop_front() {
            if module == prev_rx && pulse {
                let value = cycles.get_mut(sender).unwrap();

                if value.is_none() {
                    *value = Some(button_presses);
                }

                if cycles.iter().all(|(_, cycles)| cycles.is_some()) && button_presses > 1000 {
                    break;
                }
            }

            if button_presses <= 1000 {
                pulse_counts[pulse as usize] += 1;
            }

            let Some((kind, targets)) = modules.get_mut(module) else {
                continue;
            };

            match kind {
                ModuleType::BroadCaster => {
                    for &mut target in targets {
                        processing.push_back((module, pulse, target));
                    }
                }
                ModuleType::Conjunction(state) => {
                    *state.get_mut(sender).unwrap() = pulse;

                    let pulse_value = if state.iter().all(|(_, &last_pulse)| last_pulse) {
                        false
                    } else {
                        true
                    };

                    for &mut target in targets {
                        processing.push_back((module, pulse_value, target));
                    }
                }
                ModuleType::FlipFlop(state) if !pulse => {
                    *state = !*state;

                    for &mut target in targets {
                        processing.push_back((module, *state, target));
                    }
                }
                _ => {}
            }
        }

        if cycles.iter().all(|(_, cycles)| cycles.is_some()) && button_presses > 1000 {
            break;
        }
    }

    let part_2 = cycles
        .into_values()
        .fold(1, |prev, current| lcm_single(prev, current.unwrap()));

    (pulse_counts[0] * pulse_counts[1], part_2)
}

#[derive(Debug)]
enum ModuleType<'a> {
    BroadCaster,
    Conjunction(FxHashMap<&'a str, bool>),
    FlipFlop(bool),
}
