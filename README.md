# pgfx

PGFX is a prototype 3D graphics library. 

This library does not provide any implementation, but rather provides the common ground in which code is structured and organized.

# Disclaimer - Prototype

This is being provided as a prototype library. I am currently developing a game (Penta Terra) and do not have time to fully 
invest in this. Please feel free to use this concept if you are interesting in building it out. 

# Problems with Existing Graphics Libraries

- Redundant code
    - Rust type definition
    - Pipeline format definition and/or binding definitions
    - Type-specific input binding
- Rather verbose (even in OpenGL and other light-weight implementations)
- Each data type has a completely different methods of loading/using the data
- High degree of variability across implementations
    - Makes porting from one implementation to another very difficult

# Fundamental Concepts

3D graphics centers on two primary concepts:
1. Load data into VRAM
2. Execute code using the loaded data as parameters

It should be possible to do both of these things in a consistent and trivial manner.

## Load Data into GPU

All load operations start with `load`. Loading is done based on the data's underlying type information. 

The most typical operations are:
- `device.load(single_value)`
- `device.load_array(list_of_values)`
- `device.load_using(single_value, usages)`
- `device.load_array_using(list_of_values, usages)`

For example, loading an index buffer is as simple as: `let ibuf = device.load_array(&[0, 1, 2u32])?;`. This is because `u32` has
been defined as index buffer data type by this library. 

## Execute Code

All the information around a set of render/compute code is stored in a `Pipeline`. Pipelines are typically initialized on first
use. This avoids having to separate the layout definitions from the actual call to the pipeline.

This is an example call:

```rust
render_pass.run(&my_pipeline)
    .input(&vertex_buffer)
    .input(&index_buffer)
    .load_input(&MyUniforms { ... })
    .execute(|cfg| {
        // Initialize the pipeline: This will be executed only once.
        cfg.input(&my_vertex_shader)?;
        cfg.input(&my_fragment_shader)?;

        cfg.load_input(&load_image("my_image"))?;
        cfg.load_input(&Sampler::linear_repeat())?;

        Ok(())
    })?;
```

All parameters that are involved in binding will be bound in order of appearance. 

# Design Philosophy

Overall philosophy:
- Have a small, backend-agnostic common library
    - Allows 3rd-party libraries to link without implementation baggage
    - Defines the overall flow and organization without imposing too many constraints
- Implementations would be done in *independent* crates
    - Allows each implementation to work at their own pace
    - For platform-specific implementations, it allows a faster workflow
        - There is a testing barrier that makes contributing to platform-independent crates difficult

Ensure everything is as simple as possible for primary use cases. However, do not prevent downstream libraries from exposing 
library-specific concepts.

Data format should be specified in the type definition. Do not force the user to re-define the same data for the GPU as well.

There are two data storage methods:
- `Stored`/`StoredData`: Thread-specific data, uses `Rc` reference counting.
    - Rust's type system avoids multi-threading concerns when rendering
- `StoredSend`/`StoredSendData`: Data that can be sent between threads.
    - Cannot convert between `StoredData` and `StoredSendData` unless there is only a single reference

## Empty Arrays

Empty arrays should not be considered an error and should load successfully. However, it is considered okay if there is overhead 
to empty data sets. For example, a backend may need to store a zeroed buffer just to have a placeholder value. Likewise, when 
calling a pipeline with an empty array, it should not result in an error. Often it may mean skipping the pipeline execution 
entirely (ex. when calling a pipeline with an empty index buffer). 

# Known Issues

The following are known gaps/problems with this library:
- Mipmapping is not available
    - Create `TextureMipmap` version(s) of `Texture` and add `stored_texture.mip_level(index)` capability
- No raytracing data structures
    - Might be too soon to include these in the base library?

# FPS Counter

To enable FPS counting, call `surface.track_fps(true)`, then call `surface.fps()` to get the current FPS. 
