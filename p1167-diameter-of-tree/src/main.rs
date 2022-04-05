use std::io;
use std::collections::VecDeque;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();

        std::io::stdin().read_line(&mut inner).unwrap();

        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn breadth_first_search(tree: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, start: usize) {
    let mut visited = vec![false; tree.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start] = true;

    while !queue.is_empty() {
        let start = queue.pop_front().unwrap();

        for i in 0..tree[start].len() {
            let (end, d) = tree[start][i];
            
            if !visited[end] {
                dist[end] = dist[start] + d;
                queue.push_back(end);
                visited[end] = true;
            }
        }
    }
}

fn main() {
    let v: usize = 0;
    read!(v as usize);

    let mut tree: Vec<Vec<(usize, usize)>> = vec![Vec::new(); v + 1];
    let input: Vec<i32> = vec![];

    for _ in 0..v {
        read_vec!(input as i32);

        let start = input[0] as usize;

        for i in (1..input.len()).step_by(2) {
            if input[i] == -1 {
                break;
            }

            let end = input[i] as usize;
            let dist = input[i + 1] as usize;

            tree[start].push((end, dist));
        }
    }

    let mut dist: Vec<usize> = vec![0; v + 1];
    let mut start = 1;

    breadth_first_search(&tree, &mut dist, start);

    for i in 2..=v {
        if dist[i] > dist[start] {
            start = i;
        }
    }
    
    dist = vec![0; v + 1];

    breadth_first_search(&tree, &mut dist, start);

    dist.sort();

    println!("{}", dist[v]);
}
