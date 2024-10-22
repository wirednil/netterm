# netterm

## Descripción

**Netterm** is a tool in Rust that converts xml form definitions into interactive interfaces within the terminal, using the Cursive library. Ideal for creating and managing forms quickly and efficiently directly in a console environment.

### Personal Workflow

This repository uses a structured nomenclature for the creation of branches, which helps to keep the development organized and efficient. The methodological approach and guidelines for contributing to the project are detailed below.

### Branching structure (Git Branching)

The **netterm** project follows a specific nomenclature for development branches. This nomenclature has the following structure:

```bash
type/NET-n_SomeDescription
```

### Elementos:
- **`type`**: The type of branch (feature, bugfix, hotfix, release).
- **`NET`**: Acronym for the **Netterm** project.
- **`n`**: An incremental counter for each branch of the same type.
- **`_`**: Separator between the counter and the description.
- **`SomeDescription`**: Short description of the functionality or change in `kebab-case` format.

### Examples:
- New functionality:

```bash
feature/NET-1_new-auth-system
```

- Bug fix:

```bash
bugfix/NET-2_terminal-crash-on-exit
```

- New version:

```bash
release/NET-3_version-1.0.0
```

## Technologies used

- **Main language**: Rust
- **UI Framework**: [Cursive](https://github.com/gyscos/cursive) in Rust
- **Compiler**: cargo
- **Operating System**: Linux
- **Version control**: Git and GitHub

## Installation

Follow these steps to clone and configure the project locally:

1. Clone the repository:

```bash
git clone https://github.com/wirednil/netterm.git
cd netterm
```

2. Switch to the branch to which you are assigned:

```bash
git checkout -b "feature/NET-00n_GralDev"
```

3. Make sure you have Rust and Cargo installed on your system. If you do not have them, you can install them by following the instructions in [rustup.rs](https://rustup.rs/).

4. Install the project dependencies and compile:

```bash
cargo build
```

5. Compile the project:

```bash
cargo run
```


