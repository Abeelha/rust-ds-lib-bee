use crate::utils::{Clear, Size};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
}

#[derive(Debug, Clone)]
pub struct Edge<T, W> {
    pub to: T,
    pub weight: W,
}

impl<T, W> Edge<T, W> {
    pub fn new(to: T, weight: W) -> Self {
        Self { to, weight }
    }
}

pub struct WeightedGraph<T, W> {
    adjacency_list: HashMap<T, Vec<Edge<T, W>>>,
    graph_type: GraphType,
    edge_count: usize,
}

impl<T, W> WeightedGraph<T, W>
where
    T: Clone + Eq + Hash,
    W: Clone,
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
        use std::collections::hash_map::Entry;
        match self.adjacency_list.entry(vertex) {
            Entry::Vacant(e) => {
                e.insert(Vec::new());
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    pub fn add_edge(&mut self, from: T, to: T, weight: W) -> bool {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        let edge_added = if let Some(neighbors) = self.adjacency_list.get_mut(&from) {
            if !neighbors.iter().any(|edge| edge.to == to) {
                neighbors.push(Edge::new(to.clone(), weight.clone()));
                true
            } else {
                false
            }
        } else {
            false
        };

        if edge_added {
            self.edge_count += 1;

            if self.graph_type == GraphType::Undirected && from != to {
                if let Some(neighbors) = self.adjacency_list.get_mut(&to) {
                    neighbors.push(Edge::new(from, weight));
                }
            }
        }

        edge_added
    }

    pub fn has_vertex(&self, vertex: &T) -> bool {
        self.adjacency_list.contains_key(vertex)
    }

    pub fn has_edge(&self, from: &T, to: &T) -> bool {
        self.adjacency_list
            .get(from)
            .is_some_and(|neighbors| neighbors.iter().any(|edge| edge.to == *to))
    }

    pub fn get_edge_weight(&self, from: &T, to: &T) -> Option<&W> {
        self.adjacency_list
            .get(from)?
            .iter()
            .find(|edge| edge.to == *to)
            .map(|edge| &edge.weight)
    }

    pub fn neighbors(&self, vertex: &T) -> Option<&Vec<Edge<T, W>>> {
        self.adjacency_list.get(vertex)
    }

    pub fn vertices(&self) -> impl Iterator<Item = &T> {
        self.adjacency_list.keys()
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

    pub fn remove_vertex(&mut self, vertex: &T) -> bool {
        if !self.adjacency_list.contains_key(vertex) {
            return false;
        }

        let edges_from_vertex = self.adjacency_list[vertex].len();
        self.edge_count -= edges_from_vertex;

        for neighbors in self.adjacency_list.values_mut() {
            let initial_len = neighbors.len();
            neighbors.retain(|edge| edge.to != *vertex);
            self.edge_count -= initial_len - neighbors.len();
        }

        self.adjacency_list.remove(vertex);
        true
    }

    pub fn remove_edge(&mut self, from: &T, to: &T) -> bool {
        let edge_removed = if let Some(neighbors) = self.adjacency_list.get_mut(from) {
            let initial_len = neighbors.len();
            neighbors.retain(|edge| edge.to != *to);
            neighbors.len() < initial_len
        } else {
            false
        };

        if edge_removed {
            self.edge_count -= 1;

            if self.graph_type == GraphType::Undirected && from != to {
                if let Some(neighbors) = self.adjacency_list.get_mut(to) {
                    neighbors.retain(|edge| edge.to != *from);
                }
            }
        }

        edge_removed
    }
}

impl<T, W> Clear for WeightedGraph<T, W> {
    fn clear(&mut self) {
        self.adjacency_list.clear();
        self.edge_count = 0;
    }
}

impl<T, W> Size for WeightedGraph<T, W>
where
    T: Clone + Eq + Hash,
    W: Clone,
{
    fn len(&self) -> usize {
        self.vertex_count()
    }
}

impl<T, W> Default for WeightedGraph<T, W>
where
    T: Clone + Eq + Hash,
    W: Clone,
{
    fn default() -> Self {
        Self::directed()
    }
}

impl<T, W> fmt::Debug for WeightedGraph<T, W>
where
    T: fmt::Debug + Clone + Eq + Hash,
    W: fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeightedGraph")
            .field("adjacency_list", &self.adjacency_list)
            .field("graph_type", &self.graph_type)
            .field("edge_count", &self.edge_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_is_empty() {
        let graph: WeightedGraph<i32, f64> = WeightedGraph::directed();
        assert!(graph.is_empty());
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn add_vertices_and_edges() {
        let mut graph = WeightedGraph::directed();

        assert!(graph.add_vertex(1));
        assert!(graph.add_vertex(2));
        assert!(!graph.add_vertex(1));

        assert!(graph.add_edge(1, 2, 10.0));
        assert!(!graph.add_edge(1, 2, 15.0));

        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.has_vertex(&1));
        assert!(graph.has_vertex(&2));
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn undirected_graph_edges() {
        let mut graph = WeightedGraph::undirected();

        graph.add_edge(1, 2, 5.0);

        assert!(graph.has_edge(&1, &2));
        assert!(graph.has_edge(&2, &1));
        assert_eq!(graph.edge_count(), 1);

        assert_eq!(graph.get_edge_weight(&1, &2), Some(&5.0));
        assert_eq!(graph.get_edge_weight(&2, &1), Some(&5.0));
    }

    #[test]
    fn neighbors_with_weights() {
        let mut graph = WeightedGraph::directed();

        graph.add_edge(1, 2, 10.0);
        graph.add_edge(1, 3, 20.0);

        let neighbors = graph.neighbors(&1).unwrap();
        assert_eq!(neighbors.len(), 2);

        let weights: Vec<_> = neighbors.iter().map(|edge| edge.weight).collect();
        assert!(weights.contains(&10.0));
        assert!(weights.contains(&20.0));
    }

    #[test]
    fn remove_operations() {
        let mut graph = WeightedGraph::directed();

        graph.add_edge(1, 2, 10.0);
        graph.add_edge(2, 3, 20.0);
        graph.add_edge(1, 3, 30.0);

        assert!(graph.remove_edge(&1, &2));
        assert!(!graph.has_edge(&1, &2));
        assert_eq!(graph.edge_count(), 2);

        assert!(graph.remove_vertex(&3));
        assert!(!graph.has_vertex(&3));
        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn clear_graph() {
        let mut graph = WeightedGraph::directed();

        graph.add_edge(1, 2, 10.0);
        graph.add_edge(2, 3, 20.0);

        graph.clear();

        assert!(graph.is_empty());
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
}
