use fission_yields_data::prelude::Nuclide;

use crate::prelude::SingleNuclideSimulatorMC;

impl SingleNuclideSimulatorMC {
    // this is vibe coded 
    // obtains a unique list of nuclides  
    // within the decay chain for the simulator

    // Unique nuclides from a single simulator (sorted by Z,A; order not preserved)
    pub fn chain_nuclides_unique_sorted(&self) -> Vec<Nuclide> {
        let mut v: Vec<(Nuclide, (u32, u32))> = self
            .stochastic_decay_chain
            .nuclides_and_decay_data_vec
            .iter()
            .map(|(n, _)| (*n, n.get_z_a()))
            .collect();

        v.sort_by_key(|&(_, key)| key);
        v.dedup_by_key(|&mut (_, key)| key);

        // I had to make an edition because it was not including the  
        // current nuclide
        // then add the existing nuclide 
        let mut unique_nuclide_vector: Vec<Nuclide> = 
            v.into_iter().map(|(n, _)| n).collect();
        unique_nuclide_vector.push(self.current_nuclide);

        unique_nuclide_vector

    }

    // Unique nuclides across multiple simulators (sorted by Z,A; order not preserved)
    pub fn all_chain_nuclides_unique_sorted(sims: &[SingleNuclideSimulatorMC]) -> Vec<Nuclide> {
        let mut v: Vec<(Nuclide, (u32, u32))> = Vec::new();
        for sim in sims {
            for &(n, _) in &sim.stochastic_decay_chain.nuclides_and_decay_data_vec {
                v.push((n, n.get_z_a()));
            }
        }

        v.sort_by_key(|&(_, key)| key);
        v.dedup_by_key(|&mut (_, key)| key);
        v.into_iter().map(|(n, _)| n).collect()
    }

    // Count occurrences of each nuclide (from your unique list) across a vector of simulations,
    // without using HashMap. Compares by (Z, A) via `get_z_a()`.
    //
    // Simple version: linear search through `unique` for each encountered nuclide.
    // Fast version: build a sorted key-index map for `unique` and use binary_search.
    //
    // Assumptions:
    // - Nuclide provides `get_z_a() -> (u16, u16)` (adjust types if needed).
    // - You want counts from the stochastic decay chains of each simulator.
    //
    // Return: Vec<(Nuclide, u64)> aligned with `unique` order.

    pub fn count_nuclides_in_sims_linear(
        sims: &[SingleNuclideSimulatorMC],
        unique: &[Nuclide],
    ) -> Vec<(Nuclide, u64)> {
        // Initialize counts aligned to `unique`
        let mut counts: Vec<(Nuclide, u64)> = unique.iter().cloned().map(|n| (n, 0)).collect();

        for sim in sims {
            for &(n, _) in &sim.stochastic_decay_chain.nuclides_and_decay_data_vec {
                let key = n.get_z_a();
                if let Some((_, c)) = counts.iter_mut().find(|(u, _)| u.get_z_a() == key) {
                    *c += 1;
                }
            }
        }

        counts
    }
}
