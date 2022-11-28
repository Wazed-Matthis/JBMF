use std::collections::HashSet;

pub struct FlowGraph<V, E> {
    pub vertices: HashSet<V>,
    pub edges: Vec<E>,
}
