use crate::utils::{Clear, Size};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
}

pub struct Graph<T> {
    adjacency_list: HashMap<T, Vec<T>>,
    graph_type: GraphType,
    edge_count: usize,
}

impl<T> Graph<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(graph_type: GraphType) -> Self {
        Self {
            adjacency_list: HashMap::new(),
            graph_type,
            edge_count: 0,
        }
    }

    pub fn directed() -> Self {
        Self::new(GraphType::Directed)
    }

    pub fn undirected() -> Self {
        Self::new(GraphType::Undirected)
    }

    pub fn add_vertex(&mut self, vertex: T) -> bool {
        if self.adjacency_list.contains_key(&vertex) {
            false
        } else {
            self.adjacency_list.insert(vertex, Vec::new());
            true
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) -> bool {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        let from_list = self.adjacency_list.get_mut(&from).unwrap();
        if from_list.contains(&to) {
            return false;
        }

        from_list.push(to.clone());
        self.edge_count += 1;

        if self.graph_type == GraphType::Undirected && from != to {
            let to_list = self.adjacency_list.get_mut(&to).unwrap();
            to_list.push(from);
        }

        true
    }

    pub fn remove_vertex(&mut self, vertex: &T) -> bool {
        if !self.adjacency_list.contains_key(vertex) {
            return false;
        }

        let neighbors: Vec<T> = self.adjacency_list[vertex].clone();
        let outgoing_edges = neighbors.len();
        self.edge_count -= outgoing_edges;

        for (_, adj_list) in self.adjacency_list.iter_mut() {
            if let Some(pos) = adj_list.iter().position(|x| x == vertex) {
                adj_list.remove(pos);
                if self.graph_type == GraphType::Directed {
                    self.edge_count -= 1;
                }
            }
        }

        self.adjacency_list.remove(vertex);
        true
    }

    pub fn remove_edge(&mut self, from: &T, to: &T) -> bool {
        if let Some(from_list) = self.adjacency_list.get_mut(from) {
            if let Some(pos) = from_list.iter().position(|x| x == to) {
                from_list.remove(pos);
                self.edge_count -= 1;

                if self.graph_type == GraphType::Undirected && from != to {
                    if let Some(to_list) = self.adjacency_list.get_mut(to) {
                        if let Some(pos) = to_list.iter().position(|x| x == from) {
                            to_list.remove(pos);
                        }
                    }
                }
                return true;
            }
        }
        false
    }

    pub fn has_vertex(&self, vertex: &T) -> bool {
        self.adjacency_list.contains_key(vertex)
    }

    pub fn has_edge(&self, from: &T, to: &T) -> bool {
        self.adjacency_list
            .get(from)
            .map_or(false, |list| list.contains(to))
    }

    pub fn neighbors(&self, vertex: &T) -> Option<&Vec<T>> {
        self.adjacency_list.get(vertex)
    }

    pub fn vertices(&self) -> impl Iterator<Item = &T> {
        self.adjacency_list.keys()
    }

    pub fn edges(&self) -> EdgeIterator<T> {
        EdgeIterator::new(self)
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn graph_type(&self) -> &GraphType {
        &self.graph_type
    }

    pub fn degree(&self, vertex: &T) -> Option<usize> {
        self.adjacency_list.get(vertex).map(|list| list.len())
    }

    pub fn in_degree(&self, vertex: &T) -> Option<usize> {
        if !self.has_vertex(vertex) {
            return None;
        }

        let count = self
            .adjacency_list
            .values()
            .map(|list| list.iter().filter(|&v| v == vertex).count())
            .sum();

        Some(count)
    }

    pub fn out_degree(&self, vertex: &T) -> Option<usize> {
        self.degree(vertex)
    }
}

impl<T: Clone + Eq + Hash> Default for Graph<T> {
    fn default() -> Self {
        Self::directed()
    }
}

impl<T> Clear for Graph<T> {
    fn clear(&mut self) {
        self.adjacency_list.clear();
        self.edge_count = 0;
    }
}

impl<T> Size for Graph<T>
where
    T: Clone + Eq + Hash,
{
    fn len(&self) -> usize {
        self.vertex_count()
    }
}

impl<T: fmt::Debug + Clone + Eq + Hash> fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Graph")
            .field("adjacency_list", &self.adjacency_list)
            .field("graph_type", &self.graph_type)
            .field("edge_count", &self.edge_count)
            .finish()
    }
}

