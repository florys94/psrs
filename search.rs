fn breadth_first_search(graph: Vec<Vec<usize, usize>>, dist: Vec<usize>, start: usize) {
    // Input: s as the source node

    // BFS (G, s)
    // let Q be queue.
    // Q.enqueue( s )
 
    // mark s as visited
    // while ( Q is not empty)
    // v = Q.dequeue( )
 
    // for all neighbors w of v in Graph G
    // if w is not visited
    // Q.enqueue( w )
    // mark w as visited

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