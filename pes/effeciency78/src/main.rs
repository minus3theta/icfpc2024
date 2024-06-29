use screwsat::solver::*;
use screwsat::util;
use std::io::{self, BufRead};
use std::vec;

#[derive(Clone, Debug)]
struct Clause {
    literals: Vec<i32>,
}

impl Clause {
    fn is_satisfied(&self, assignment: &Vec<Option<bool>>) -> bool {
        for &literal in &self.literals {
            let var_index = literal.abs() as usize - 1;
            match assignment[var_index] {
                Some(value) => {
                    if (literal > 0 && value) || (literal < 0 && !value) {
                        return true;
                    }
                }
                None => continue,
            }
        }
        false
    }
    fn get_unassigned_literal(&self, assignment: &Vec<Option<bool>>) -> Vec<i32> {
        let mut unassigned_literals = Vec::new();
        for &literal in &self.literals {
            let var_index = literal.abs() as usize - 1;
            if assignment[var_index].is_none() {
                unassigned_literals.push(literal);
            }
        }
        unassigned_literals
    }
}

fn parse_input() -> (usize, Vec<Clause>) {
    let stdin = io::stdin();
    let mut clauses = Vec::new();
    let mut max_var = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let literals: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        max_var = literals
            .iter()
            .map(|&lit| lit.abs() as usize)
            .max()
            .unwrap_or(max_var)
            .max(max_var);

        if !literals.is_empty() {
            clauses.push(Clause { literals });
        }
    }

    (max_var, clauses)
}
fn main() {
    let (num_vars, clauses) = parse_input();
    let mut assignment = vec![None; num_vars];

    for i in (0..num_vars).rev() {
        eprintln!("Trying to set v{}", i + 1);
        assignment[i] = Some(false);
        let mut solver = Solver::default();
        let mut ok = true;
        let mut has_clause = false;
        for clause in &clauses {
            if clause.is_satisfied(&assignment) {
                continue;
            }
            let unassigned_literals = clause.get_unassigned_literal(&assignment);
            if unassigned_literals.is_empty() {
                ok = false;
                break;
            }
            let mut clause_for_solver = Vec::new();
            for l in unassigned_literals {
                clause_for_solver.push(Lit::from(l));
            }
            has_clause = true;
            solver.add_clause(&clause_for_solver);
        }
        if ok && has_clause {
            let status = solver.solve(None);
            if status != Status::Sat {
                ok = false;
            }
        }
        if !ok {
            assignment[i] = Some(true);
        }
    }

    for (i, &value) in assignment.iter().enumerate() {
        println!("v{}: {}", i + 1, value.unwrap());
    }
    let mut result: i64 = 0;
    for &value in assignment.iter().rev() {
        result = 2 * result;
        if value.unwrap() {
            result += 1;
        }
    }
    println!("Result: {}", result);
}
