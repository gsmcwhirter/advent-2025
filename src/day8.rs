use std::{collections::HashMap, collections::HashSet, process::exit};

#[path = "./util.rs"]
mod util;

#[derive (Copy, Clone, Debug)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
    group: usize
}

#[derive (Copy, Clone, Debug)]
struct Distance {
    from: usize,
    to: usize,
    dist: i64
}

pub fn a(inf: &String) {
    let mut junctions: Vec<Junction> = vec![];
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut distances: Vec<Distance> = vec![];

    groups.insert(0, HashSet::new()); // dummy group

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut j = Junction{x: 0, y: 0, z: 0, group: 0};
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() != 3 {
                println!("Malformed input line: {}", line);
                exit(1)
            }

            match parts[0].parse::<i64>() {
                Ok(v) => {
                    j.x = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[0]);
                    exit(1)
                }
            }

            match parts[1].parse::<i64>() {
                Ok(v) => {
                    j.y = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[1]);
                    exit(1)
                }
            }

            match parts[2].parse::<i64>() {
                Ok(v) => {
                    j.z = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[2]);
                    exit(1)
                }
            }

            junctions.push(j);
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    for (i, junction1) in junctions.iter().enumerate() {
        println!("Distances from {} {:?}", i, junction1);
        for (j, junction2) in junctions[i+1..].iter().enumerate() {
            let d = dist(junction1, junction2);
            distances.push(Distance{from: i, to: j+i+1, dist: d});
            println!("\tto {} {:?}: {}", j+i+1, junction2, d);
        }
    }

    distances.sort_by(|x, y| x.dist.cmp(&y.dist));

    for d in distances[..1000].iter() {

        let j1 = junctions[d.from];
        let j2 = junctions[d.to];

        println!("Distance: {:?} (j1={:?}, j2={:?})", d, j1, j2);

        if j1.group == 0 && j2.group == 0 {
            let mut g = HashSet::new();
            let group_id = groups.len();
            println!("Creating new group {}", group_id);
            add_to_group(&mut g, group_id, &mut junctions, d.from);
            add_to_group(&mut g, group_id, &mut junctions, d.to);
            groups.insert(group_id, g);
        } else if j1.group == 0 {
            println!("Adding j1 to group {}", j2.group);
            match groups.get_mut(&j2.group) {
                Some(v) => {
                    add_to_group(v, j2.group, &mut junctions, d.from);
                    println!("group {} is now {:?}", j2.group, v);
                }
                None => {
                    println!("Group {} does not exist (how did we get here?)", j2.group);
                    exit(1);
                }
            }
        } else if j2.group == 0 {
            println!("Adding j2 to group {}", j1.group);
            match groups.get_mut(&j1.group) {
                Some(v) => {
                    add_to_group(v, j1.group, &mut junctions, d.to);
                    println!("group {} is now {:?}", j1.group, v);
                }
                None => {
                    println!("Group {} does not exist (how did we get here?)", j1.group);
                    exit(1);
                }
            }
        } else if j1.group == j2.group {
            println!("Same group, nothing to do");
            // nothing to do
        } else {
            println!("Merging group {} and group {}", j1.group, j2.group);
            merge_groups(&mut groups, &mut junctions, j1.group, j2.group);
        }
    }

    let mut group_list: Vec<&HashSet<usize>> = groups.values().collect();
    group_list.sort_by(|g1, g2| g2.len().cmp(&g1.len()));
    for g in group_list.iter() {
        println!("Group of size {}", g.len());
    }
    let product = group_list[0].len() * group_list[1].len() * group_list[2].len();
    println!("Product: {}", product);
}

fn dist(a: &Junction, b: &Junction) -> i64 {
    return (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);
}

fn merge_groups(groups: &mut HashMap<usize, HashSet<usize>>, junctions: &mut Vec<Junction>, group1_id: usize, group2_id: usize) -> usize {
    // keep group 1, toss group 2
    let mut group1;
    match groups.get_mut(&group1_id) {
        Some(v) => {
            group1 = v.clone();
        }
        None => {
            println!("Group {} does not exist", group1_id);
            exit(1);
        }
    }

    if group1_id == group2_id {
        return group1.len();
    }

    println!("group {} was {:?}", group1_id, group1);

    match groups.get(&group2_id) {
        Some(group2) => {
            println!("group {} was {:?}", group2_id, group2);
            for item_id in group2.iter() {
                add_to_group(&mut group1, group1_id, junctions, *item_id);
            }
        }
        None => {
            println!("Group {} does not exist", group1_id);
            exit(1);
        }
    }

    println!("group {} is now {:?}", group1_id, group1);
    let l = group1.len();
    groups.insert(group1_id, group1);

    match groups.get_mut(&group2_id) {
        Some(v) => {
            v.clear();
        }
        None => {
            println!("Group {} does not exist (how did we get here?)", group2_id);
            exit(1);
        }
    }

    return l;
}

