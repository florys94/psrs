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

fn depth_first_search_recusive(graph: &Vec<Vec<(usize, usize)>>, visited: &mut Vec<bool>, dist: &mut Vec<usize>, start: usize) {
    // procedure DFS_recusive(G, v) is
    // label v as discovered
    // for all directed edges from v to w that are in G.adjacentEdges(v) do
    //     if vertex w is not labeled as discovered then
    //         recursively call DFS_recusive(G, w)
    visited[start] = true;

    for i in 0..graph[start].len() {
        let (end, d) = graph[start][i];

        if !visited[end] {
            dist[end] = dist[start] + d;
            depth_first_search_recusive(graph, visited, dist, end);
        }
    }
}

fn depth_first_search_iterative(graph: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, start: usize) {
    // procedure DFS_iterative(G, v) is
    // let S be a stack
    // S.push(v)
    // while S is not empty do
    //     v = S.pop()
    //     if v is not labeled as discovered then
    //         label v as discovered
    //         for all edges from v to w in G.adjacentEdges(v) do 
    //             S.push(w)

    let mut stack: Vec<usize> = Vec::new();
    let mut visited = vec![false; graph.len()];

    stack.push(start);
    
    while !stack.is_empty() {
        let start = stack.pop().unwrap();

        if !visited[start] {
            visited[start] = true;
            for i in 0..graph[start].len() {
                let (end, d) = graph[start][i];
            
                if !visited[end] {
                    dist[end] = dist[start] + d;
                    stack.push(end);
                }
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
            let d = input[i + 1] as usize;

            tree[start].push((end, d));
        }
    }

    let mut dist: Vec<usize> = vec![0; v + 1];
    let mut start = 1;

    breadth_first_search(&tree, &mut dist, start);

    // let mut visited: Vec<bool> = vec![false; v + 1];
    // depth_first_search_recusive(&tree, &mut visited, &mut dist, start);
    // depth_first_search_iterative(&tree, &mut dist, start);

    for i in 2..=v {
        if dist[i] > dist[start] {
            start = i;
        }
    }

    dist = vec![0; v + 1];

    breadth_first_search(&tree, &mut dist, start);

    // visited = vec![false; v + 1];
    // depth_first_search_recusive(&tree, &mut visited, &mut dist, start);
    // depth_first_search_iterative(&tree, &mut dist, start);

    dist.sort();

    println!("{}", dist[v]);
}
