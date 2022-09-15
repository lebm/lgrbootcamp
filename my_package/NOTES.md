# Rust Project Structure

* Packages: one or more crates
  * Cargo.toml
  * Rules: 
    * At least 1 crate
    * At most 1 library crate
    * Zero or more binary crates
* Crate: tree of modules
* Module: Control organization, scope and privacy.
  * Every crate has a root module with possibly sub-modules

Example:

```
my_package on  main [?] is 📦 v0.1.0 via 🦀 v1.63.0
(ins)❯ exa -I 'target|.git' --tree
.
├── Cargo.lock
├── Cargo.toml
├── NOTES.md
└── src
   ├── bin
   │  └── another_one.rs
   ├── lib.rs
   └── main.rs
```

## Modules

* Control  scope and privacy
* Contains functions, types, traits, etc.
* Explicitly defined (mod keyword)
  * Not mapped to the filesystem
    * Flexible and easy conditional compilation