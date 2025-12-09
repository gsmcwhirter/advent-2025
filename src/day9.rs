use std::{collections::HashMap, collections::HashSet, process::exit};

#[path = "./util.rs"]
mod util;

#[derive (Copy, Clone, Debug)]
struct Tile {
    x: i64,
    y: i64,
}

#[derive (Copy, Clone, Debug)]
struct Edge {
    start: i64,
    end: i64
}

#[derive (Copy, Clone, Debug)]
struct Rectangle {
    c1: usize,
    c2: usize,
    area: i64
}

#[derive (Copy, Clone, Debug)]
struct Corners {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64
}

pub fn a(inf: &String) {
    let mut tiles: Vec<Tile> = vec![];
    let mut corners = Corners{xmin: -1, xmax: -1, ymin: -1, ymax: -1};
    let mut areas: Vec<Rectangle> = vec![];

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut t = Tile{x: -1, y: -1};
            match line.split_once(",") {
                Some((xstr,ystr)) => {
                    match xstr.parse::<i64>() {
                        Ok(x) => {
                            t.x = x;

                            if corners.xmin == -1 || x < corners.xmin {
                                corners.xmin = x;
                            }
                            if x > corners.xmax {
                                corners.xmax = x;
                            }
                        }
                        Err(_) => {
                            println!("Could not parse coordinate '{}'.", xstr);
                            exit(1);
                        }
                    }

                    match ystr.parse::<i64>() {
                        Ok(y) => {
                            t.y = y;

                            if corners.ymin == -1 || y < corners.ymin {
                                corners.ymin = y;
                            }
                            if y > corners.ymax {
                                corners.ymax = y;
                            }
                        }
                        Err(_) => {
                            println!("Could not parse coordinate '{}'.", ystr);
                            exit(1);
                        }
                    }

                    tiles.push(t);
                }
                None => {
                    println!("Could not parse line '{}'.", line);
                    exit(1);
                }
            }
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    corners.xmin -= 1;
    corners.xmax += 1;
    corners.ymin -= 1;
    corners.ymax += 1;

    println!("Corners: {:?}", corners);

    for (i, t1) in tiles.iter().enumerate() {
        for (j, t2) in tiles[i+1..].iter().enumerate() {
            areas.push(Rectangle { c1: i, c2: i+j+1, area: area(t1, t2) });
        }
    }

    areas.sort_by(|a1, a2| a2.area.cmp(&a1.area));
    println!("Largest area: {}", areas[0].area);

    // for t in tiles.iter_mut() {
    //     t.score = i64::min(t.x - corners.xmin, corners.xmax - t.x) * i64::min(t.y - corners.ymin, corners.ymax - t.y)
    // }

    // tiles.sort_by(|t1, t2| t1.score.cmp(&t2.score));
    // let c1 = tiles[0];
    // println!("C1: {:?}", c1);

    // for t in tiles.iter_mut() {
    //     t.score = area(&c1, t)
    // }
    // tiles.sort_by(|t1, t2| t2.score.cmp(&t1.score));
    // let c2 = tiles[0];
    // println!("C2: {:?}", c2);

    // println!("Area: {}", area(&c1, &c2))
}

fn area(t1: &Tile, t2: &Tile) -> i64 {
    return (i64::abs(t1.x - t2.x) + 1) * (i64::abs(t1.y - t2.y)+1)
}

