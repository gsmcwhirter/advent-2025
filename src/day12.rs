use std::{collections::{HashMap, HashSet}, process::exit};

#[path = "./util.rs"]
mod util;

#[derive (Clone, Debug)]
struct Shape{
    cells: Vec<(usize, usize)>,
    cell_idx: HashMap<(usize, usize), usize>,
    width: usize,
    height: usize,
    count: usize,
    op_idx: usize
}

impl Shape {
    fn fix_idx(&mut self) {
        self.cell_idx.clear();
        for (i, c) in self.cells.iter().enumerate() {
            self.cell_idx.insert(*c, i);
        }
    }

    fn reset_iter(&mut self) {
        self.op_idx = 0;
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cell_idx.contains_key(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn shift(&mut self, by_rows: usize, by_cols: usize) {
        for i in 0..self.cells.len() {
            self.cells[i] = (self.cells[i].0 + by_rows, self.cells[i].1 + by_cols);
        }
        self.fix_idx();
    }

    fn unshift(&mut self, by_rows: usize, by_cols: usize) {
        for i in 0..self.cells.len() {
            self.cells[i] = (self.cells[i].0 - by_rows, self.cells[i].1 - by_cols);
        }
        self.fix_idx();
    }
}

impl Iterator for Shape {
    type Item = Shape;

    // iterate over Dihedral(4) operation results
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.clone();

        if self.op_idx == 0 {
            self.op_idx += 1;
            return Some(next);
        }

        if self.op_idx >= 8 {
            return None
        }

        if self.op_idx < 8 {
            // println!("\t rotating");
            for _ in 0..self.op_idx.min(4) {
                // rotate 90deg
                for i in 0..next.cells.len() {
                    next.cells[i] = (next.cells[i].1, next.height - next.cells[i].0 - 1)
                }
                // shape gets adjusted also
                (next.width, next.height) = (next.height, next.width)
            }
            if self.op_idx >= 4 {
                // println!("\t flipping");
                // we already rotated back to the original,
                // now we flip over horizontal midline before more rotations
                for i in 0..next.cells.len() {
                    next.cells[i] = (next.height - next.cells[i].0 - 1, next.cells[i].1)
                }
            }
            for _ in self.op_idx.min(4)..self.op_idx {
                // rotate 90deg
                for i in 0..next.cells.len() {
                    next.cells[i] = (next.cells[i].1, next.height - next.cells[i].0 - 1)
                }
                // shape gets adjusted also
                (next.width, next.height) = (next.height, next.width)
            }
            self.op_idx += 1;
        } 

        next.fix_idx();
        return Some(next);
    }
}

#[derive (Clone, Debug)]
struct Target{
    width: usize,
    height: usize,
    shapes: Vec<i64>,
    filled: HashSet<(usize, usize)>
}

impl Target {
    fn spaces_free(&self) -> usize {
        return self.width * self.height - self.filled.len();
    }

    fn clear(&mut self) {
        self.filled.clear();
    }

    fn fits(&mut self, sh: &Shape) -> bool {
        return sh.cells.iter().all(|c| !self.filled.contains(c));
    }

    fn fill(&mut self, sh: &Shape) {
        for c in sh.cells.iter() {
            self.filled.insert(*c);
        }
    }

    fn remove(&mut self, sh: &Shape) {
        for c in sh.cells.iter() {
            self.filled.remove(c);
        }
    }

    fn first_empty_col(&self) -> usize {
        for c in 0..self.width {
            let mut empty = true;
            for entry in self.filled.iter() {
                if entry.1 == c {
                    empty = false;
                    break
                }
            }
            if empty {
                return c
            }
        }
        return self.width
    }

    fn first_empty_col_in_row(&self, row: usize) -> usize {
        for c in 0..self.width {
            if !self.filled.contains(&(row, c)) {
                return c
            }
        }
        return self.width
    }

    // fn first_empty_row_after_col(&self, col: usize) {
    //     for r in 0..self.height {
    //         let mut empty = true;
    //         for entry in self.filled.iter() {

    //         }
    //     }
    // }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.filled.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!()
    }
}

