use crate::graph::{Graph, WeightedGraph};
use crate::heap::BinaryHeap;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone)]
struct DijkstraNode<T, W> {
    vertex: T,
    distance: W,
}

impl<T, W: PartialEq> PartialEq for DijkstraNode<T, W> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T, W: PartialEq> Eq for DijkstraNode<T, W> {}

impl<T, W: PartialOrd> PartialOrd for DijkstraNode<T, W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl<T, W: Ord> Ord for DijkstraNode<T, W> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

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

fn dfs_recursive<T>(graph: &Graph<T>, vertex: &T, visited: &mut HashSet<T>, result: &mut Vec<T>)
where
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

    fn dfs_cycle<T>(graph: &Graph<T>, vertex: &T, colors: &mut HashMap<T, Color>) -> bool
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
        if colors[vertex] == Color::White && dfs_cycle(graph, vertex, &mut colors) {
            return true;
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
        if !visited.contains(vertex) && dfs_cycle(graph, vertex, None, &mut visited) {
            return true;
        }
    }

    false
}

pub fn dijkstra<T, W>(graph: &WeightedGraph<T, W>, start: &T) -> HashMap<T, W>
where
    T: Clone + Eq + Hash,
    W: Clone + PartialOrd + Ord + Default + std::ops::Add<Output = W>,
{
    let mut distances: HashMap<T, W> = HashMap::new();
    let mut visited: HashSet<T> = HashSet::new();
    let mut heap = BinaryHeap::max_heap();

    if !graph.has_vertex(start) {
        return distances;
    }

    distances.insert(start.clone(), W::default());
    heap.push(DijkstraNode {
        vertex: start.clone(),
        distance: W::default(),
    });

    while let Some(current_node) = heap.pop() {
        if visited.contains(&current_node.vertex) {
            continue;
        }

        visited.insert(current_node.vertex.clone());

        if let Some(neighbors) = graph.neighbors(&current_node.vertex) {
            for edge in neighbors {
                if !visited.contains(&edge.to) {
                    let new_dist = current_node.distance.clone() + edge.weight.clone();

                    let should_update = distances
                        .get(&edge.to)
                        .map_or(true, |existing_dist| new_dist < *existing_dist);

                    if should_update {
                        distances.insert(edge.to.clone(), new_dist.clone());
                        heap.push(DijkstraNode {
                            vertex: edge.to.clone(),
                            distance: new_dist,
                        });
                    }
                }
            }
        }
    }

    distances
}

pub fn dijkstra_with_path<T, W>(
    graph: &WeightedGraph<T, W>,
    start: &T,
) -> (HashMap<T, W>, HashMap<T, T>)
where
    T: Clone + Eq + Hash,
    W: Clone + PartialOrd + Ord + Default + std::ops::Add<Output = W>,
{
    let mut distances: HashMap<T, W> = HashMap::new();
    let mut previous: HashMap<T, T> = HashMap::new();
    let mut visited: HashSet<T> = HashSet::new();
    let mut heap = BinaryHeap::max_heap();

    if !graph.has_vertex(start) {
        return (distances, previous);
    }

    distances.insert(start.clone(), W::default());
    heap.push(DijkstraNode {
        vertex: start.clone(),
        distance: W::default(),
    });

    while let Some(current_node) = heap.pop() {
        if visited.contains(&current_node.vertex) {
            continue;
        }

        visited.insert(current_node.vertex.clone());

        if let Some(neighbors) = graph.neighbors(&current_node.vertex) {
            for edge in neighbors {
                if !visited.contains(&edge.to) {
                    let new_dist = current_node.distance.clone() + edge.weight.clone();

                    let should_update = distances
                        .get(&edge.to)
                        .map_or(true, |existing_dist| new_dist < *existing_dist);

                    if should_update {
                        distances.insert(edge.to.clone(), new_dist.clone());
                        previous.insert(edge.to.clone(), current_node.vertex.clone());
                        heap.push(DijkstraNode {
                            vertex: edge.to.clone(),
                            distance: new_dist,
                        });
                    }
                }
            }
        }
    }

    (distances, previous)
}

