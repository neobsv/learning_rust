# learning_rust

Excerpts and examples from the rust book for fast learning

Rust Book: 
https://doc.rust-lang.org/stable/book/


## Resources

LGR videos on the Rust Book: 
https://youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&si=YS2iKvmK8PHZCPFh

Rust by Example: 
https://doc.rust-lang.org/rust-by-example/

Rust API guidelines: 
https://rust-lang.github.io/api-guidelines/about.html

Cargo book: 
https://doc.rust-lang.org/cargo/

Rustdoc book: 
https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

Docs: 
https://doc.rust-lang.org/std/index.html

Hands On Learning: 
https://github.com/rust-lang/rustlings/

Rust Performance Book:
https://nnethercote.github.io/perf-book/

## Other Resources

Lobsters: 
https://lobste.rs/

Discord: 
* Rust Programming Language: https://discord.com/invite/rust-lang
* Rust Programming Language Community: https://discord.com/invite/rust-lang-community

Are we there yet: 
https://github.com/UgurcanAkkok/AreWeRustYet

Educational Rust Live Coding:
https://youtube.com/playlist?list=PL8lUUBadSMNBNKMYJpUE830tBiN6bxVRw&si=Z9voECcHKHalakYY

Zero to Production In Rust: 
https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/

Benchmarking Rust: 
https://bencher.dev/learn/benchmarking/rust/criterion/


## Notes

Command to run rustfmt over all .rs files recursively:
`find . | grep -E "^.*\.rs$" | xargs -i rustfmt {}`

Note: 
On a mac,
* brew install grep
* you need to call 'ggrep' instead of 'grep' in the above command
* you need to use -I instead of -i for xargs

`find . | ggrep -E "^.*\.rs$" | xargs -I {} rustfmt {}`