fn add_to_group(group: &mut HashSet<usize>, group_id: usize, junctions: &mut Vec<Junction>, item_id: usize) -> usize {
    if item_id >= junctions.len() {
        println!("Item {} does not exist", item_id);
        exit(1);
    }

    junctions[item_id].group = group_id;
    _ = group.insert(item_id);

    return group.len();
}

pub fn b(inf: &String) {
    let mut junctions: Vec<Junction> = vec![];
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut distances: Vec<Distance> = vec![];

    groups.insert(0, HashSet::new()); // dummy group

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut j = Junction{x: 0, y: 0, z: 0, group: 0};
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() != 3 {
                println!("Malformed input line: {}", line);
                exit(1)
            }

            match parts[0].parse::<i64>() {
                Ok(v) => {
                    j.x = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[0]);
                    exit(1)
                }
            }

            match parts[1].parse::<i64>() {
                Ok(v) => {
                    j.y = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[1]);
                    exit(1)
                }
            }

            match parts[2].parse::<i64>() {
                Ok(v) => {
                    j.z = v;
                }
                Err(_) => {
                    println!("Malformed input part: {}", parts[2]);
                    exit(1)
                }
            }

            junctions.push(j);
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    for (i, junction1) in junctions.iter().enumerate() {
        // println!("Distances from {} {:?}", i, junction1);
        for (j, junction2) in junctions[i+1..].iter().enumerate() {
            let d = dist(junction1, junction2);
            distances.push(Distance{from: i, to: j+i+1, dist: d});
            // println!("\tto {} {:?}: {}", j+i+1, junction2, d);
        }
    }

    distances.sort_by(|x, y| x.dist.cmp(&y.dist));
    let target_size = junctions.len();

    for d in distances.iter() {

        let j1 = junctions[d.from];
        let j2 = junctions[d.to];

        println!("Distance: {:?} (j1={:?}, j2={:?})", d, j1, j2);

        if j1.group == 0 && j2.group == 0 {
            let mut g = HashSet::new();
            let group_id = groups.len();
            println!("Creating new group {}", group_id);
            add_to_group(&mut g, group_id, &mut junctions, d.from);
            let new_size = add_to_group(&mut g, group_id, &mut junctions, d.to);
            if new_size == target_size {
                println!("Last x product: {}", j1.x * j2.x);
                break;
            }
            groups.insert(group_id, g);
        } else if j1.group == 0 {
            println!("Adding j1 to group {}", j2.group);
            match groups.get_mut(&j2.group) {
                Some(v) => {
                    let new_size = add_to_group(v, j2.group, &mut junctions, d.from);
                    println!("group {} is now {:?}", j2.group, v);
                    if new_size == target_size {
                        println!("Last x product: {}", j1.x * j2.x);
                        break;
                    }
                }
                None => {
                    println!("Group {} does not exist (how did we get here?)", j2.group);
                    exit(1);
                }
            }
        } else if j2.group == 0 {
            println!("Adding j2 to group {}", j1.group);
            match groups.get_mut(&j1.group) {
                Some(v) => {
                    let new_size = add_to_group(v, j1.group, &mut junctions, d.to);
                    println!("group {} is now {:?}", j1.group, v);
                    if new_size == target_size {
                        println!("Last x product: {}", j1.x * j2.x);
                        break;
                    }
                }
                None => {
                    println!("Group {} does not exist (how did we get here?)", j1.group);
                    exit(1);
                }
            }
        } else if j1.group == j2.group {
            println!("Same group, nothing to do");
            // nothing to do
        } else {
            println!("Merging group {} and group {}", j1.group, j2.group);
            let new_size = merge_groups(&mut groups, &mut junctions, j1.group, j2.group);
            if new_size == target_size {
                println!("Last x product: {}", j1.x * j2.x);
                break;
            }
        }
    }
}