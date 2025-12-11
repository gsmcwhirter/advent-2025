use std::{collections::{HashMap, HashSet, VecDeque}, process::exit};
use microlp::{Problem, OptimizationDirection, ComparisonOp, Variable};

#[path = "./util.rs"]
mod util;

#[derive (Clone, Debug)]
struct LightGoal {
    goal: i64,
    size: usize,
    buttons: Vec<Vec<usize>>,
    // // map from light index to buttons that control it
    button_idx: HashMap<usize, HashSet<usize>>
}

#[derive (Clone, Debug)]
struct JoltageGoal {
    goal: Vec<i64>,
    buttons: Vec<Vec<usize>>,
    button_idx: HashMap<usize, HashSet<usize>>
}

pub fn a(inf: &String) {
    let mut light_goals: Vec<LightGoal> = vec![];

    let mut tmp = String::new();

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut light_goal = LightGoal{goal: 0, size: 0, buttons: vec![], button_idx: HashMap::new()};

            for c in line.chars() {
                match c {
                    ' ' => {}, // do nothing
                    '[' => tmp.clear(),
                    ']' => {
                        for (i, lc) in tmp.chars().enumerate() {
                            light_goal.size += 1;
                            if lc == '#' {
                                light_goal.goal = toggle_light(&light_goal.goal, i);
                            }
                        }
                    },
                    '(' => tmp.clear(),
                    ')' => {
                        // let button_idx = light_goal.buttons.len();
                        let mut button: Vec<usize> = vec![];
                        for idx_str in tmp.split(","){
                            match idx_str.parse::<usize>() {
                                Ok(v) => {
                                    button.push(v);
                                }
                                Err(_) => {
                                    println!("Malformed number: {}", idx_str);
                                    exit(1);
                                }
                            }
                        }
                        button.sort();
                        light_goal.buttons.push(button);
                    },
                    '{' => {
                        break;
                    },
                    _ => tmp.push(c),
                }
            }

            for (button_num, button) in light_goal.buttons.iter().enumerate() {
                for light_num in button.iter() {
                    match light_goal.button_idx.get_mut(&light_num) {
                        Some(bi) => {
                            bi.insert(button_num);
                        }
                        None => {
                            let mut s: HashSet<usize> = HashSet::new();
                            s.insert(button_num);
                            light_goal.button_idx.insert(*light_num, s);
                        }
                    }
                }
            }

            light_goals.push(light_goal);
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    let mut total_pushes = 0;
    for light_goal in light_goals.iter() {
        println!("Goal: {}", light_repr(&light_goal.goal, light_goal.size));
        println!("Buttons: {:?}", light_goal.buttons);

        let initial_state = BFSStateLights{
            prior_steps: 0,
            lights: 0,
            next_idx: next_bad_light(&0, &light_goal.goal, light_goal.size),
        };

        let mut queue: VecDeque<BFSStateLights> = VecDeque::new();
        queue.push_back(initial_state);

        let mut seen: HashSet<i64> = HashSet::new();

        while queue.len() > 0 {
            match bfs_lights(light_goal, &mut queue, &mut seen) {
                Some(res) => {
                    match res {
                        Ok(steps) => {
                            println!("Reached goal in {} steps", steps);
                            total_pushes += steps;
                            break;
                        }
                        Err(reason) => {
                            println!("{}", reason);
                            exit(1);
                        }
                    }
                }
                None => {}
            }
        }    
    }

    println!("Total: {}", total_pushes);
}

#[derive (Copy, Clone, Debug)]
struct BFSStateLights {
    prior_steps: i64,
    lights: i64,
    next_idx: usize
}

