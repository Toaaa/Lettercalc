# Lettercalc

Lettercalc is a small tool that calculates the average score from a list of ratings ranging from `0.5` to `5` in `0.5` steps. It allows you to provide ratings either manually (`-r`) or by reading them from a file (`-f`). The tool calculates the average rating and represents it in both numeric and star formats.

## Installation

Currently, there is a precompiled version available for **Windows only**. You can download it from the [Releases](https://github.com/toaaa/lettercalc/releases/latest) page.

Linux users will have to [build from source](#building-from-source).

## Usage

By default, Lettercalc only accepts ratings in the range of `0.5` to `5`, with steps of `0.5` (e.g., `0.5`, `1.0`, `1.5`, etc.). This means you cannot specify ratings like `3.7`. If you prefer to allow more flexible ratings (e.g., `3.7`), simply add the `-x` flag to your command to disable the step restriction.

```bash
lettercalc -f your-ratings-file.txt -x
```

1. Launch the `lettercalc` binary and **provide ratings**.

2. **Provide Ratings**:
   - **From a File**: Prepare a text file (e.g., `ratings.txt`) with one rating per line. Each rating should be a number between 0.5 and 5 in 0.5 steps. For example:
    
     `ratings.txt`:
     ```txt
     5
     4.5
     4
     3.5
     3
     2.5
     2
     1.5
     1
     ```

     Run the tool with the file option:
     ```bash
     lettercalc -f your-ratings-file.txt
     ```
    
     Output:
     ```
     ⌀: ★★★ (3.00)
     ```
     ---
   - **Manually**: Provide a comma-separated list of ratings directly:
     ```bash
     lettercalc -r 4.5,4,4,4,3.5,3.5,3,2.5,2.5,2.5
     ```

     Output:
     ```
     ⌀: ★★★½ (3.40)
     ```
     ---
   - **From a File (*or manually*) using flexible ratings**
     Do the same as above but this time add the `-x` or `--flexible` option to the executable

     `ratings.txt`:
     ```txt
     4.8
     4.1
     2.9
     0.7
     ```

     ```bash
     lettercalc -f your-ratings-file.txt -x
     ```

     Output:
     ```
     ⌀: ★★★ (3.12)
     ```

## Building from source
### Requirements

- A working installation of the Rust toolchain (typically installed via [rustup](https://rustup.rs/)).

If you want to build it from source, clone this repository and navigate to its directory:

```bash
git clone https://github.com/toaaa/lettercalc.git
cd lettercalc
```

Next, build the application using Cargo:

```bash
cargo build --release
```

Once the build process is complete, you can find the executable file in the `target/release` directory.
Alternatively, you can run it using:

```bash
cargo run --release
```