pub fn b(inf: &String) {
    let mut tiles: Vec<Tile> = vec![];
    let mut corners = Corners{xmin: -1, xmax: -1, ymin: -1, ymax: -1};
    let mut areas: Vec<Rectangle> = vec![];

    let mut tiles_idx: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut vert_edges: HashMap<i64, Vec<Edge>> = HashMap::new();
    let mut hori_edges: HashMap<i64, Vec<Edge>> = HashMap::new();

    let mut interior_cache: HashMap<(i64, i64), bool> = HashMap::new();

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let mut t = Tile{x: -1, y: -1};
            match line.split_once(",") {
                Some((xstr,ystr)) => {
                    match xstr.parse::<i64>() {
                        Ok(x) => {
                            t.x = x;

                            if corners.xmin == -1 || x < corners.xmin {
                                corners.xmin = x;
                            }
                            if x > corners.xmax {
                                corners.xmax = x;
                            }
                        }
                        Err(_) => {
                            println!("Could not parse coordinate '{}'.", xstr);
                            exit(1);
                        }
                    }

                    match ystr.parse::<i64>() {
                        Ok(y) => {
                            t.y = y;

                            if corners.ymin == -1 || y < corners.ymin {
                                corners.ymin = y;
                            }
                            if y > corners.ymax {
                                corners.ymax = y;
                            }
                        }
                        Err(_) => {
                            println!("Could not parse coordinate '{}'.", ystr);
                            exit(1);
                        }
                    }

                    match tiles_idx.get_mut(&t.x) {
                        Some(v) => {
                            v.insert(t.y);
                        }
                        None => {
                            let mut hs = HashSet::new();
                            hs.insert(t.y);
                            tiles_idx.insert(t.x, hs);
                        }
                    }
                    
                    if tiles.len() > 0 {
                        let prev = tiles[tiles.len()-1];
                        if prev.x == t.x {
                            match vert_edges.get_mut(&t.x) {
                                Some(v) => {
                                    v.push(Edge{start: i64::min(prev.y, t.y), end: i64::max(prev.y, t.y)});
                                }
                                None => {
                                    let mut v = vec![];
                                    v.push(Edge{start: i64::min(prev.y, t.y), end: i64::max(prev.y, t.y)});
                                    vert_edges.insert(t.x, v); 
                                }
                            }
                        } else {
                            match hori_edges.get_mut(&t.y) {
                                Some(v) => {
                                    v.push(Edge{start: i64::min(prev.x, t.x), end: i64::max(prev.x, t.x)});
                                }
                                None => {
                                    let mut v = vec![];
                                    v.push(Edge{start: i64::min(prev.x, t.x), end: i64::max(prev.x, t.x)});
                                    hori_edges.insert(t.y, v); 
                                }
                            }
                        }
                    }

                    tiles.push(t);
                }
                None => {
                    println!("Could not parse line '{}'.", line);
                    exit(1);
                }
            }
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    corners.xmin -= 1;
    corners.xmax += 1;
    corners.ymin -= 1;
    corners.ymax += 1;

    println!("Corners: {:?}", corners);

    // for y in corners.ymin..=corners.ymax {
    //     for x in corners.xmin..=corners.xmax {
    //         match tiles_idx.get(&x) {
    //             Some(v) => {
    //                 if v.contains(&y) {
    //                     print!("X");
    //                 } else {
    //                     print!(".");
    //                 }
    //             }
    //             None => {
    //                 print!(".");
    //             }
    //         }   
    //     }
    //     println!();
    // }

    for (i, t1) in tiles.iter().enumerate() {
        for (j, t2) in tiles[i+1..].iter().enumerate() {
            areas.push(Rectangle { c1: i, c2: i+j+1, area: area(t1, t2) });
        }
    }

    areas.sort_by(|a1, a2| a2.area.cmp(&a1.area));
    for r in areas.iter() {
        if valid_rectangle(&tiles, r, &corners, &vert_edges, &hori_edges, &mut interior_cache) {
            let start = usize::min(r.c1, r.c2);
            let ts = tiles[start];
            let end = usize::max(r.c1, r.c2);
            let te = tiles[end];

            println!("\tContained tiles:");
            for x in i64::min(ts.x, te.x)..=i64::max(ts.x, te.x) {
                match tiles_idx.get(&x) {
                    Some(v) => {
                        for y in i64::min(ts.y, te.y)..=i64::max(ts.y, te.y) {
                            // if tiles_set.contains(&(x, y)) {
                            //     println!("\t\t{:?}", Tile{x: x, y: y})
                            // }
                            if v.contains(&y) {
                                println!("\t\t{:?}", Tile{x: x, y: y})
                            }
                        }
                    }
                    None => {}
                }
            }
            println!("Largest area: {}", r.area);
            break
        }
    }
}

