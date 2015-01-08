extern crate test;
extern crate neural;

use test::Bencher;

use std::default::Default;
use std::rand;
use std::rand::Rng;
use std::rand::distributions::{Normal, IndependentSample};
use std::num::Float;

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};

#[bench]
fn spikes(bn: &mut Bencher) {
  let mut rng = rand::task_rng();
  let mut network = Network::new(20);

  let excitatory_count = 800u64;
  let inhibitory_count = 200u64;
  let total_count = excitatory_count + inhibitory_count;

  for _ in range(0u64, excitatory_count) {
    let r = rng.gen::<f64>();
    let a = 0.02;
    let b = 0.2;
    let c = -65.0 + (15.0 * r.powi(2));
    let d = 8.0 - (6.0 * r.powi(2));
    let v = -65.0;
    let u = b * v;

    network.add_neuron(box IzhikevichNeuron::new(IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    }));
  }

  for _ in range(0u64, inhibitory_count) {
    let r = rng.gen::<f64>();
    let a = 0.02 + (0.08 * r);
    let b = 0.25 - (0.05 * r);
    let c = -65.0;
    let d = 2.0;
    let v = -65.0;
    let u = b * v;

    network.add_neuron(box IzhikevichNeuron::new(IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    }));
  }

  for n in range(0u64, total_count) {
    for m in range(0u64, total_count) {
      let weight = if n < excitatory_count { // excitatory
        0.5 * rng.gen::<f64>()
      } else { // inhibitory
        -1.0 * rng.gen::<f64>()
      };

      network.add_synapse(box STDPSynapse::new(STDPConfig{
        weight: weight,
        min: -1.0,
        max: 1.0,
        n_pos: 0.0,
        n_neg: 0.0,
        tau_pos: 0,
        tau_neg: 0,
        a_pos: 0.0,
        a_neg: 0.0,
        continuous: true,
        scale: true,
        delay: 1
      }), n, m);
    }
  }

  let norm = Normal::new(0.0, 1.0);

  bn.iter(|| {
    for n in range(0u64, total_count) {
      // thalmic input
      let i = if n < excitatory_count {
        5.0 * norm.ind_sample(&mut rng)
      } else {
        2.0 * norm.ind_sample(&mut rng)
      };

      network.recv(n, i);
    }

    network.tick(1.0);
  });
}