fn bfs_lights(goal: &LightGoal, queue: &mut VecDeque<BFSStateLights>, seen: &mut HashSet<i64>) -> Option<Result<i64, String>> {
    match queue.pop_front() {
        Some(state) => {
            if state.lights == goal.goal {
                return Some(Ok(state.prior_steps));
            }

            seen.insert(state.lights);

            match goal.button_idx.get(&state.next_idx) {
                Some(v) => {
                    for bidx in v.iter() {
                        let mut next_state = BFSStateLights{
                            prior_steps: state.prior_steps + 1,
                            lights: apply_button(&state.lights, &goal.buttons[*bidx]),
                            next_idx: state.next_idx
                        };

                        if seen.contains(&next_state.lights) {
                            continue
                        }

                        next_state.next_idx = next_bad_light(&next_state.lights, &goal.goal, goal.size);
                        queue.push_back(next_state);
                    }

                    return None
                }
                None => {
                    return Some(Err("No button to toggle idx".to_string()))
                }
            }
        }
        None => {
            return Some(Err("Empty queue".to_string()));
        }
    }
}

fn toggle_light(lights: &i64, light: usize) -> i64 {
    return lights ^ i64::pow(2, light.try_into().unwrap());
}

fn apply_button(lights: &i64, button: &Vec<usize>) -> i64 {
    let mut new_lights = *lights;
    for light in button.iter() {
        new_lights = toggle_light(&new_lights, *light);
    }

    return new_lights;
}

fn light_at(lights: &i64, light: usize) -> i64 {
    return *lights & i64::pow(2, light.try_into().unwrap())
}

fn light_on(lights: &i64, light: usize) -> bool {
    return light_at(lights, light) != 0;
}

fn light_repr(lights: &i64, light_len: usize) -> String {
    let mut repr = String::new();
    for i in 0..light_len {
        if light_on(lights, i) {
            repr.push('#');
        } else {
            repr.push('.');
        }
    }
    return repr;
}

fn next_bad_light(lights: &i64, target: &i64, len: usize) -> usize {
    for i in 0..len {
        if light_at(lights, i) != light_at(target, i) {
            return i;
        }
    }

    return len;
}

// #[derive (Clone, Debug)]
// struct BFSStateJoltage {
//     prior_steps: i64,
//     joltages: Vec<i64>,
//     dist_to_goal: i64
// }

// // this is backwards to make BinaryHeap easier
// impl Ord for BFSStateJoltage {
//     fn cmp(&self, other:&Self) -> Ordering {
//         match other.dist_to_goal.cmp(&self.dist_to_goal) {
//             Ordering::Equal => {
//                 return other.prior_steps.cmp(&self.prior_steps)
//             }
//             v => {
//                 return v
//             }
//         }
//     }
// }

// impl PartialOrd for BFSStateJoltage {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// } 

// impl PartialEq for BFSStateJoltage {
//     fn eq(&self, other: &Self) -> bool {
//         (self.dist_to_goal, self.prior_steps) == (other.dist_to_goal, other.prior_steps)
//     }
// }

// impl Eq for BFSStateJoltage{}

// struct SearchState {
//     step_limit: i64,
// }

// fn heap_joltages(goal: &JoltageGoal, search_state: &mut SearchState, heap: &mut BinaryHeap<BFSStateJoltage>, seen_at: &mut HashMap<String, i64>) -> Option<Result<i64, String>> {
    
//     match heap.pop() {
//         Some(state) => {
//             println!("\tState: {:?}, steps: {}", state.joltages, state.prior_steps);
//             if next_unequal_joltage(&state.joltages, &goal.goal) == goal.goal.len() {
//                 return Some(Ok(state.prior_steps));
//                 // search_state.step_limit = state.prior_steps;
//                 // return None
//             }

//             // if search_state.step_limit > 0 && state.prior_steps >= search_state.step_limit {
//             //     return None
//             // }

//             seen_at.insert(joltage_key(&state.joltages), state.prior_steps);

//             let candidates = joltage_max_candidates(&state.joltages, goal);

//             for next_idx in 0..state.joltages.len() {
//                 if state.joltages[next_idx] > goal.goal[next_idx] {
//                     return None
//                 }