fn valid_rectangle(tiles: &Vec<Tile>, r: &Rectangle, corners: &Corners, vert_edges: &HashMap<i64, Vec<Edge>>, hori_edges: &HashMap<i64, Vec<Edge>>, interior_cache: &mut HashMap<(i64, i64), bool>) -> bool {
    let start = usize::min(r.c1, r.c2);
    let ts = tiles[start];
    let end = usize::max(r.c1, r.c2);
    let te = tiles[end];

    if ts.x == te.x || ts.y == te.y {
        return true
    }

    let xdir: i64;
    let ydir: i64;
    if ts.x < te.x {
        xdir = 1; // rectangle extends to the right from start
    } else {
        xdir = -1; // rectangle extends to the left from start
    }

    if ts.y < te.y {
        ydir = 1; // rectangle extends down from start
    } else {
        ydir = -1; // rectangle extends up from start
    }

    println!("Checking {:?} ({}) -> {:?} ({}) (area: {}, xd: {}, yd: {})", ts, start, te, end, r.area, xdir, ydir);
 
    // for i in 0..tiles.len() {
    //     let tn = tiles[i];
        
    //     if encroaches(&ts, &te, &tn, xdir, ydir) {
    //         println!("\tencroachment: {:?} ({})", tn, i);
    //         return false
    //     }

    //     let tnn;
    //     if i+1 == tiles.len() {
    //         tnn = tiles[0];
    //     } else {
    //         tnn = tiles[i+1];
    //     }

    //     if encroaches_next(&ts, &te, &tn, &tnn, xdir, ydir) {
    //         println!("\tencroachment: {:?}->{:?} ({})", tn, tnn, i);
    //         return false
    //     }
    // }


    // for x in i64::min(ts.x, te.x)..=i64::max(ts.x, te.x) {
    //     for y in i64::min(ts.y, te.y)..=i64::max(ts.y, te.y) {
    //         if !is_interior(&Tile{x: x, y: y}, corners, vert_edges, interior_cache) {
    //             return false
    //         }
    //     }    
    // }

    let interior_pt = Tile{x: (ts.x + te.x) / 2, y: (ts.y + te.y) / 2};
    if !is_interior_x(&interior_pt, corners, vert_edges) {
        return false;
    }
    if !is_interior_y(&interior_pt, corners, hori_edges) {
        return false;
    }

    let minx = i64::min(ts.x, te.x);
    let maxx = i64::max(ts.x, te.x);
    let miny = i64::min(ts.y, te.y);
    let maxy = i64::max(ts.y, te.y);
    for x in minx+1..maxx {
        match vert_edges.get(&x) {
            Some(v) => {
                for e in v.iter() {
                    if e.end <= miny {
                        continue
                    }

                    if e.start >= maxy {
                        continue
                    }

                    println!("\tIntersected edge x={}, y={:?}", x, e);
                    return false
                }
            }
            None => {}
        }
    }

    // match vert_edges.get(&minx) {
    //     Some(v) => {
    //         for e in v.iter() {
    //             if e.end < miny {
    //                 continue
    //             }

    //             if e.start > maxy {
    //                 continue
    //             }

    //             if (e.start <= miny && e.end < maxy) || (e.start > miny && e.end >= maxy) {
    //                 return false
    //             }
    //         }
    //     }
    //     None => {}
    // }

    // match vert_edges.get(&maxx) {
    //     Some(v) => {
    //         for e in v.iter() {
    //             if e.end < miny {
    //                 continue
    //             }

    //             if e.start > maxy {
    //                 continue
    //             }

    //             if (e.start <= miny && e.end < maxy) || (e.start > miny && e.end >= maxy) {
    //                 return false
    //             }
    //         }
    //     }
    //     None => {}
    // }

    for y in miny+1..maxy {
        match hori_edges.get(&y) {
            Some(v) => {
                for e in v.iter() {
                    if e.end <= minx {
                        continue
                    }

                    if e.start >= maxx {
                        continue
                    }

                    println!("\tIntersected edge y={}, x={:?}", y, e);
                    return false
                }
            }
            None => {}
        } 
    }

    // match hori_edges.get(&miny) {
    //     Some(v) => {
    //         for e in v.iter() {
    //             if e.end < minx {
    //                 continue
    //             }

    //             if e.start > maxx {
    //                 continue
    //             }

    //             if (e.start <= minx && e.end < maxx) || (e.start > minx && e.end >= maxx) {
    //                 return false
    //             }
    //         }
    //     }
    //     None => {}
    // } 

    // match hori_edges.get(&maxy) {
    //     Some(v) => {
    //         for e in v.iter() {
    //             if e.end < minx {
    //                 continue
    //             }

    //             if e.start > maxx {
    //                 continue
    //             }

    //             if (e.start <= minx && e.end < maxx) || (e.start > minx && e.end >= maxx) {
    //                 return false
    //             }
    //         }
    //     }
    //     None => {}
    // } 

    // for i in start+1..end {
        
    // }

    // for i in end+1..tiles.len() {
    //     let t = tiles[i];
    //     if encroaches(&ts, &te, &t, xdir, ydir) {
    //         return false
    //     }
    // }

    // for i in 0..start {
    //     let t = tiles[i];
    //     if encroaches(&ts, &te, &t, xdir, ydir) {
    //         return false
    //     }
    // }

    return true
}

