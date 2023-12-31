use fxhash::FxHashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let (rules_str, parts_str) = input.split_once("\n\n").unwrap();

    let mut workflows = FxHashMap::default();

    for line in rules_str.lines() {
        let (name, line) = line.split_once('{').unwrap();

        let line = line.trim_end_matches('}');

        let rule_vec = line
            .split(',')
            .map(|instruction| {
                if instruction.contains(':') {
                    let component = instruction.chars().next().unwrap();
                    let less = instruction
                        .chars()
                        .skip(1)
                        .next()
                        .map(|c| c == '<')
                        .unwrap();

                    let (condition, target) = instruction.split_once(':').unwrap();
                    let condition = condition[2..].parse().unwrap();

                    Rule::Condition {
                        component,
                        less,
                        condition,
                        target,
                    }
                } else {
                    Rule::Goto {
                        target: instruction,
                    }
                }
            })
            .collect::<Vec<_>>();

        assert!(workflows.insert(name, rule_vec).is_none());
    }

    let mut part_1 = 0;

    for part in parts_str.lines() {
        let part = part.trim_matches(['{', '}']);

        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for (index, component) in part.split(',').enumerate() {
            let value = component
                .split_once('=')
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            match index {
                0 => x = value,
                1 => m = value,
                2 => a = value,
                3 => s = value,
                _ => unreachable!(),
            }
        }

        let mut rule_name = "in";

        while rule_name != "A" && rule_name != "R" {
            let rules = workflows.get(&rule_name).unwrap();

            for rule in rules {
                match rule {
                    Rule::Condition {
                        component,
                        less,
                        condition,
                        target,
                    } => {
                        let value = match component {
                            'x' => x,
                            'm' => m,
                            'a' => a,
                            's' => s,
                            _ => unreachable!(),
                        };

                        if *less {
                            if value < *condition {
                                rule_name = *target;
                                break;
                            }
                        } else {
                            if value > *condition {
                                rule_name = *target;
                                break;
                            }
                        }
                    }
                    Rule::Goto { target } => {
                        rule_name = *target;
                        break;
                    }
                }
            }
        }

        if rule_name == "A" {
            part_1 += x + m + a + s;
        }
    }

    let mut part_2 = 0;

    let mut seeds = vec![(
        "in",
        [1..4001usize, 1..4001usize, 1..4001usize, 1..4001usize],
        workflows.get("in").map(|rules| rules.as_slice()),
    )];

    while let Some((name, mut components, rules)) = seeds.pop() {
        if name == "A" {
            part_2 += components
                .into_iter()
                .map(|range| range.len())
                .product::<usize>();
            continue;
        } else if name == "R" {
            continue;
        }

        let rule = rules.unwrap().first().unwrap();

        match rule {
            Rule::Condition {
                component,
                less,
                condition,
                target,
            } => {
                let change_index = match component {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };

                let value = components[change_index].clone();

                let rule = workflows.get(target).map(|val| val.as_slice());

                if *less {
                    let lower_start = usize::min(value.start, *condition);
                    let lower_end = usize::min(value.end, *condition);

                    let upper_start = usize::max(value.start, *condition);
                    let upper_end = usize::max(value.end, *condition);

                    if !(lower_start..lower_end).is_empty() {
                        components[change_index] = lower_start..lower_end;
                        seeds.push((target, components.clone(), rule));
                    }

                    if !(upper_start..upper_end).is_empty() {
                        components[change_index] = upper_start..upper_end;
                        seeds.push((name, components, rules.map(|rules| &rules[1..])));
                    }
                } else {
                    let lower_start = usize::min(value.start, *condition + 1);
                    let lower_end = usize::min(value.end, *condition + 1);

                    let upper_start = usize::max(value.start, *condition + 1);
                    let upper_end = usize::max(value.end, *condition + 1);

                    if !(lower_start..lower_end).is_empty() {
                        components[change_index] = lower_start..lower_end;
                        seeds.push((name, components.clone(), rules.map(|rules| &rules[1..])));
                    }

                    if !(upper_start..upper_end).is_empty() {
                        components[change_index] = upper_start..upper_end;
                        seeds.push((target, components, rule));
                    }
                }
            }
            Rule::Goto { target } => {
                let rule = workflows.get(target).map(|val| val.as_slice());
                seeds.push((target, components, rule));
            }
        }
    }

    (part_1, part_2)
}

#[derive(Clone, Copy)]
enum Rule<'a> {
    Condition {
        component: char,
        less: bool,
        condition: usize,
        target: &'a str,
    },
    Goto {
        target: &'a str,
    },
}
