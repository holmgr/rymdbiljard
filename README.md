# Rymdbiljard
A simple game as part of the TFAY87 Physics course

##Description
Rymdbiljard is a simple game build in Rust built as part of the TFYA87 Physics
Cource at Linköping University. Rymdbiljard is a basic biljards game in which
the player is faced with the problem of knocking all poolballs into some goalzone
by hitting them with the white poolball of which he/she has controll over.

[Preview gif of project](preview.gif)

The project relies on some third party libraries
mainly [nalgebra](https://github.com/sebcrozet/nalgebra) for linear algebra
calculations and [Piston](http://www.piston.rs/) for rendering.
The entire physics engine is written from scratch and includes collision detection
and collision handling. There are two types of collisions which are supported;
ball-to-ball and ball-to-wall. The physics engine also includes friction and
gravitation simulations. The frictions is done between the balls and the "floor",
and the gravitation is done between "blackholes" and the poolballs.

To calculate the physics we use a time step solution which simulates how the objects
properties would change over the time in a given step. The step time will vary
between different steps.

Some assumptions has been made to simplify the physics engine in terms of simplyfying
the actual physics. First, we regard all collisions as fully elastic, i.e that
there exists no energy loss. Second, the graviational pull from a blackhole has
a maximum reach to improve gameplay. This means that force is only exerted on
balls within the specified reach. Third, the poolballs "glide" over the floor
instead of rolling and no spin is possible. Thus there is no consideration to
rotational momentum as this would complicate both collision detection and resolution.
Last, all constants in the physical formulas has been
tweaked considerably to improve gameplay, this includes both gravitational
constant as well as the friction coefficient between the poolballs and the floor.

The most interesting part of the project is the coliision detection and resolution
algorithms. To ensure that all collisions are resolved in the correct order we
do the following:

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
