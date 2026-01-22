# Boon Lay Decay Simulator

For this app, I intend to create a decay simulator where there is 
a lattice of nuclides, of which the user can select any nuclide.

Typical nuclides can be given

1. Cs-137
2. I-131 
3. U233 
4. U238 
5. Th232

and so on.


# how the app should run from user perspective

## Main Page
As the user enters the interface, the user should see a main display 
panel. There should be a lattice of radioactive nuclides, maybe about 
100 by 100 (10000), or 500 by 500 (250,000 particles). 

This should be drag-able.

The default particle is U238. 

The nuclides should start decaying in real-time.
The user will be able to 

1. speed up the time, start/stop and pause, reset buttons as well
2. force decays
3. select nuclides of interest to perform decay simulation
4. select any nuclide to perform a decay simulation


As usual, there must be an elapsed time counter and simulation time counter 
as well as a fast forward slider.


Fast forward slider should be on the right hand side.


## Graph and Data Page

A second page should have the decay data for various nuclides.

The most important is the population of the parent product over time. 

There should be a graph and periodic sampling of the population over time.

For daughter products, there may be some challenge, because there are many 
daughter products to consider.

However, I could have the user select up to 3 daughter products over time. 
And during display, the user can select which daughter product is which 
nuclide through a drop down menu of sorts.

This is custom selectable. 

And the UI can also display all the daughter products on hand.


## Radioactivity Page 

This should be a page to determine dose (but is planned for future releases,
not now).

It is a good outreach tool though, but I'll do it for later.



# How app may run at backend 


## Backend threads and Frontend display Separation of responsibility

The Egui app is meant to display information. I cannot have code meant for 
the backend be based on the frontend display. 

Hence, the backend side is solely responsible for calculation. Whereas 
the frontend side is responsible for infromation display ONLY.

## Backend

For the backend, the backend threads will be separate from the frontend display.
I will have 4 threads managing the 250,000 particles. 62,500 particles each,
to track decay and such.

This will be handled using Arc-Mutex locks to enable parallelism.

At the beginning, the particle decay trajectories are calculated stochastically 
during the construction phase.

At each timestep, the responsibility of the thread is just to forward the 
timestep.

Moreover, at the backend, a representative position must be given for the 
nuclide of interest.

## Frontend 

The job of the frontend is just to obtain the state of the particles to 
display it to the user.

At the frontend, it will need some information 

1. What nuclide is represented (name and a certain colour)
2. Where the nuclides are (this is based on the x,y,z coordinates)

