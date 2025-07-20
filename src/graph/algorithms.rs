use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub fn bfs<T>(graph: &Graph<T>, start: &T) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    if !graph.has_vertex(start) {
        return result;
    }

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(vertex) = queue.pop_front() {
        result.push(vertex.clone());

        if let Some(neighbors) = graph.neighbors(&vertex) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    result
}

pub fn dfs<T>(graph: &Graph<T>, start: &T) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    if graph.has_vertex(start) {
        dfs_recursive(graph, start, &mut visited, &mut result);
    }

    result
}

fn dfs_recursive<T>(
    graph: &Graph<T>,
    vertex: &T,
    visited: &mut HashSet<T>,
    result: &mut Vec<T>,
) where
    T: Clone + Eq + Hash,
{
    visited.insert(vertex.clone());
    result.push(vertex.clone());

    if let Some(neighbors) = graph.neighbors(vertex) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs_recursive(graph, neighbor, visited, result);
            }
        }
    }
}

pub fn has_path<T>(graph: &Graph<T>, start: &T, end: &T) -> bool
where
    T: Clone + Eq + Hash,
{
    if !graph.has_vertex(start) || !graph.has_vertex(end) {
        return false;
    }

    if start == end {
        return true;
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(vertex) = queue.pop_front() {
        if let Some(neighbors) = graph.neighbors(&vertex) {
            for neighbor in neighbors {
                if neighbor == end {
                    return true;
                }

                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    false
}

pub fn shortest_path<T>(graph: &Graph<T>, start: &T, end: &T) -> Option<Vec<T>>
where
    T: Clone + Eq + Hash,
{
    if !graph.has_vertex(start) || !graph.has_vertex(end) {
        return None;
    }

    if start == end {
        return Some(vec![start.clone()]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent: HashMap<T, T> = HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(vertex) = queue.pop_front() {
        if let Some(neighbors) = graph.neighbors(&vertex) {
            for neighbor in neighbors {
                if neighbor == end {
                    parent.insert(neighbor.clone(), vertex.clone());
                    return Some(reconstruct_path(&parent, start, end));
                }

                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    parent.insert(neighbor.clone(), vertex.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    None
}

fn reconstruct_path<T>(parent: &HashMap<T, T>, start: &T, end: &T) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut path = Vec::new();
    let mut current = end.clone();

    while current != *start {
        path.push(current.clone());
        current = parent[&current].clone();
    }

    path.push(start.clone());
    path.reverse();
    path
}

pub fn connected_components<T>(graph: &Graph<T>) -> Vec<Vec<T>>
where
    T: Clone + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for vertex in graph.vertices() {
        if !visited.contains(vertex) {
            let component = dfs_component(graph, vertex, &mut visited);
            components.push(component);
        }
    }

    components
}

fn dfs_component<T>(graph: &Graph<T>, start: &T, visited: &mut HashSet<T>) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut component = Vec::new();
    let mut stack = vec![start.clone()];

    while let Some(vertex) = stack.pop() {
        if !visited.contains(&vertex) {
            visited.insert(vertex.clone());
            component.push(vertex.clone());

            if let Some(neighbors) = graph.neighbors(&vertex) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
    }

    component
}

pub fn is_cyclic<T>(graph: &Graph<T>) -> bool
where
    T: Clone + Eq + Hash,
{
    use crate::graph::adjacency_list::GraphType;

    match graph.graph_type() {
        GraphType::Directed => is_cyclic_directed(graph),
        GraphType::Undirected => is_cyclic_undirected(graph),
    }
}

fn is_cyclic_directed<T>(graph: &Graph<T>) -> bool
where
    T: Clone + Eq + Hash,
{
    #[derive(PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut colors: HashMap<T, Color> = HashMap::new();
    for vertex in graph.vertices() {
        colors.insert(vertex.clone(), Color::White);
    }

    fn dfs_cycle<T>(
        graph: &Graph<T>,
        vertex: &T,
        colors: &mut HashMap<T, Color>,
    ) -> bool
    where
        T: Clone + Eq + Hash,
    {
        colors.insert(vertex.clone(), Color::Gray);

        if let Some(neighbors) = graph.neighbors(vertex) {
            for neighbor in neighbors {
                match colors.get(neighbor) {
                    Some(Color::Gray) => return true,
                    Some(Color::White) => {
                        if dfs_cycle(graph, neighbor, colors) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }

        colors.insert(vertex.clone(), Color::Black);
        false
    }

    for vertex in graph.vertices() {
        if colors[vertex] == Color::White {
            if dfs_cycle(graph, vertex, &mut colors) {
                return true;
            }
        }
    }

    false
}

fn is_cyclic_undirected<T>(graph: &Graph<T>) -> bool
where
    T: Clone + Eq + Hash,
{
    let mut visited = HashSet::new();

    fn dfs_cycle<T>(
        graph: &Graph<T>,
        vertex: &T,
        parent: Option<&T>,
        visited: &mut HashSet<T>,
    ) -> bool
    where
        T: Clone + Eq + Hash,
    {
        visited.insert(vertex.clone());

        if let Some(neighbors) = graph.neighbors(vertex) {
            for neighbor in neighbors {
                if Some(neighbor) == parent {
                    continue;
                }

                if visited.contains(neighbor) || dfs_cycle(graph, neighbor, Some(vertex), visited) {
                    return true;
                }
            }
        }

        false
    }

    for vertex in graph.vertices() {
        if !visited.contains(vertex) {
            if dfs_cycle(graph, vertex, None, &mut visited) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let result = bfs(&graph, &1);
        assert_eq!(result[0], 1);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        let result = dfs(&graph, &1);
        assert_eq!(result[0], 1);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert!(result.contains(&4));
    }

    #[test]
    fn test_has_path() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert!(has_path(&graph, &1, &3));
        assert!(!has_path(&graph, &3, &1));
        assert!(has_path(&graph, &1, &1));
    }

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let path = shortest_path(&graph, &1, &4).unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], 1);
        assert_eq!(path[2], 4);
    }

    #[test]
    fn test_connected_components() {
        let mut graph = Graph::undirected();
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_vertex(5);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_cycle_detection() {
        let mut directed_cyclic = Graph::directed();
        directed_cyclic.add_edge(1, 2);
        directed_cyclic.add_edge(2, 3);
        directed_cyclic.add_edge(3, 1);
        assert!(is_cyclic(&directed_cyclic));

        let mut directed_acyclic = Graph::directed();
        directed_acyclic.add_edge(1, 2);
        directed_acyclic.add_edge(2, 3);
        assert!(!is_cyclic(&directed_acyclic));

        let mut undirected_cyclic = Graph::undirected();
        undirected_cyclic.add_edge(1, 2);
        undirected_cyclic.add_edge(2, 3);
        undirected_cyclic.add_edge(3, 1);
        assert!(is_cyclic(&undirected_cyclic));
    }
}