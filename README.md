# Rusty Days Hackathon
My entry for Rusty Days - Virtual Rust Conference Hackathon

Distance field based spheres raymarching rendering shader generator

![screen](https://github.com/zsacul/RustyDaysHackathon/blob/master/distance_field_editor/images/preview.png)

## Simple rules, awesome result:

Raymarching is technique that uses one simple rule to generate outstanding pictures,
for each simple step you search for a distance to all objects in the scene, then you select minimal distance and move along the ray for that distance. Then you repeat until setup number of steps or when distance is smaller then selected epsilon.

You can read more about it here:

http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/

It is quite hard to write such a shaders by a hand, so my contribution it visual editor written in Rust that allows to setup primitives (as now only spheres) by adding simple blocks and setup its properties (like position, radius and color).
In future I am going to extend it for other primitives like plane, box, torus, prism, capsule, round cone, cylinder, octahedron, pyramid and others. I am going also add possibility to compute full constructive solid geometry (CSG) as now only union is supported - intersection and difference is missing.

## Source code used:

##### crates:

https://crates.io/crates/shadertoy

https://crates.io/crates/nuklear-rust

##### IQ shader:

https://www.shadertoy.com/view/Xds3zN