//                 if state.joltages[next_idx] == goal.goal[next_idx] {
//                     continue
//                 }

//                 let mut btnset: Vec<(usize, &Vec<usize>, i64)> = vec![];
//                 for (bidx, ct) in &candidates[next_idx] {
//                     btnset.push((*bidx, &goal.buttons[*bidx], *ct));
//                 }
//                 btnset.sort_by(|(_, a, _), (_, b, _)| b.len().cmp(&a.len()));

//                 for (_, button, ct) in btnset.iter() {
//                     for c in (1..=*ct).rev() {
//                         let mut next_state = BFSStateJoltage{
//                             prior_steps: state.prior_steps + c,
//                             joltages: apply_joltage_button(&state.joltages, button, c),
//                             dist_to_goal: -1
//                         };
//                         next_state.dist_to_goal = dist(&next_state.joltages, &goal.goal);

//                         match seen_at.get(&joltage_key(&next_state.joltages)) {
//                             Some(seen_steps) => {
//                                 if *seen_steps < next_state.prior_steps {
//                                     continue
//                                 }
//                             }
//                             None => {}
//                         }

//                         let mut keep = true;
//                         for i in 0..next_state.joltages.len() {
//                             if next_state.joltages[i] > goal.goal[i] {
//                                 keep = false;
//                                 break;
//                             }
//                         }

//                         if keep {
//                             heap.push(next_state);
//                         }
//                     }
//                 }
//             }

//             return None
//         }
//         None => {
//             return Some(Err("Empty queue".to_string()));
//         }
//     }
// }

// fn dfs_joltages(goal: &JoltageGoal, state: &BFSStateJoltage, index_order: &Vec<usize>, seen_at: &mut HashMap<String, i64>, known_best_steps: i64, prefix: &str) -> Option<Result<i64, String>> {
//     println!("{}\tState: {:?}, steps: {}, best_steps: {}", prefix, state.joltages, state.prior_steps, known_best_steps);
//     if next_unequal_joltage(&state.joltages, &goal.goal) == goal.goal.len() {
//         println!("{}\tfound!", prefix);
//         return Some(Ok(state.prior_steps));
//     }

//     seen_at.insert(joltage_key(&state.joltages), state.prior_steps);

//     let candidates = joltage_max_candidates(&state.joltages, goal);

//     let mut best_steps = known_best_steps;

//     for next_idx in index_order {
//         if state.joltages[*next_idx] > goal.goal[*next_idx] {
//             println!("{}\tout of bounds!", prefix);
//             return None
//         }

//         if state.joltages[*next_idx] == goal.goal[*next_idx] {
//             continue
//         }

//         let mut btnset: Vec<(usize, &Vec<usize>, i64)> = vec![];
//         println!("{}\tCandidates: {:?}", prefix, &candidates[*next_idx]);
//         for (bidx, ct) in &candidates[*next_idx] {
//             btnset.push((*bidx, &goal.buttons[*bidx], *ct));
//         }
//         btnset.sort_by(|(_, a, _), (_, b, _)| b.len().cmp(&a.len()));

//         for (btn_idx, button, ct) in btnset.iter() {
//             println!("{}\t\tconsidering button {} ({:?}) with ct {}", prefix, btn_idx, button, ct);
//             for c in (1..=*ct).rev() {
//                 if best_steps > 0 && state.prior_steps + c >= best_steps {
//                     println!("{}\tskipping; too many steps", prefix);
//                     continue
//                 }

//                 let mut next_state = BFSStateJoltage{
//                     prior_steps: state.prior_steps + c,
//                     joltages: apply_joltage_button(&state.joltages, button, c),
//                     dist_to_goal: -1
//                 };
//                 next_state.dist_to_goal = dist(&next_state.joltages, &goal.goal);

