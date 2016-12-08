# Rymdbiljard
A simple game as part of the TFAY87 Physics course

##Description
Rymdbiljard is a simple game built in Rust. It uses nalgebra for linear algebra, 
and Piston for rendering. The calculations of physics were written from scratch, and 
include collision between balls and with walls, friction with the "floor" and 
gravitational pull from black holes. Some assumptions have been made regarding 
the coefficient of friction and the gravitational constant, to avoid the need to
tweak the masses of all the objects, and also avoid some rounding errors. We also 
do not take the rotation of the balls into account, as it would make our collission
much more complicated. We have also limited the reach of the gravitational force to
stop it from attracting all balls, regardless of their distance.

To calculate the physics we use a time step solution which simulates how the objects
properties would change over the time it is given each step. The time for the 
step may vary between steps. To solve the collisions, we use a simle but powerful 
algorithm to make sure all collisions happen in the right order. 

1. Calculate the collision time for all collisions that will happen if the objects continue with 
their current velocity.
2. Pick the collision which happens first and simulate the movement of all objects up to that time
3. Solve the collision found in step 2 (We use perfectly elastic collisions)    
4. Repeat until no collisions are found or all collisions found happen after the time of our current 
time step

## Documentation
The documentation for the head of the master branch is available online at
[https://holmgr.github.io/rymdbiljard/](https://holmgr.github.io/rymdbiljard/),
and is built automatically. If you wish to build the doucmentation locally or
for branch or state other than the master head then the following command
can be issued in the project root:

```
cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments
```

## Running
To run the project simply run:

```
cargo run
```

## Testing
Most methods and functions in this project are unit tested using the Rust
included testing framework. To run the tests issue the following command in
the project root (or in sub folder):

```
cargo test
```

## Code style
The code styling for this project is following the Rust standard by the use
of the RustFmt project.
This provides an utility for automatically formatting the source code form the
terminal by issuing the following command:

```
cargo fmt
```
