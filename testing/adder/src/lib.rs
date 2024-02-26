// Testing

// We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options 
// provided for running your tests, and how to organize tests into unit tests and integration tests.

// The bodies of test functions perform these actions:
    // 1. Setup any needed data or state
    // 2. Run the code you need to test
    // 3. Assert the results are what you expect

// Let's look at the test attribute, macros and the should_panic attribute.

// Anatomy of a Test Function

// To change a function into a test function, add #[test] on the line before fn
// Run the tests with the cargo test command, and rust builds a test runner binary and generates a test report as well

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code, unused_variables)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}


pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}


#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }

        Guess { value }
    }
}



#[cfg(test)]
mod tests {

    // The tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. And we use a glob (*) here.
    use super::*;

    // Note the #[test] annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // We might also have non-test functions in the tests module

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // A panic can make a test fail
        // panic!("fail.")
    }

// Features:
// Ignored tests: We can mark a test as ignored, so that cargo test won't pick it up.
// Filtering tests: Can pass an argument to cargo test in order to select only particular tests to run.
// Documentation tests: Rust can compile any code examples that appear in our API documentation, Doc-tests output
// Benchmark tests: The command cargo bench can be used to run benchmark tests (not available yet)


// Checking results with the assert!() macro

// The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
// If the value is false, the assert! macro calls panic! to cause the test to fail.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }


// Testing equality with the assert_eq! and assert_ne! macros

// Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their 
// arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.


    #[test]
    fn it_adds_three() {
        assert_eq!(5, add_three(2));
    }

    #[test]
    fn it_doesnt_add_two() {
        assert_ne!(4, add_three(2));
    }


// Adding Custom Failure Messages

// Add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros.
// Underlying mechanism to support this is the format!() macro of course.

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }


    /*
    // In this test, we purposefully introduce a bug in order to fail it, and the failure message is passed into the assert!() macro
    #[test]
    fn greeting_doesnt_contain_name() {
        let result = greeting("Carol");
        assert!(result.contains("David"), "Greeting did not contain name David, value was `{}`", result);
    }
    */

    // Checking for panics with should_panic

    // We do this by adding the attribute should_panic to our test function. 
    // The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.

    // IMPORTANT: We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }


    // We can extend this by adding an optional expected parameter to the should_panic function that specifies exactly why the test should panic
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100_ii() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests

    // Instead of panic! , we can let the test return an error using the Result<T, E> enum.
    // The it_works function now has the Result<(), String> return type. We return Ok(()) when the test passes and an Err with a String inside when the test fails.
    #[test]
    fn it_works_ii() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // IMPORTANT: You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
    // To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).


    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }


}

// Controlling How Tests Are Run

// The default behavior of the binary produced by cargo test is to run all the tests in parallel and capture output generated during test runs.
// The stdout of each test function is not printed to console, unless explicitly specified
// cargo test --help prints the options you can use with cargo test (test runner), and cargo test -- --help prints the options you can use with the test binary.

// Run tests sequentially, do not parallelize:
// $ cargo test -- --test-threads=1

// Show function output after execution:
// $ cargo test -- --show-output

// Running a subset of tests

// To Run a particular testcase, specify the full name of the test you want to run:
// For example, from the above module,
// $ cargo test one_hundred

// Filtering to run multiple testcases
// For example, just enter the common substring i.e the common substring contained in all of the subset of tests that you want to execute:
// $ cargo test add
/* 
running 4 tests
test tests::add_three_and_two ... ok
test tests::it_adds_three ... ok
test tests::add_two_and_two ... ok
test tests::it_doesnt_add_two ... ok
*/

// Ignoring Some Tests Unless Specifically Requested

// The #[ignore] annotation right after the #[test] annotation can be used to ignore test cases
// For example, consider the expensive_test() function above, and it will be ignored when cargo test is run
// $ cargo test
/*
running 16 tests
test expensive_test ... ignored
...
*/

// And, in order to explictly select and run only the ignored tests, use this command
// $ cargo test -- --ignored
/*
running 1 test
test tests::expensive_test ... ok
*/

// Finally, run all tests whether they’re ignored or not, you can use: 
// $ cargo test -- --include-ignored


// Test Organization

// Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
// Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.


// Unit Tests
// The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
// You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named tests in each file to contain the test functions and to annotate the module with #[cfg(test)].
// Because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

/* NOTE:
The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option. 
In this case, the configuration option is test, which is provided by Rust for compiling and running tests. 
By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test. 
This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].
*/


// Testing Private Functions

pub fn add_two_too(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// NOTE: Note that the internal_adder function is not marked as pub.
// In this test, we bring all of the test module’s parent’s items into scope with use super::*, and then the test can call internal_adder. 
// BIG NOTE: If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so.

// Integration Tests

// Placed in a separate folder called 'tests' in the top level dir.
// They can only call functions that are part of your library’s public API. 
// Their purpose is to test whether many parts of your library work together correctly.
// Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.

// There are integration tests placed in the top level dir, in tests/integration_test.rs
// The code in this lib.rs is imported using 'use adder;' since integration tests are external to the crate
// In fact, Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.

// No need for the #[cfg(test)] annotation for integration tests, Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

/* IMPORTANT: IT FAILS FAST
The three sections of output include the unit tests, the integration test, and the doc tests. Note that if any test in a section fails, the following sections will not be run. 
For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
*/

// The integration tests section starts with the line Running tests/integration_test.rs. Next, there is a line for each test function in that integration test

// In order to run only the integration tests:
// $ cargo test --test integration_test
/*
Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 
*/

// Submodules in Integration Tests

// For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files:
// However, cargo test does something funny, it treats common.rs as yet another integration test file and runs it separately.
// We do not want common.rs to be executed by the test runner, in the test results with running 0 tests displayed. We just wanted to share some code with the other integration test files.

// To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs.
// Call the common module defined in the folder common/mod.rs in the tests/integration_test.rs file.

// Integration Tests for Binary Crates

// If the project only contains src/main.rs, and doesn't have a src/lib.rs then we cannot create integration tests in the tests directory as we did above, and bring functions defined in the src/main.rs file into scope with a use statement
// Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with the use keyword, to import the modules from the lib.rs file.
