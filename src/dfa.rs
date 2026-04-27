use crate::cluster::GraphemeCluster;
use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::{Edges, StableGraph};
use petgraph::visit::Dfs;
use petgraph::{Directed, Direction};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet};
type State = NodeIndex<u32>;
type StateLabel = String;
type EdgeLabel = Grapheme;
pub(crate) struct Dfa<'a> {
    alphabet: BTreeSet<Grapheme>,
    graph: StableGraph<StateLabel, EdgeLabel>,
    initial_state: State,
    final_state_indices: HashSet<usize>,
    config: &'a RegExpConfig,
}
impl<'a> Dfa<'a> {
    pub(crate) fn from(
        grapheme_clusters: &[GraphemeCluster],
        is_minimized: bool,
        config: &'a RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn state_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn states_in_depth_first_order(&self) -> Vec<State> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn outgoing_edges(&self, state: State) -> Edges<'_, Grapheme, Directed> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_final_state(&self, state: State) -> bool {
        panic!("STUB: not implemented");
    }
    fn new(config: &'a RegExpConfig) -> Self {
        panic!("STUB: not implemented");
    }
    fn insert(&mut self, cluster: &GraphemeCluster) {
        panic!("STUB: not implemented");
    }
    fn return_next_state(
        &mut self,
        current_state: State,
        edge_label: &Grapheme,
    ) -> State {
        panic!("STUB: not implemented");
    }
    fn find_next_state(
        &mut self,
        current_state: State,
        grapheme: &Grapheme,
    ) -> Option<State> {
        panic!("STUB: not implemented");
    }
    fn add_new_state(&mut self, current_state: State, edge_label: &Grapheme) -> State {
        panic!("STUB: not implemented");
    }
    #[allow(clippy::many_single_char_names)]
    fn minimize(&mut self) {
        panic!("STUB: not implemented");
    }
    fn get_initial_partition(&self) -> Vec<HashSet<State>> {
        panic!("STUB: not implemented");
    }
    fn get_parent_states(&self, a: &HashSet<State>, label: &Grapheme) -> HashSet<State> {
        panic!("STUB: not implemented");
    }
    fn recreate_graph(&mut self, p: Vec<&HashSet<State>>) {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_count() {
        let config = RegExpConfig::new();
        let mut dfa = Dfa::new(&config);
        assert_eq!(dfa.state_count(), 1);
        dfa.insert(&GraphemeCluster::from("abcd", &RegExpConfig::new()));
        assert_eq!(dfa.state_count(), 5);
    }
    #[test]
    fn test_is_final_state() {
        let config = RegExpConfig::new();
        let dfa = Dfa::from(
            &[GraphemeCluster::from("abcd", &RegExpConfig::new())],
            true,
            &config,
        );
        let intermediate_state = State::new(3);
        assert_eq!(dfa.is_final_state(intermediate_state), false);
        let final_state = State::new(4);
        assert_eq!(dfa.is_final_state(final_state), true);
    }
    #[test]
    fn test_outgoing_edges() {
        let config = RegExpConfig::new();
        let dfa = Dfa::from(
            &[
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("abxd", &RegExpConfig::new()),
            ],
            true,
            &config,
        );
        let state = State::new(2);
        let mut edges = dfa.outgoing_edges(state);
        let first_edge = edges.next();
        assert!(first_edge.is_some());
        assert_eq!(
            first_edge.unwrap().weight(), & Grapheme::from("c", false, false, false)
        );
        let second_edge = edges.next();
        assert!(second_edge.is_some());
        assert_eq!(
            second_edge.unwrap().weight(), & Grapheme::from("x", false, false, false)
        );
        let third_edge = edges.next();
        assert!(third_edge.is_none());
    }
    #[test]
    fn test_states_in_depth_first_order() {
        let config = RegExpConfig::new();
        let dfa = Dfa::from(
            &[
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("axyz", &RegExpConfig::new()),
            ],
            true,
            &config,
        );
        let states = dfa.states_in_depth_first_order();
        assert_eq!(states.len(), 7);
        let first_state = states.get(0).unwrap();
        let mut edges = dfa.outgoing_edges(*first_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("a", false, false, false)
        );
        assert!(edges.next().is_none());
        let second_state = states.get(1).unwrap();
        edges = dfa.outgoing_edges(*second_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("b", false, false, false)
        );
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("x", false, false, false)
        );
        assert!(edges.next().is_none());
        let third_state = states.get(2).unwrap();
        edges = dfa.outgoing_edges(*third_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("y", false, false, false)
        );
        assert!(edges.next().is_none());
        let fourth_state = states.get(3).unwrap();
        edges = dfa.outgoing_edges(*fourth_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("z", false, false, false)
        );
        assert!(edges.next().is_none());
        let fifth_state = states.get(4).unwrap();
        edges = dfa.outgoing_edges(*fifth_state);
        assert!(edges.next().is_none());
        let sixth_state = states.get(5).unwrap();
        edges = dfa.outgoing_edges(*sixth_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("c", false, false, false)
        );
        assert!(edges.next().is_none());
        let seventh_state = states.get(6).unwrap();
        edges = dfa.outgoing_edges(*seventh_state);
        assert_eq!(
            edges.next().unwrap().weight(), & Grapheme::from("d", false, false, false)
        );
        assert!(edges.next().is_none());
    }
    #[test]
    fn test_minimization_algorithm() {
        let config = RegExpConfig::new();
        let mut dfa = Dfa::new(&config);
        assert_eq!(dfa.graph.node_count(), 1);
        assert_eq!(dfa.graph.edge_count(), 0);
        dfa.insert(&GraphemeCluster::from("abcd", &RegExpConfig::new()));
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 4);
        dfa.insert(&GraphemeCluster::from("abxd", &RegExpConfig::new()));
        assert_eq!(dfa.graph.node_count(), 7);
        assert_eq!(dfa.graph.edge_count(), 6);
        dfa.minimize();
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }
    #[test]
    fn test_dfa_constructor() {
        let config = RegExpConfig::new();
        let dfa = Dfa::from(
            &[
                GraphemeCluster::from("abcd", &RegExpConfig::new()),
                GraphemeCluster::from("abxd", &RegExpConfig::new()),
            ],
            true,
            &config,
        );
        assert_eq!(dfa.graph.node_count(), 5);
        assert_eq!(dfa.graph.edge_count(), 5);
    }
}
