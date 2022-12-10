import subprocess

base = b"""
# Advent of Code 2022

Rust solves for Advent of Code 2022.

Each day has it's own source file with builtin tests and it's own input file.

Some days are broken up between parts 1 and 2 completely, others are combined.

Hopefully this will help somebody.

## Usage
Feel free to use as you see fit.

## Output
```
"""

with open("README.md", "wb") as readme:
    readme.write(base)
    readme.write(subprocess.check_output("cargo run --release".split(" ")))
    readme.write(b"```")
