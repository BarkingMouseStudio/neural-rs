#![allow(unused_attributes)]

#![feature(std_misc)]
#![feature(collections)]

pub use self::network::Network;
pub use self::neuron::Neuron;
pub use self::synapse::Synapse;
pub use self::trace::Trace;

pub mod network;
pub mod neuron;
pub mod synapse;
pub mod trace;

pub mod izhikevich;
pub mod stdp;

mod spike;