fn is_interior_x(pt: &Tile, corners: &Corners, vert_edges: &HashMap<i64, Vec<Edge>>) -> bool {
    println!("\tChecking interior point: {:?}", pt);
    let mut intersections = 0;
    for x in pt.x..corners.xmax {
        match vert_edges.get(&x) {
            Some(v) => {
                for e in v.iter() {
                    // we're imaginging that the y is actually bumped a bit inwards
                    if e.start <= pt.y && e.end > pt.y {
                        intersections += 1;
                    }
                }
            }
            None => {}
        }
    }

    return intersections % 2 != 0;
}

fn is_interior_y(pt: &Tile, corners: &Corners, hori_edges: &HashMap<i64, Vec<Edge>>) -> bool {
    println!("\tChecking interior point: {:?}", pt);
    let mut intersections = 0;
    for y in pt.y..corners.ymax {
        match hori_edges.get(&y) {
            Some(v) => {
                for e in v.iter() {
                    // we're imaginging that the x is actually bumped a bit inwards
                    if e.start <= pt.x && e.end > pt.x {
                        // println!("\t\tIntersected x={} y={:?}", x, e);
                        intersections += 1;
                    }
                }
            }
            None => {}
        }
    }

    return intersections % 2 != 0;
}

// fn encroaches(ts: &Tile, te: &Tile, tn: &Tile, xdir: i64, ydir: i64) -> bool {
//     if te.x*xdir > tn.x*xdir && tn.x*xdir > ts.x*xdir && ts.y*ydir < tn.y*ydir && tn.y*ydir < te.y*ydir {
//         return true
//     }

//     if tn.x == ts.x && tn.y == te.y {
//         return true
//     }

//     return false
// }

// fn encroaches_next(ts: &Tile, te: &Tile, tn: &Tile, tnn: &Tile, xdir: i64, ydir: i64) -> bool {
//     if te.x*xdir > tn.x*xdir && tn.x*xdir > ts.x*xdir && ((te.y == tn.y && ts.y == tnn.y) || (ts.y == tn.y && te.y == tnn.y))  {
//         return true
//     }

//     if ts.y*ydir < tn.y*ydir && tn.y*ydir < te.y*ydir && ((te.x == tn.x && ts.x == tnn.x) || (ts.x == tn.x && te.x == tnn.x))  {
//         return true
//     }

//     return false
// }