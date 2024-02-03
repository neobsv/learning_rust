# learning_rust

Excerpts and examples from the rust book for fast learning

Command to run rustfmt over all .rs files recursively:
find . | grep -E "^*.rs$" | xargs -i rustfmt {}
