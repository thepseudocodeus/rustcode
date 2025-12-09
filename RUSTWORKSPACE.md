# WORKSPACE


## Use library
source: google search

To use a library within a Rust workspace, follow these steps: create a workspace.
If you don't already have one, create a new directory for your workspace and initialize it:
Code

    mkdir my_workspace
    cd my_workspace
    cargo init --vcs none # Initializes an empty workspace
Then, modify the Cargo.toml file in my_workspace to define it as a workspace:
Code

    # my_workspace/Cargo.toml
    [workspace]
    members = [
        "my_library",
        "my_binary",
    ]
Create the Library Crate.
Inside your workspace directory, create a new library crate:
Code

    cargo new --lib my_library
This will create a my_library directory with its own Cargo.toml and src/lib.rs.
Create the Binary Crate (or another library that uses it):
Similarly, create a binary crate that will use the library:
Code

    cargo new my_binary
This creates a my_binary directory with its Cargo.toml and src/main.rs. Add the Library as a Dependency.
In the Cargo.toml of the crate that needs to use the library (e.g., my_binary), add a path dependency to your library:
Code

    # my_binary/Cargo.toml
    [dependencies]
    my_library = { path = "../my_library" }
The path specifies the relative path from the my_binary crate's Cargo.toml to the my_library crate's root. Use the Library in Your Code.
In the src/main.rs (or src/lib.rs if it's another library) of the dependent crate, you can now use the items exported by my_library:
Code

    // my_binary/src/main.rs
    use my_library::some_function; // Assuming some_function is public in my_library

    fn main() {
        some_function();
        println!("Hello from my_binary!");
    }
Define and Export Items in the Library:
Ensure that the items you want to use from my_library are declared as pub in my_library/src/lib.rs:
Code

    // my_library/src/lib.rs
    pub fn some_function() {
        println!("Hello from my_library!");
    }
Now, when you run cargo build or cargo run from the workspace root or within my_binary, Cargo will automatically build my_library and link it to my_binary
