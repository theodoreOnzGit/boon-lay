# TRISO decay\_diffusion\_simulator 


## To Do List 1. Release Fraction calculator plus graphing 

2. Change the Graphics for the TRISO (Always needed to scroll too far in)


4. Find reasonable diffusion coefficients for other elements (eg. Y and Zr)
, otherwise, keep them as silver, cos 1e-6 is way too high

5. Temperature Slider (no TH integration yet)
7. Graphics for TRISO core, colour is wrong
8. For solution verification, will need to have fine temperature control 
ie, at x seconds, change temperature of kernel. Time control as well. Typical 
tests are in 200h. Hence timestep may want to be much less. 

3. DONE Change the slider for timestep control (900s is quite fast), need to 
reduce maximum timestep to 1500s. no Log scale (done)
6. DONE Default timestep should be much lower 

## For diffusion scaled timestep

Now, FPs tend to skip the buffer region completely due to high diffusion 
coefficient.
I'm wondering if that plays a factor in the release fraction being obscenely 
large for the triso particle.

Hence, I tried doing diffusion scaled timesteps, to ensure that the 
FPs do random walk through the buffer layer properly. Or could we just assume 
them as skipping the entire layer in one step.

In doing so, this diffusion scaled timestep. The timestep for the buffer 
layer is so small that I cannot even proceed with the simulation. The timestep 
is 1e-6s and less, I can't even move the simulation.