pub fn reconstruct_dijkstra_path<T>(previous: &HashMap<T, T>, start: &T, end: &T) -> Option<Vec<T>>
where
    T: Clone + Eq + Hash,
{
    if start == end {
        return Some(vec![start.clone()]);
    }

    let mut path = Vec::new();
    let mut current = end.clone();

    while current != *start {
        path.push(current.clone());
        match previous.get(&current) {
            Some(prev) => current = prev.clone(),
            None => return None,
        }
    }

    path.push(start.clone());
    path.reverse();
    Some(path)
}

pub fn dijkstra_shortest_path<T, W>(
    graph: &WeightedGraph<T, W>,
    start: &T,
    end: &T,
) -> (Option<W>, Option<Vec<T>>)
where
    T: Clone + Eq + Hash,
    W: Clone + PartialOrd + Ord + Default + std::ops::Add<Output = W>,
{
    let (distances, previous) = dijkstra_with_path(graph, start);

    let distance = distances.get(end).cloned();
    let path = reconstruct_dijkstra_path(&previous, start, end);

    (distance, path)
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

    #[test]
    fn test_dijkstra_basic() {
        let mut graph = WeightedGraph::directed();
        graph.add_edge(1, 2, 10);
        graph.add_edge(1, 3, 5);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 4, 2);

        let distances = dijkstra(&graph, &1);

        assert_eq!(distances.get(&1), Some(&0));
        assert_eq!(distances.get(&2), Some(&10));
        assert_eq!(distances.get(&3), Some(&5));
        assert_eq!(distances.get(&4), Some(&7));
    }

    #[test]
    fn test_dijkstra_shortest_path() {
        let mut graph = WeightedGraph::directed();
        graph.add_edge(1, 2, 4);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 4, 3);
        graph.add_edge(3, 2, 1);
        graph.add_edge(3, 4, 5);

        let (distance, path) = dijkstra_shortest_path(&graph, &1, &4);

        assert_eq!(distance, Some(6));
        assert_eq!(path, Some(vec![1, 3, 2, 4]));
    }

    #[test]
    fn test_dijkstra_no_path() {
        let mut graph = WeightedGraph::directed();
        graph.add_edge(1, 2, 5);
        graph.add_vertex(3);

        let distances = dijkstra(&graph, &1);
        assert!(!distances.contains_key(&3));

        let (distance, path) = dijkstra_shortest_path(&graph, &1, &3);
        assert_eq!(distance, None);
        assert_eq!(path, None);
    }

    #[test]
    fn test_dijkstra_same_vertex() {
        let mut graph = WeightedGraph::directed();
        graph.add_vertex(1);

        let (distance, path) = dijkstra_shortest_path(&graph, &1, &1);
        assert_eq!(distance, Some(0));
        assert_eq!(path, Some(vec![1]));
    }

    #[test]
    fn test_dijkstra_with_strings() {
        let mut graph = WeightedGraph::directed();
        graph.add_edge("A", "B", 25);
        graph.add_edge("A", "C", 10);
        graph.add_edge("B", "D", 15);
        graph.add_edge("C", "D", 35);

        let distances = dijkstra(&graph, &"A");

        assert_eq!(distances.get(&"A"), Some(&0));
        assert_eq!(distances.get(&"B"), Some(&25));
        assert_eq!(distances.get(&"C"), Some(&10));
        assert_eq!(distances.get(&"D"), Some(&40));
    }

    #[test]
    fn test_dijkstra_complex_graph() {
        let mut graph = WeightedGraph::directed();

        graph.add_edge(0, 1, 4);
        graph.add_edge(0, 7, 8);
        graph.add_edge(1, 2, 8);
        graph.add_edge(1, 7, 11);
        graph.add_edge(2, 3, 7);
        graph.add_edge(2, 8, 2);
        graph.add_edge(2, 5, 4);
        graph.add_edge(3, 4, 9);
        graph.add_edge(3, 5, 14);
        graph.add_edge(4, 5, 10);
        graph.add_edge(5, 6, 2);
        graph.add_edge(6, 7, 1);
        graph.add_edge(6, 8, 6);
        graph.add_edge(7, 8, 7);

        let distances = dijkstra(&graph, &0);

        assert_eq!(distances.get(&0), Some(&0));
        assert_eq!(distances.get(&1), Some(&4));
        assert_eq!(distances.get(&2), Some(&12));
        assert_eq!(distances.get(&5), Some(&16));
        assert_eq!(distances.get(&6), Some(&18));
    }
}
