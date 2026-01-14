# Intro 

This contains endf 8 depletion chains in xml format from OpenMC.

```
https://openmc.org/data/
```

This will contain decay and transmutation data for Nuclides.


The file is a huge xml file, which needs to be split, and deserialised into 
Rust code.

This crate though, only deals with fission-yields-data, rather than the 
nuclides themselves.

My guess, I would like to move the converted versions of the files to 
another repository. And also it is good to do some kind of educational 
decay simulator for public outreach and education.