pub fn a(inf: &String) {

    let mut shapes: Vec<Shape> = vec![];
    let mut targets: Vec<Target> = vec![];

    let mut shape_row: usize = 0;
    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                shape_row = 0;
                continue
            }
            
            if shape_row > 0 {
                let shape_idx = shapes.len() - 1;
                if shapes[shape_idx].width == 0 {
                    shapes[shape_idx].width = line.len();
                }
                shapes[shape_idx].height = shape_row;
                for (shape_col, c) in line.chars().enumerate() {
                    if c == '#' {
                        let cell_idx = shapes[shape_idx].cells.len();
                        shapes[shape_idx].cell_idx.insert((shape_row-1, shape_col), cell_idx);
                        shapes[shape_idx].cells.push((shape_row-1, shape_col));
                        shapes[shape_idx].count += 1;
                    }
                }
                shape_row += 1;
                continue
            }

            match line.split_once(":") {
                Some((size, shapes_str)) => {
                    if shapes_str.trim() == "" {
                        shape_row = 1;
                        shapes.push(Shape{cells: vec![], width: 0, height: 0, count: 0, cell_idx: HashMap::new(), op_idx: 0});
                        continue
                    }

                    let mut target = Target{width: 0, height: 0, shapes: vec![], filled: HashSet::new()};
                    match size.split_once("x") {
                        Some((wstr, hstr)) => {
                            match wstr.parse::<usize>() {
                                Ok(w) => target.width = w,
                                Err(_) => {
                                    println!("Malformed shape width: {}", wstr);
                                    exit(1);
                                }
                            }
                            match hstr.parse::<usize>() {
                                Ok(h) => target.height = h,
                                Err(_) => {
                                    println!("Malformed shape height: {}", hstr);
                                    exit(1);
                                }
                            }
                        }
                        None => {
                            println!("Malformed target size: {}", size);
                            exit(1);
                        }
                    }

                    let shape_cts = shapes_str.split(" ");
                    for shape_ct in shape_cts {
                        if shape_ct == "" {
                            continue
                        }

                        match shape_ct.parse::<i64>() {
                            Ok(ct) => {
                                target.shapes.push(ct);
                            }
                            Err(_) => {
                                println!("Malformed shape count: {}", shape_ct);
                                exit(1);
                            }
                        }
                    }

                    targets.push(target);
                }
                None => {}
            }
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    for (i, shape) in shapes.iter_mut().enumerate() {
        println!("{}:", i);
        shape.reset_iter();
        for shapei in shape.into_iter() {
            shapei.print();
            println!();
        }
    }

    let mut fit_ct = 0;
    let mut shape_order: Vec<usize> = (0..shapes.len()).collect();
    shape_order.sort_by(|a, b| shapes[*a].count.cmp(&shapes[*b].count));

    for (i, target) in targets.iter_mut().enumerate() {
        let mut to_try = target.shapes.clone();
        if can_all_fit(target, &mut shapes, &mut to_try, &shape_order) {
            println!("Target {} can fit!", i);
            fit_ct += 1;
        } else {
            println!("Target {} can NOT fit!", i);
        }
        println!();
    }

    println!("Ok Targets: {}", fit_ct)
}

fn can_all_fit(target: &mut Target, shapes: &Vec<Shape>, to_try: &mut Vec<i64>, shape_order: &Vec<usize>) -> bool {
    target.print();
    println!("Remaining: {:?}", to_try);
    if to_try.iter().all(|v| *v == 0) {
        return true
    }

    let total_needed: i64 = to_try.iter().enumerate().map(|(idx, ct)| *ct * shapes[idx].count as i64).sum();
    if ((target.height * target.width) as i64) < total_needed {
        return false;
    }

    for shape_idx in shape_order.iter() {
        if to_try[*shape_idx] == 0 {
            continue
        }

        let mut shape = shapes[*shape_idx].clone();

        if target.spaces_free() < shape.count {
            continue
        }

        // let mut first_empty_row: usize = 0;
        let first_empty_col: usize = target.first_empty_col();

        shape.reset_iter();
        for mut variant in shape.into_iter() {
            if variant.width > target.width {
                continue
            }

            if variant.height > target.height {
                continue
            }

            for shift_row in 0..=(target.height - variant.height) {
                for shift_col in (shift_row..(shift_row+variant.height)).map(|row| target.first_empty_col_in_row(row)).min().unwrap()..=first_empty_col.min(target.width - variant.width) {
                    variant.shift(shift_row, shift_col);
                    if target.fits(&variant) {
                        target.fill(&variant);
                        to_try[*shape_idx] -= 1;
                        if can_all_fit(target, shapes, to_try, shape_order) {
                            return true;
                        }
                        to_try[*shape_idx] += 1;
                        target.remove(&variant);
                    }
                    variant.unshift(shift_row, shift_col);
                }
            }
        }
    }

    return false
}

pub fn b(inf: &String) {

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }
            
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

}