pub struct EdgeIterator<'a, T> {
    graph: &'a Graph<T>,
    vertex_iter: std::collections::hash_map::Keys<'a, T, Vec<T>>,
    current_vertex: Option<&'a T>,
    neighbor_index: usize,
}

impl<'a, T> EdgeIterator<'a, T>
where
    T: Clone + Eq + Hash,
{
    fn new(graph: &'a Graph<T>) -> Self {
        Self {
            graph,
            vertex_iter: graph.adjacency_list.keys(),
            current_vertex: None,
            neighbor_index: 0,
        }
    }
}

impl<'a, T> Iterator for EdgeIterator<'a, T>
where
    T: Clone + Eq + Hash,
{
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(vertex) = self.current_vertex {
                if let Some(neighbors) = self.graph.adjacency_list.get(vertex) {
                    if self.neighbor_index < neighbors.len() {
                        let neighbor = &neighbors[self.neighbor_index];
                        self.neighbor_index += 1;
                        return Some((vertex, neighbor));
                    }
                }
            }

            match self.vertex_iter.next() {
                Some(vertex) => {
                    self.current_vertex = Some(vertex);
                    self.neighbor_index = 0;
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_graph_is_empty() {
        let graph: Graph<i32> = Graph::directed();
        assert!(graph.is_empty());
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn add_vertices() {
        let mut graph = Graph::directed();
        
        assert!(graph.add_vertex(1));
        assert!(graph.add_vertex(2));
        assert!(!graph.add_vertex(1));

        assert_eq!(graph.vertex_count(), 2);
        assert!(graph.has_vertex(&1));
        assert!(graph.has_vertex(&2));
        assert!(!graph.has_vertex(&3));
    }

    #[test]
    fn directed_graph_edges() {
        let mut graph = Graph::directed();
        
        assert!(graph.add_edge(1, 2));
        assert!(graph.add_edge(2, 3));
        assert!(!graph.add_edge(1, 2));

        assert_eq!(graph.edge_count(), 2);
        assert!(graph.has_edge(&1, &2));
        assert!(graph.has_edge(&2, &3));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn undirected_graph_edges() {
        let mut graph = Graph::undirected();
        
        assert!(graph.add_edge(1, 2));
        assert!(graph.add_edge(2, 3));

        assert_eq!(graph.edge_count(), 2);
        assert!(graph.has_edge(&1, &2));
        assert!(graph.has_edge(&2, &1));
        assert!(graph.has_edge(&2, &3));
        assert!(graph.has_edge(&3, &2));
    }

    #[test]
    fn remove_operations() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(1, 3);

        assert!(graph.remove_edge(&1, &2));
        assert!(!graph.has_edge(&1, &2));
        assert_eq!(graph.edge_count(), 2);

        assert!(graph.remove_vertex(&3));
        assert!(!graph.has_vertex(&3));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn neighbors_and_degrees() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 1);

        assert_eq!(graph.neighbors(&1), Some(&vec![2, 3]));
        assert_eq!(graph.out_degree(&1), Some(2));
        assert_eq!(graph.in_degree(&1), Some(1));
        assert_eq!(graph.out_degree(&2), Some(1));
        assert_eq!(graph.in_degree(&2), Some(1));
    }

    #[test]
    fn edge_iterator() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let edges: HashSet<_> = graph.edges().collect();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(&1, &2)));
        assert!(edges.contains(&(&2, &3)));
    }

    #[test]
    fn vertex_iterator() {
        let mut graph = Graph::directed();
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);

        let vertices: HashSet<_> = graph.vertices().collect();
        assert_eq!(vertices.len(), 3);
        assert!(vertices.contains(&&1));
        assert!(vertices.contains(&&2));
        assert!(vertices.contains(&&3));
    }

    #[test]
    fn clear_graph() {
        let mut graph = Graph::directed();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert!(!graph.is_empty());
        graph.clear();
        assert!(graph.is_empty());
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
}