//                 match seen_at.get(&joltage_key(&next_state.joltages)) {
//                     Some(seen_steps) => {
//                         if *seen_steps < next_state.prior_steps {
//                             continue
//                         }
//                     }
//                     None => {}
//                 }

//                 let mut keep = true;
//                 for i in 0..next_state.joltages.len() {
//                     if next_state.joltages[i] > goal.goal[i] {
//                         keep = false;
//                         break;
//                     }
//                 }

//                 let mut should_break = false;
//                 if keep {
//                     println!("{}\trecursing", prefix);
//                     match dfs_joltages(goal, &next_state, index_order, seen_at, best_steps, (prefix.to_owned()+"\t").as_str()) {
//                         Some(res) => {
//                             match res {
//                                 Ok(steps) => {
//                                     if best_steps == 0 || steps < best_steps {
//                                         best_steps = steps;
//                                         println!("{}\t\tnew best steps: {}", prefix, best_steps);
//                                     }
//                                     println!("{}\t\tbreaking button loop {}", prefix, btn_idx);
//                                     should_break = true;
//                                 }
//                                 Err(e) => {
//                                     return Some(Err(e));
//                                 }
//                             }
//                         },
//                         None => {}
//                     }
//                     println!("{}\tpost best_steps: {}", prefix, best_steps);
//                 }
//                 if should_break {
//                     break;
//                 }
//             }
//         }
//     }

//     if best_steps > 0 {
//         return Some(Ok(best_steps));
//     }
//     return None
// }

// fn dist(j1: &Vec<i64>, j2: &Vec<i64>) -> i64 {
//     let mut dist = 0;

//     for i in 0..j1.len() {
//         dist += i64::abs(j1[i] - j2[i])
//     }

//     return dist;
// }

// fn joltage_key(j: &Vec<i64>) -> String {
//     let mut s = String::new();
//     for v in j.iter() {
//         s.extend(v.to_string().chars());
//     }
//     return s;
// }

// fn next_unequal_joltage(j1: &Vec<i64>, j2: &Vec<i64>) -> usize {
//     if j1.len() != j2.len() {
//         println!("Comparing mismatched lengths");
//         exit(1);
//     }

//     for i in 0..j1.len() {
//         if j1[i] != j2[i] {
//             return i
//         }
//     }

//     return j1.len()
// }

// fn apply_joltage_button(joltages: &Vec<i64>, button: &Vec<usize>, ct: i64) -> Vec<i64> {
//     let mut new_joltages = joltages.clone();
//     for i in button.iter() {
//         new_joltages[*i] += ct;
//     }
//     return new_joltages;
// }

// fn joltage_max_candidates(joltages: &Vec<i64>, goal: &JoltageGoal) -> Vec<HashMap<usize, i64>> {
//     let mut candidates: Vec<HashMap<usize, i64>> = vec![];

//     for i in 0..joltages.len() {
//         let mut c = HashMap::new();
//         let missing = goal.goal[i] - joltages[i];
        
//         if missing > 0 {
//             match goal.button_idx.get(&i) {
//                 Some(v) => {
//                     for bidx in v.iter() {
//                         match c.get(bidx) {
//                             Some(ct) => {
//                                 c.insert(*bidx, ct+missing);
//                             }
//                             None => {
//                                 c.insert(*bidx, missing);
//                             }
//                         }
//                     }
//                 }
//                 None => {
//                     println!("No buttons for index");
//                     exit(1);
//                 }
//             }
//         }

//         candidates.push(c);
//     }

//     return candidates
// }

