use std::{process::exit};

#[path = "./util.rs"]
mod util;

struct Cell {
    val: char,
    adj: i32,
}

pub fn a(inf: &String) {
    let mut grid: Vec<Vec<Cell>> = vec![];

    if let Ok(lines) = util::read_lines(inf) {
        for row in lines.flatten() {
            if row.trim() == "" {
                continue
            }

            grid.push(row.chars().map(|x| -> Cell { Cell{val: x, adj: 0}} ).collect());
        }

        let num_rows = grid.len();
        if num_rows == 0 {
            println!("No rows!");
            exit(1);
        }

        let num_cols = grid[0].len();

        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j].val == '@' {
                    grid_adjust_at(&mut grid, i, j, 1);
                }
            }
        }

        let (accessible, _) = accessible_in_grid(&grid);

        println!("Accessible: {}", accessible)
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}

fn grid_adjust_at(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize, val: i32) {
    let num_rows = grid.len();
    if num_rows == 0 {
        println!("No rows!");
        exit(1);
    }

    let num_cols = grid[0].len();

    if row > 0 {
        grid[row-1][col].adj += val;

        if col > 0 {
            grid[row-1][col-1].adj += val;
        }

        if col < num_cols - 1 {
            grid[row-1][col+1].adj += val;
        }
    }

    if row < num_rows - 1 {
        grid[row+1][col].adj += val;

        if col > 0 {
            grid[row+1][col-1].adj += val;
        }

        if col < num_cols - 1 {
            grid[row+1][col+1].adj += val;
        }
    }

    if col > 0 {
        grid[row][col-1].adj += val;
    }

    if col < num_cols - 1 {
        grid[row][col+1].adj += val;
    }
}

fn accessible_in_grid(grid: &Vec<Vec<Cell>>) -> (i64, Vec<(usize,usize)>) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut removable: Vec<(usize,usize)> = vec![];

    let mut accessible: i64 = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i][j].adj < 4 && grid[i][j].val == '@' {
                accessible += 1;
                removable.push((i, j));
            }
        }
    }

    return (accessible, removable);
}

fn adjust_from_removable(grid: &mut Vec<Vec<Cell>>, removables: Vec<(usize, usize)>) {
    for (i, j) in removables {
        grid_adjust_at(grid, i, j, -1);
        grid[i][j].val = 'x';
    }
}

pub fn b(inf: &String) {
    let mut grid: Vec<Vec<Cell>> = vec![];

    if let Ok(lines) = util::read_lines(inf) {
        for row in lines.flatten() {
            if row.trim() == "" {
                continue
            }

            grid.push(row.chars().map(|x| -> Cell { Cell{val: x, adj: 0}} ).collect());
        }

        let num_rows = grid.len();
        if num_rows == 0 {
            println!("No rows!");
            exit(1);
        }

        let num_cols = grid[0].len();

        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j].val == '@' {
                    grid_adjust_at(&mut grid, i, j, 1);
                }
            }
        }

        let mut accessible: i64 = 0;
        let mut new_access: i64;
        let mut removables: Vec<(usize, usize)>;
        
        (new_access, removables) = accessible_in_grid(&grid);
        while new_access != 0 {
            println!("Found {} removable", new_access);
            accessible += new_access;
            adjust_from_removable(&mut grid, removables);
            (new_access, removables) = accessible_in_grid(&grid);
        }

        println!("Accessible: {}", accessible)
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}