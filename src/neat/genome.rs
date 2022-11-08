use std::collections::HashMap;

use crate::neat::node_gene::ActivationFunction;

use super::{
    connection_gene::ConnectionGene,
    implementation_config,
    innovation_number::InnovationNumberGenerator,
    node_gene::{NodeGene, NodeGeneKind},
    node_id::NodeID,
    predictor::Predictor,
};

/// Represents one genome in a NEAT population
#[derive(Clone, Debug)]
pub struct Genome {
    input_count: usize,
    output_count: usize,

    // Invariant: the first `input_count` nodes in `node_genes` exist and
    // correspond to the input nodes, in order
    // Invariant: the first `output_count` nodes in `node_genes` after the input
    // nodes exist and correspond to the output nodes, in order
    node_genes: Vec<NodeGene>,

    connection_genes: Vec<ConnectionGene>,
    innovation_number_generator: InnovationNumberGenerator,
}

impl Predictor<Vec<f64>, Vec<f64>> for Genome {
    fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        // Ensure we're given the right number of inputs
        assert_eq!(inputs.len(), self.input_count);

        // TODO
        // This is a naive implementation just to get it working. I'm sure this
        // code could be improved significantly, both in terms of efficiency and
        // understandability. It is probably a good target for improvement
        // My least favorite part of this funcion is that there are unexpressed
        // invariants all over the place... that NEEDS to be fixed
        // I still love you anyway, code <3

        // Maintain a list of all the nodes in this predictor, and whether or
        // not each one has fired
        struct PhenotypeNode {
            current_sum: f64,
            bias: f64,
            fired: bool,
            activation_function: ActivationFunction,
        }
        impl PhenotypeNode {
            pub fn new_unfired(bias: f64, activation_function: ActivationFunction) -> Self {
                Self {
                    current_sum: 0.0,
                    bias,
                    fired: false,
                    activation_function,
                }
            }

            pub fn new_fired(
                value: f64,
                bias: f64,
                activation_function: ActivationFunction,
            ) -> Self {
                Self {
                    current_sum: value,
                    bias,
                    fired: true,
                    activation_function,
                }
            }

            pub fn add_to_sum(&mut self, value: f64) {
                if self.fired {
                    panic!("Cannot add to the sum of a PhenotypeNode that already fired");
                }

                self.current_sum += value;
            }

            pub fn get_activation(&self) -> Option<f64> {
                if self.fired {
                    Some(self.current_sum)
                } else {
                    None
                }
            }

            pub fn fired(&self) -> bool {
                self.fired
            }

            pub fn fire(&mut self) {
                if self.fired {
                    panic!("Cannot fire a PhenotypeNode that already fired");
                }

                self.fired = true;
                self.current_sum = self.activation_function.apply(self.current_sum + self.bias);
            }
        }
        let mut phenotype_nodes: HashMap<NodeID, PhenotypeNode> = HashMap::new();
        let mut inputs_iter = inputs.into_iter();
        for node in &self.node_genes {
            phenotype_nodes.insert(
                node.id(),
                match node.kind() {
                    NodeGeneKind::Input => PhenotypeNode::new_fired(
                        inputs_iter.next().unwrap(),
                        node.bias(),
                        node.activation_function(),
                    ),
                    NodeGeneKind::Hidden | NodeGeneKind::Output => {
                        PhenotypeNode::new_unfired(node.bias(), node.activation_function())
                    }
                },
            );
        }

        // Maintain a list of all unused connections, then just use em all (lol)
        let mut unused_connections: Vec<&ConnectionGene> = self
            .connection_genes
            .iter()
            .filter(|connection| connection.enabled())
            .collect();
        'outer: while !unused_connections.is_empty() {
            // Look for an unused connection that has a fired from node and an
            // unfired to node
            for index in (0..unused_connections.len()).rev() {
                let connection = unused_connections[index];
                if phenotype_nodes[&connection.get_from_node()].fired()
                    && !phenotype_nodes[&connection.get_to_node()].fired()
                {
                    // Remove this connection from the list of unused
                    // connections (we're about to use it)
                    unused_connections.remove(index);

                    // Add the "from" node's activation to the "to" node's sum
                    let activation = phenotype_nodes[&connection.get_from_node()]
                        .get_activation()
                        .unwrap();
                    phenotype_nodes
                        .get_mut(&connection.get_to_node())
                        .unwrap()
                        .add_to_sum(activation * connection.weight());

                    // If that was the last unused connection into the "to"
                    // node, fire the "to" node
                    if !unused_connections
                        .iter()
                        .any(|c| c.get_to_node() == connection.get_to_node())
                    {
                        phenotype_nodes
                            .get_mut(&connection.get_to_node())
                            .unwrap()
                            .fire();
                    }

                    continue 'outer;
                }
            }

            unreachable!();
        }

        // All connections were used, fire any unfired nodes
        for node in phenotype_nodes.values_mut() {
            if !node.fired() {
                node.fire();
            }
        }

        // Get the results from the output nodes
        self.node_genes
            .iter()
            .skip(self.input_count)
            .take(self.output_count)
            .map(|node_gene| phenotype_nodes[&node_gene.id()].get_activation().unwrap())
            .collect()
    }
}

impl Genome {
    /// Constructs a new Genome from a list of input `NodeID`s and a list of
    /// output `NodeID`s
    pub fn new(input_node_ids: &[NodeID], output_node_ids: &[NodeID]) -> Self {
        let input_count = input_node_ids.len();
        let output_count = output_node_ids.len();
        let mut innovation_number_generator = InnovationNumberGenerator::new();

        // Initialize node genes
        let node_genes = (input_node_ids
            .iter()
            .copied()
            .map(|id| NodeGene::new(id, NodeGeneKind::Input)))
        .chain(
            output_node_ids
                .iter()
                .copied()
                .map(|id| NodeGene::new(id, NodeGeneKind::Output)),
        )
        .collect();

        // Initialize connection genes
        let mut connection_genes = Vec::with_capacity(input_count * output_count);
        for from_node in input_node_ids.iter().copied() {
            for to_node in output_node_ids.iter().copied() {
                let innovation_number =
                    innovation_number_generator.gen_from_endpoints(from_node, to_node);

                connection_genes.push(ConnectionGene::new(
                    innovation_number,
                    from_node,
                    to_node,
                    implementation_config::DEFAULT_WEIGHT,
                    implementation_config::DEFAULT_CONNECTION_ENABLED,
                ));
            }
        }

        Self {
            innovation_number_generator,
            input_count,
            output_count,
            node_genes,
            connection_genes,
        }
    }
}
