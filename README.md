# BOmbardment of neutrons On Nuclides with Lagrangian transport and transmutation Yields (Boon Lay)

For TRISO particle radionuclide transportation visualisation, this 
involves fission, decay, transmutation and diffusion occuring simultaneously 
during reactor operation.

If we consider fission product transport in HTGRs or FHRs, then the 
fission product burnup matrix for an eulerian approach (control volumes)
can become excessively expensive to compute, and difficult to visualise.

Boon Lay approaches the problem from a Lagrangian perspective where 
representative atoms and nuclides are considered. This serves as a way 
to simulate and visualise radionuclide transport in real-time for 
public education and outreach, as well as open source research.


To run the Boon Lay Decay Simulator,
```bash 
cargo run --release --example boon_lay_decay_simulator
```


# Patch Notes 

## v0.1.0 Initial Commit 

This has shown promise because all tests can pass, and the Boon Lay 
Decay Simulator works.

However, the files are much too big for upload to cargo. The compressed 
file size is 18++ MB, whereas cargo limits are around 10 MB. 

I'm going to split up the files into other elements.
