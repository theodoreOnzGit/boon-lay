# Lagrangian Decay Simulator

Now, for decay and transmutation with transport, we could take a control 
volume approach. However, a full blown burnup matrix for even one 
mesh cell is problematic. Could be 4000 by 4000.

1. Large Matrices needed 
2. Stiff system of matrices because half lives differ much in order of 
magnitude. Thus decay and transmutation is slow 
3. There are several reactions to include (all decay, plus neutron 
induced fission, spallation and proton, or alphas)

For diffusion and advection inclusive, there 
are many mesh cells required.

4. now advection and diffusion timescales must be included
