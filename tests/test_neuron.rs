extern crate test;
extern crate neural;

use std::default::Default;
use std::num::Float;

use neural::Neuron;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};

struct Test<'a> {
  name: &'static str,
  timespan: f64,
  tau: f64,
  config: IzhikevichConfig,
  input: |f64|:'a -> f64,
  spikes: u64,
}

fn run(t: Test) {
  let mut neuron = IzhikevichNeuron::new(t.config);
  let mut now = 0f64;
  let mut spikes = 0;

  println!("{}", t.name);

  while now < t.timespan {
    let ip = (t.input)(now);
    neuron.recv(ip);

    if neuron.tick(t.tau) > 0.0 {
      spikes = spikes + 1;
    }

    println!("{}, {}, {}, {}", now, ip, neuron.v, neuron.u);

    now = now + t.tau;
  }

  assert!(spikes == t.spikes);
}

#[test]
fn default_neuron() {
  run(Test{
    name: "default",
    config: Default::default(),
    timespan: 1000.0,
    tau: 1.0,
    spikes: 65,
    input: |t| {
      if t > 0.0 {
        15.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn tonic_spiking() {
  run(Test{
    name: "tonic_spiking",
    config: IzhikevichConfig::tonic_spiking(),
    timespan: 100.0,
    tau: 0.25,
    spikes: 5,
    input: |t| {
      if t > 10.0 {
        14.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn phastic_spiking() {
  run(Test{
    name: "phastic_spiking",
    config: IzhikevichConfig::phastic_spiking(),
    timespan: 200.0,
    tau: 0.25,
    spikes: 1,
    input: |t| {
      if t > 20.0 {
        0.5
      } else {
        0.0
      }
    }
  });
}

#[test]
fn tonic_bursting() {
  run(Test{
    name: "tonic_bursting",
    config: IzhikevichConfig::tonic_bursting(),
    timespan: 220.0,
    tau: 0.25,
    spikes: 28,
    input: |t| {
      if t > 22.0 {
        15.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn phastic_bursting() {
  run(Test{
    name: "phastic_bursting",
    config: IzhikevichConfig::phastic_bursting(),
    timespan: 200.0,
    tau: 0.2,
    spikes: 6,
    input: |t| {
      if t > 20.0 {
        0.6
      } else {
        0.0
      }
    }
  });
}

#[test]
fn mixed_mode() {
  run(Test{
    name: "mixed_mode",
    config: IzhikevichConfig::mixed_mode(),
    timespan: 160.0,
    tau: 0.25,
    spikes: 6,
    input: |t| {
      if t > 16.0 {
        10.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn spike_frequency_adaptation() {
  run(Test{
    name: "spike_frequency_adaptation",
    config: IzhikevichConfig::spike_frequency_adaptation(),
    timespan: 85.0,
    tau: 0.25,
    spikes: 6,
    input: |t| {
      if t > 8.0 {
        30.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn class1() {
  run(Test{
    name: "class1",
    config: IzhikevichConfig::class1(),
    timespan: 300.0,
    tau: 0.25,
    spikes: 10,
    input: |t| {
      if t > 30.0 {
        0.075 * (t - 30.0)
      } else {
        0.0
      }
    }
  });
}

#[test]
fn class2() {
  run(Test{
    name: "class2",
    config: IzhikevichConfig::class2(),
    timespan: 300.0,
    tau: 0.25,
    spikes: 15,
    input: |t| {
      if t > 30.0 {
        -0.5 + (0.015 * (t - 30.0))
      } else {
        0.5
      }
    }
  });
}

#[test]
fn spike_latency() {
  run(Test{
    name: "spike_latency",
    config: IzhikevichConfig::spike_latency(),
    timespan: 100.0,
    tau: 0.2,
    spikes: 1,
    input: |t| {
      if t > 10.0 && t < 13.0 {
        7.04
      } else {
        0.0
      }
    }
  });
}

#[test]
fn subthreshold_oscillation() {
  run(Test{
    name: "subthreshold_oscillation",
    config: IzhikevichConfig::subthreshold_oscillation(),
    timespan: 200.0,
    tau: 0.25,
    spikes: 1,
    input: |t| {
      if t > 20.0 && t < 25.0 {
        2.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn resonator() {
  run(Test{
    name: "resonator",
    config: IzhikevichConfig::resonator(),
    timespan: 400.0,
    tau: 0.25,
    spikes: 1,
    input: |t| {
      if (t > 40.0 && t < 44.0) || (t > 60.0 && t < 64.0) || (t > 280.0 && t < 284.0) || (t > 320.0 && t < 324.0) {
        0.65
      } else {
        0.0
      }
    }
  });
}

#[test]
fn integrator() {
  run(Test{
    name: "integrator",
    config: IzhikevichConfig::integrator(),
    timespan: 100.0,
    tau: 0.25,
    spikes: 1,
    input: |t| {
      if (t > 9.09 && t < 11.09) || (t > 14.09 && t < 16.09) || (t > 70.0 && t < 72.0) || (t > 80.0 && t < 82.0) {
        9.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn rebound_spike() {
  run(Test{
    name: "rebound_spike",
    config: IzhikevichConfig::rebound_spike(),
    timespan: 200.0,
    tau: 0.2,
    spikes: 1,
    input: |t| {
      if t > 20.0 && t < 25.0 {
        -15.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn rebound_burst() {
  run(Test{
    name: "rebound_burst",
    config: IzhikevichConfig::rebound_burst(),
    timespan: 200.0,
    tau: 0.2,
    spikes: 7,
    input: |t| {
      if t > 20.0 && t < 25.0 {
        -15.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn threshold_variability() {
  run(Test{
    name: "threshold_variability",
    config: IzhikevichConfig::threshold_variability(),
    timespan: 100.0,
    tau: 0.25,
    spikes: 1,
    input: |t| {
      if (t > 10.0 && t < 15.0) || (t > 80.0 && t < 85.0) {
        1.0
      } else if t > 70.0 && t < 75.0 {
        -6.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn bistability() {
  run(Test{
    name: "bistability",
    config: IzhikevichConfig::bistability(),
    timespan: 300.0,
    tau: 0.25,
    spikes: 6,
    input: |t| {
      if (t > 38.0 && t < 43.0) || (t > 216.0 && t < 221.0) {
        1.24
      } else {
        0.24
      }
    }
  });
}

#[test]
fn depolarizing_after_potential() {
  run(Test{
    name: "depolarizing_after_potential",
    config: IzhikevichConfig::depolarizing_after_potential(),
    timespan: 50.0,
    tau: 0.1,
    spikes: 1,
    input: |t| {
      if (t - 10.0).abs() < 1.0 {
        20.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn accomodation() {
  run(Test{
    name: "accomodation",
    config: IzhikevichConfig::accomodation(),
    timespan: 400.0,
    tau: 0.5,
    spikes: 50,
    input: |t| {
      if t < 200.0 {
        t / 25.0
      } else if t < 300.0 {
        0.0
      } else if t < 312.5 {
        ((t as f64) - 300.0) / 12.5 * 4.0
      } else {
        0.0
      }
    }
  });
}

#[test]
fn inhibition_induced_spiking() {
  run(Test{
    name: "inhibition_induced_spiking",
    config: IzhikevichConfig::inhibition_induced_spiking(),
    timespan: 350.0,
    tau: 0.5,
    spikes: 3,
    input: |t| {
      if t < 50.0 || t > 250.0 {
        80.0
      } else {
        75.0
      }
    }
  });
}

#[test]
fn inhibition_induced_bursting() {
  run(Test{
    name: "inhibition_induced_bursting",
    config: IzhikevichConfig::inhibition_induced_bursting(),
    timespan: 350.0,
    tau: 0.5,
    spikes: 12,
    input: |t| {
      if t < 50.0 || t > 250.0 {
        80.0
      } else {
        75.0
      }
    }
  });
}