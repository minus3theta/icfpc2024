fn solve_impl(
    target: &mut std::collections::HashSet<(i32, i32)>,
    pos: &(i32, i32),
    vel: &(i32, i32),
) -> anyhow::Result<String> {
    if target.is_empty() {
        return Ok(String::new());
    }
    for i in 0..9 {
        let dx = i % 3 - 1;
        let dy = i / 3 - 1;
        let nx = pos.0 + vel.0 + dx;
        let ny = pos.1 + vel.1 + dy;
        if target.contains(&(nx, ny)) {
            let next_pos = (nx, ny);
            let next_vel = (vel.0 + dx, vel.1 + dy);
            target.remove(&(nx, ny));
            if let Ok(next_res) = solve_impl(target, &next_pos, &next_vel) {
                let mut a = String::from(char::from_digit(i as u32 + 1, 10).unwrap());
                a.push_str(&next_res);
                return Ok(a);
            }
            target.insert((nx, ny));
        }
    }
    anyhow::bail!("Failed to solve.")
}

fn solve(input: &String) -> anyhow::Result<String> {
    let mut target = std::collections::HashSet::new();
    for l in input.lines() {
        let mut it = l.split_whitespace();
        let x = it.next().unwrap().parse::<i32>()?;
        let y = it.next().unwrap().parse::<i32>()?;
        target.insert((x, y));
    }
    let pos = (0, 0);
    let vel = (0, 0);
    solve_impl(&mut target, &pos, &vel)
}

fn main() {
    for i in 1..=25 {
        // "../data/spaceship/spaceship{i}.rs" が存在すれば、ファイルを読み込みsolveを呼ぶ
        if let Ok(input) = std::fs::read_to_string(format!("./data/spaceship/spaceship{}.in", i)) {
            if let Ok(res) = solve(&input) {
                println!("Solved {}: {}", i, res);
            }
        }
    }
}
