# Windows `#![no_std]` and `#![no_main]` template!
this is a template to get started with no_std and no_main in an windows environment 

this project aims for people who want to quickly get started with no_std on winodws to reduce the file size of their application
this project has been entirely made with rusts core features and the [winapi crate](https://crates.io/crates/winapi)

this project also has an allocator which is possible through [wee_alloc](https://crates.io/crates/wee_alloc). even tho its designed for wasm, it also works with `#![no_std]`.

an allocater is required for some basic functionality of rust e.g. a `vec![];`

## Get started 🔥 

### prerequisites 
have rust installed for your system.

well first of all, it is recommended to use the nightly compiler because of some unstable features, and because its common practice to use nightly for `#![no_std]` aka in embedded environments (which we dont do)
you dont have to do it, but it would be better
i tried it with both stable and nightly, and both worked for me.

anyways, if you want to make sure that it works for you, you can install nightly with the following commands.

```
rustup install nightly
```

```
rustup default nightly
```

to switch back to stable use

```
rustup default stable
```

after you are ready:
run the project on a **WINDOWS MACHINE** with `cargo run`

## Compare the results.
in the root of the project is a `hello_world_std.rs` file

you can build it with rustc to an executable by running

```
rustc hello_world_std.rs
```

__SPOILER ABOUT THE DIFFERENCE BELOW HERE__
<details>
<summary>click here to view results</summary>

> with hello_world_std.rs compiled to a binary
> the file size of the executable with stdlib was around 144kb

> with my no_std lib version the file size is around 89kb with the template code unmodified.

</details>