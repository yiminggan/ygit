use std::collections::HashMap;

fn shortest_edit(str1: &[String], str2: &[String]) -> Vec<HashMap<isize, isize>> {
    let n: isize = str1.len() as isize;
    let m: isize = str2.len() as isize;
    let max: isize = n + m;
    let mut v: HashMap<isize, isize> = HashMap::new();
    v.insert(1,0);
    let mut trace: Vec<HashMap<isize, isize>> = Vec::new();

    for d in 0..=max {
        // let mut current_v = v.clone();
        trace.push(v.clone());
        for k in (-d..=d+1).step_by(2) {
            let go_down: bool = if k == -d { 
                true
            } else if k == d as isize {
                false
            } else {
                *v.get(&(k-1)).unwrap_or(&-1) < *v.get(&(k+1)).unwrap_or(&-1)
            };
            let mut x = if go_down {
                *v.get(&(k+1)).unwrap_or(&-1)
            } else{
                *v.get(&(k-1)).unwrap_or(&-1) + 1

            };
            let mut y = x - k;
            
            // print!("x={} y={} k={}\n", x, y, k);
            while (x < n) && (y < m) && (str1[x as usize] == str2[y as usize]) {
                x += 1;
                y += 1;
            }
            if x >= n && y >= m {
                return trace;
            }
            else {
                v.insert(k, x);
            }
        }
    }
    trace
}

fn backtrack(trace: &Vec<HashMap<isize, isize>>, n: isize, m: isize) -> Vec<(isize, isize, isize, isize)> {
    let mut script: Vec<(isize, isize, isize, isize)> = Vec::new();
    let mut x: isize = n;
    let mut y: isize = m;
    for d in (0..trace.len() as isize).rev() {
        let v: &HashMap<isize, isize> = &trace[d as usize];
        let k: isize = x- y;
        let go_down: bool = if k == -d { 
            true
        } else if k == d as isize {
            false
        } else {
            *v.get(&(k-1)).unwrap_or(&-1) < *v.get(&(k+1)).unwrap_or(&-1)
        };
        let mut prev_k = k -1;
        if go_down{
            prev_k = k + 1;
        }
        let prev_x = *v.get(&prev_k).unwrap();
        let prev_y = prev_x - prev_k;
        while x > prev_x && y > prev_y{
            script.push((x-1, y-1, x, y));
            x -= 1;
            y -= 1;
        }

        if d > 0 {
            script.push((prev_x, prev_y, x, y));
        }
        x = prev_x;
        y = prev_y;
    }
    script.reverse();
    script
}

#[derive(Debug)]
pub enum EditMethod{
    Insert,
    Delete,
    Keep,
}
#[derive(Debug)]
pub struct Edit {
    method: EditMethod,
    content: String,
}

pub fn diff(a_lines: &[String], b_lines: &[String]) -> Vec<Edit> {
    let mut diff_edits: Vec<Edit> = Vec::new();
    let trace: Vec<HashMap<isize, isize>> = shortest_edit(a_lines, b_lines);
    let scripts: Vec<(isize, isize, isize, isize)> = backtrack(&trace, a_lines.len() as isize, b_lines.len() as isize);
    for (prev_x, prev_y, x, y) in scripts {
        if x == prev_x {
            diff_edits.push(Edit { method: EditMethod::Insert, content: b_lines[prev_y as usize].clone() });
        } else if y == prev_y {
            diff_edits.push(Edit { method: EditMethod::Delete, content: a_lines[prev_x as usize].clone() });
        } else {
            diff_edits.push(Edit { method: EditMethod::Keep, content: a_lines[prev_x as usize].clone() });
        }
    }
    diff_edits
}