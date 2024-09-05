use qip::prelude::*;

fn main() {
    println!("Evan's Quantum Computing Simulation Sandbox");
    bell_circuit();
}

fn bell_circuit() {
    println!("\nA Bell circuit simulation");

    // Create a new circuit builder
    let mut b = LocalBuilder::<f64>::default();

    // Allocate two qubits
    let q0 = b.qubit();
    let q1 = b.qubit();

    // Apply Hadamard gate to the first qubit (get it spinning!)
    let q0 = b.h(q0);

    // Apply CNOT gate with first qubit as control, second as target (first qubit now controls second qubit's state)
    let (q0, q1) = b.cnot(q0, q1).unwrap();

    // Measure both qubits
    let (_, m0) = b.measure(q0);
    let (_, m1) = b.measure(q1);

    // Run the circuit
    let (_, measured) = b.calculate_state();

    // Get measurement results
    let (result0, p0) = measured.get_measurement(m0);
    let (result1, p1) = measured.get_measurement(m1);

    println!("Qubit 0: {:?} (probability: {:?})", result0, p0);
    println!("Qubit 1: {:?} (probability: {:?})", result1, p1);

    // Run the circuit multiple times to see the distribution
    let mut counts = std::collections::HashMap::new();
    for _ in 0..1000 {
        let (_, measured) = b.calculate_state();
        let (r0, _) = measured.get_measurement(m0);
        let (r1, _) = measured.get_measurement(m1);
        let key = format!("{}{}", r0, r1);
        *counts.entry(key).or_insert(0) += 1;
    }

    println!("Measurement counts: {:?}", counts);
}