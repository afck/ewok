// TODO: consider getting rid of "num_nodes" parameter?
#[derive(Clone)]
pub struct SimulationParams {
    /// Number of nodes.
    pub num_nodes: u64,
    /// Number of steps to run the simulation for.
    pub num_steps: u64,
    /// Maximum number of steps a message can be delayed by before it's delivered.
    pub max_delay: u64,
    /// Probability of a node joining on a given step.
    pub prob_join: f64,
    /// Probability of a node leaving on a given step.
    pub prob_drop: f64,
    /// Step at which to start dropping nodes (gives the network time to start up).
    pub drop_step: u64,
    // Probability that a two-way connection will be lost on any given step.
    // prob_disconnect: f64,
    // Probability that a lost two-way connection will be re-established on any given step.
    // prob_reconnect: f64
}


#[derive(Clone)]
pub struct NodeParams {
    /// Number of steps to wait for a node to appear in all current sections after it has
    /// first appeared in a single current section.
    pub join_stabilisation_timeout: u64
}