pub fn b(inf: &String) {
    let mut joltage_goals: Vec<JoltageGoal> = vec![];

    let mut tmp = String::new();

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut joltage_goal = JoltageGoal{goal: vec![], buttons: vec![], button_idx: HashMap::new()};

            for c in line.chars() {
                match c {
                    ' ' => {}, // do nothing
                    '[' => tmp.clear(),
                    ']' => {},
                    '(' => tmp.clear(),
                    ')' => {
                        // let button_idx = light_goal.buttons.len();
                        let mut button: Vec<usize> = vec![];
                        for idx_str in tmp.split(","){
                            match idx_str.parse::<usize>() {
                                Ok(v) => {
                                    button.push(v);
                                }
                                Err(_) => {
                                    println!("Malformed number: {}", idx_str);
                                    exit(1);
                                }
                            }
                        }
                        button.sort();
                        joltage_goal.buttons.push(button);
                    },
                    '{' => tmp.clear(),
                    '}' => {
                        for jv_str in tmp.split(","){
                            match jv_str.parse::<i64>() {
                                Ok(v) => {
                                    joltage_goal.goal.push(v);
                                }
                                Err(_) => {
                                    println!("Malformed number: {}", jv_str);
                                    exit(1);
                                }
                            }
                        }
                    }
                    _ => tmp.push(c),
                }
            }

            for (button_num, button) in joltage_goal.buttons.iter().enumerate() {
                for light_num in button.iter() {
                    match joltage_goal.button_idx.get_mut(&light_num) {
                        Some(bi) => {
                            bi.insert(button_num);
                        }
                        None => {
                            let mut s: HashSet<usize> = HashSet::new();
                            s.insert(button_num);
                            joltage_goal.button_idx.insert(*light_num, s);
                        }
                    }
                }
            }

            joltage_goals.push(joltage_goal);
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    let mut total_pushes: f64 = 0.0;
    for joltage_goal in joltage_goals.iter() {
        println!("Goal: {:?}", joltage_goal.goal);
        println!("Buttons: {:?}", joltage_goal.buttons);

        let mut initial_joltages: Vec<i64> = vec![];
        for _ in 0..joltage_goal.goal.len() {
            initial_joltages.push(0);
        }

        // let mut initial_state = BFSStateJoltage{
        //     joltages: initial_joltages,
        //     prior_steps: 0,
        //     dist_to_goal: -1,
        // };
        // initial_state.dist_to_goal = dist(&initial_state.joltages, &joltage_goal.goal);

        // let mut heap: BinaryHeap<BFSStateJoltage> = BinaryHeap::new();
        // heap.push(initial_state);

        // let mut seen: HashMap<String, i64> = HashMap::new();

        // let mut search_state = SearchState{step_limit: 0};

        let mut indexes: Vec<usize> = (0..joltage_goal.goal.len()).collect();
        indexes.sort_by(|a, b| joltage_goal.goal[*a].cmp(&joltage_goal.goal[*b]));
        println!("Index order: {:?}", indexes);

        // match dfs_joltages(joltage_goal, &initial_state, &indexes, &mut seen, 0, "") {
        //     Some(res) => {
        //         match res {
        //             Ok(steps) => {
        //                 println!("Reached goal in {} steps", steps);
        //                 total_pushes += steps;
        //             }
        //             Err(reason) => {
        //                 println!("{}", reason);
        //                 exit(1);
        //             }
        //         }
        //     }
        //     None => {}
        // }

        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let mut vars: Vec<Variable> = vec![];
        for _ in joltage_goal.buttons.iter() {
            vars.push(problem.add_integer_var(1.0, (0, joltage_goal.goal[indexes[indexes.len()-1]].try_into().unwrap())));
        }

        for idx in indexes {
            let mut constraint: Vec<(Variable, f64)> = vec![];
            match joltage_goal.button_idx.get(&idx) {
                Some(buttons) => {
                    for button in buttons.iter() {
                        constraint.push((vars[*button], 1.0))
                    }
                }
                None => {}
            }

            problem.add_constraint(constraint, ComparisonOp::Eq, joltage_goal.goal[idx] as f64);
        }

        // Optimal value is 7, achieved at x = 1 and y = 3.
        let solution = problem.solve().unwrap();
        println!("Solution: {}", solution.objective());
        total_pushes += solution.objective();
        
    }

    println!("Total: {}", total_pushes);
}