use crate::neat::node_id::NodeID;

use super::innovation_number::InnovationNumber;

#[derive(Clone, Copy, Debug)]
pub enum ActivationFunction {
    Identity,
    Step,
    Sigmoid,
    Tanh,
    Arctan,
    Relu,
    LeakyRelu(f64),
    SQNL,
}

impl ActivationFunction {
    pub fn apply(&self, x: f64) -> f64 {
        if x.is_infinite() || x.is_nan() {
            panic!("Input to apply cannot be infinite or NaN");
        }

        match self {
            Self::Identity => x,
            Self::Step => {
                if x > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Sigmoid => (x.exp() + 1.0).recip(),
            Self::Tanh => x.tanh(),
            Self::Arctan => std::f64::consts::FRAC_2_PI * x.atan(),
            Self::Relu => {
                if x >= 0.0 {
                    x
                } else {
                    0.0
                }
            }
            Self::LeakyRelu(leak_factor) => {
                if x >= 0.0 {
                    x
                } else {
                    x * leak_factor
                }
            }
            Self::SQNL => {
                if x < -2.0 {
                    -1.0
                } else if x > 2.0 {
                    1.0
                } else {
                    x - x * x.abs() / 4.0
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeGene {
    id: NodeID,
    bias: f64,
    activation_function: ActivationFunction,
}

impl NodeGene {
    /// Constructs a new NodeGene
    pub fn new(id: NodeID, bias: f64, activation_function: ActivationFunction) -> Self {
        Self {
            id,
            bias,
            activation_function,
        }
    }

    /// Returns the NodeID of this NodeGene
    pub fn id(&self) -> NodeID {
        self.id
    }

    /// Returns the bias of this NodeGene
    pub fn bias(&self) -> f64 {
        self.bias
    }

    /// Returns the activation function of this NodeGene
    pub fn activation_function(&self) -> ActivationFunction {
        self.activation_function
    }
}
