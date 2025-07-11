use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("src/input.txt")?;

    println!("{content}");

    let mut lines = content.lines();

    let n: u8 = lines.next().unwrap().parse().unwrap();

    let mut graph: Vec<Vec<u8>> = vec![vec![]; n as usize];

    lines.for_each(|line| {
        let components: Vec<u8> = line
            .split_whitespace()
            .map(|c| c.parse::<u8>().unwrap() - 1)
            .collect();
        graph[components[0] as usize] = components[1..].to_vec();
    });

    println!("{n}");
    println!("{graph:?}");

    println!("{:?}", topo_sort(&graph));

    Ok(())
}

fn topo_sort(graph: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut ans = vec![];

    fn dfs(u: usize, graph: &Vec<Vec<u8>>, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
        visited[u] = true;
        for &e in &graph[u] {
            if !visited[e as usize] {
                dfs(e as usize, graph, visited, ans);
            }
        }
        ans.push(u);
    }

    for v in 0..graph.len() {
        if !visited[v] {
            dfs(v, graph, &mut visited, &mut ans);
        }
    }

    ans.iter().rev().map(|e| e + 1).collect()
}
