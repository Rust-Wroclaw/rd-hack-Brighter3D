# Rusty Days Hackathon
My entry for Rusty Days - Virtual Rust Conference Hackathon

Distance field based spheres raymarching rendering shader generator

![screen](https://github.com/zsacul/RustyDaysHackathon/blob/master/distance_field_editor/images/preview.png)

## Simple rules, awesome result:

Raymarching is technique that uses one simple rule to generate outstanding pictures,
for each simple step you search for distance to all objects in the scene, then you select minimal distance and move along the ray for that distance.

You can read more about it here:

http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/


## Source code used:

##### crates:

https://crates.io/crates/shadertoy

https://crates.io/crates/nuklear-rust

##### IQ shader:

https://www.shadertoy.com/view/Xds3zN