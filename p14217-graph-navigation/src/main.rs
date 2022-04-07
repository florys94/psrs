use std::collections::VecDeque;
use std::io;

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

fn breadth_first_search(graph: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, start: usize) {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start] = true;

    while !queue.is_empty() {
        let start = queue.pop_front().unwrap();

        for i in 0..graph[start].len() {
            let (end, d) = graph[start][i];

            if !visited[end] {
                dist[end] = dist[start] + d;
                queue.push_back(end);
                visited[end] = true;
            }
        }
    }
}

fn main() {
    let input: Vec<usize> = Vec::new();

    read_vec!(input as usize);

    let (n, m) = (input[0], input[1]);
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n + 1];

    for _ in 0..m {
        read_vec!(input as usize);

        let (start, end) = (input[0], input[1]);

        graph[start].push((end, 1));
        graph[end].push((start, 1));
    }

    read_vec!(input as usize);

    let q = input[0];

    for _ in 0..q {
        let mut dist: Vec<usize> = vec![0; n + 1];
        read_vec!(input as usize);

        let (a, i, j) = (input[0], input[1], input[2]);

        if a == 1 {
            graph[i].push((j, 1));
            graph[j].push((i, 1));
        } else if a == 2 {
            let index = graph[i].iter().position(|&x| x == (j, 1)).unwrap();
            graph[i].remove(index);

            let index = graph[j].iter().position(|&x| x == (i, 1)).unwrap();
            graph[j].remove(index);
        }

        breadth_first_search(&graph, &mut dist, 1);

        print!("{} ", 0);
        for i in 2..=n {
            if dist[i] == 0 {
                print!("{} ", -1);
                continue;
            } 
            print!("{} ", dist[i]);
        }
        println!("");
    }
}
