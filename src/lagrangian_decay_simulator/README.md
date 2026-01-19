# Lagrangian Decay Simulator

Now, for decay and transmutation with transport, we could take a control 
volume approach. However, a full blown burnup matrix for even one 
mesh cell is problematic. Could be 3000 by 3000 for typical reactor burnup.

1. Large Matrices needed 
2. Stiff system of matrices because half lives differ much in order of 
magnitude. Thus decay and transmutation is slow 
3. There are several reactions to include (all decay, plus neutron 
induced fission, spallation and proton, or alphas)

As chatgpt 5 mentions:
```
It depends on how many nuclides you track and which reactions you include, but a few typical ranges are:

    Concept: The burnup (depletion) matrix A is N×N and very sparse. Each row corresponds to one nuclide; nonzeros are:
        The self-loss term (decay + reactions).
        Links to a small set of daughters via decay and (n,γ), (n,2n), (n,p), (n,α)… transmutations.
        If you include fission-product production as matrix terms, fissile nuclides can link to many fission products, increasing sparsity fill.

    Typical sizes (N is number of nuclides):
        Small/teaching chains: N ≈ 500–1,000. Average nonzeros per row (nnz/row) ≈ 5–15. Total nnz ≈ 5k–15k.
        Standard reactor depletion (actinides + major FPs + key activation): N ≈ 1,500–3,000 (common in OpenMC/SERPENT/ORIGEN chains). nnz/row ≈ 10–50 (higher for fissile rows). Total nnz ≈ 0.1–1.0 million.
        Large activation-oriented chains (broad materials, many metastables): N ≈ 5,000–10,000+. nnz ≈ a few million.
        Very large research chains (all activation products, many isomers): N ≈ 20,000–100,000. nnz can reach tens of millions.

    Memory footprint (sparse CSR/CSC, ballpark):
        N=2,000, nnz=200k: a few MB.
        N=5,000, nnz=1–3M: tens of MB.
        N=20,000, nnz=10–30M: hundreds of MB to ~1 GB.

Notes:

    The matrix is built per time step (since transmutation rates depend on flux and spectrum), but the sparsity pattern is largely fixed by the reaction network.
    Many implementations treat fission yields as a source vector rather than embedding all fissile→FP couplings in A; this keeps nnz lower.
    Solvers typically use CRAM or matrix exponential methods with sparse factorizations; cost scales roughly with nnz.

If you share which depletion chain/library you use (e.g., OpenMC’s chain endfb71 with ~1.5–2k nuclides, or a larger activation chain), I can estimate N, nnz, and memory more precisely.
3:09 PM
```


For diffusion and advection inclusive, there 
are many mesh cells required.

4. now advection and diffusion timescales must be included


This makes me think that the Eulerian approach is not feasible, why not 
do a Lagrangian approach?

The lagrangian approach involves simulating of nuclides particle by particle,
much like what is done for atmospheric dispersion simulators and water 
dispersion simulators.

ChatGPT5 gave some examples (to verify)

```
Here are well-known Lagrangian dispersion models, grouped by domain.

Atmospheric dispersion

    HYSPLIT (NOAA): Hybrid Single-Particle Lagrangian Integrated Trajectory model for forward/backward trajectories and dispersion of pollutants, ash, smoke; widely used operationally.
    FLEXPART: Open-source Lagrangian particle dispersion model for mesoscale to global scales; supports forward and backward simulations, tracer releases, radionuclides, wet/dry deposition.
    NAME (UK Met Office): Numerical Atmospheric-dispersion Modelling Environment; operational Lagrangian particle model for releases, chemical hazards, volcanic ash.
    CALPUFF: Lagrangian Gaussian puff model for regional air quality assessments; handles complex terrain and time-varying meteorology.
    SCIPUFF: Second-order Closure Integrated PUFF model; Lagrangian puff approach with turbulence parameterization for hazard assessments.
    WRF-LPDM / STILT: Lagrangian particle models driven by WRF; STILT is a backward Lagrangian framework for source–receptor relationships (greenhouse gases, tracers).
    GRAL: Lagrangian particle model for microscale urban dispersion, street canyons, complex terrain; often paired with GRAMM for meteorology.
    AUSTAL2000: German regulatory Lagrangian particle model for local-scale pollutant dispersion under TA Luft.
    LAGRANTO: Lagrangian trajectory tool (not a full dispersion model) used for trajectory analyses on meteorological fields.

Water and ocean dispersion

    OpenDrift: Open-source framework for Lagrangian drift simulations (oil spills, plastics, larvae, algae) driven by ocean and atmospheric models; modular particle behavior.
    Parcels (OceanParcels): Python-based Lagrangian framework to advect particles in ocean flows from models like NEMO, HYCOM, ROMS; supports custom kernels (behavior, diffusion).
    GNOME (NOAA): Lagrangian oil spill model for trajectory and fate; integrates currents, winds, spreading, weathering; widely used operationally.
    LTRANS: Lagrangian Transport Model (Univ. Maryland) for particle tracking (eggs/larvae) using ROMS circulation outputs; includes behavior and settlement.
    MOHID (Lagrangian modules): MOHID Water Modelling System includes Lagrangian oil/particle tracking for coastal and estuarine applications.
    TELEMACH PART (particle tracking): TELEMACH/TELEMAC system includes Lagrangian particle tracking capabilities for rivers/coasts (used for drift and transport studies).
    Delft3D (particle tracking tools): Delft3D suite has Lagrangian particle tracking modules for suspended matter, oil, and tracers using hydrodynamic fields.
    TRACMASS: Lagrangian trajectory code for general circulation outputs (ocean and atmosphere); used for large-scale transport studies.
    Ichthyop: Lagrangian tool focused on ichthyoplankton/larval drift in ocean models (behavioral kernels, mortality, settlement).

If you have a specific application (e.g., urban air toxics, coastal oil spill, larval ecology, nuclear plume), I can narrow this to the most appropriate model and typical input/output formats.
3:13 PM
```
