// Advanced Traits

fn main() {

    // Specifying Placeholder Types in Trait Definitions
    // Associated types connect something called a "placeholder type" with a trait such that the trait method definitions can use these types in their signatures.
    // The implementor of a trait will specify the concrete type to be used instead of the type for the particular implementation.

    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // The type Item is a placeholder, and the next method’s definition shows that it will return values of type Option<Self::Item>. 
    // Implementors of the Iterator trait will specify the concrete type for Item.

    // We’ll look at an implementation of the Iterator trait on a type named Counter that specifies the Item type is u32:

    struct Counter {
        count: u32,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    
    impl Iterator for Counter {
        type Item = u32; // Specifying the concrete type for the placeholder type
    
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // So then, why don't we just use generics? Like this:

    pub trait IteratorO<T> {
        fn next(&mut self) -> Option<T>;
    }

    // IMPORTANT: The difference is that when using generics, we must annotate the types in each implementation; 
    // because we can also implement Iterator<String> for Counter or any other type, we could have MULTIPLE IMPLEMENTATIONS of Iterator for Counter.

    // In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. 
    // When we use the next method on Counter, we would have to provide which implementation of Iterator we want to use.

    // With associated types, we don’t need to annotate types because we CANNOT implement a trait on a type MULTIPLE times.
    // We can only choose what the type of Item will be once, because there can only be one impl Iterator for Counter. 
    // We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
    // This way, Associated types also become part of the trait’s contract: meaning we need to specify the "placeholder type", so it is more rigid.


    // Default Generic Type Parameters and Operator Overloading

    // When we use generic type parameters, we can specify a default concrete type for the generic type.
    // You specify a default type when declaring a generic type with the <T=ConcreteType> syntax.

    // A great example of a situation where this technique is useful is with operator overloading, in which you customize the behavior of an operator (such as +) in particular situations.

    // Rust doesn't allow operator overloading directly, you can implement the traits listed under std::ops in order to do operator overloading.
    // Example of operator overloading for Add:

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // The add method adds the x values of two Point instances and the y values of two Point instances to create a new Point.
    // The associated type placeholder is called 'Output' here, which is set to Point appropriately.

    // Default generic type within the Add trait:

    trait AddO<Rhs=Self> {
        type Output;
    
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // Default Generic type is  Rhs=Self: this syntax is called default type parameters. If the type is not specified while implementing the Add trait, 
    // the type of Rhs will default to Self, which will be the type we’re implementing Add on.
    // When we implemented Add for Point, we used the default for Rhs because we wanted to add two Point instances so the "other" object passed in was also a Point.


    // We have two structs, Millimeters and Meters, holding values in different units. 
    // This thin wrapping of an existing type in another struct is known as the NEWTYPE PATTERN. 
    // We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly. 
    // We can implement Add for Millimeters with Meters as the Rhs, and this is possible due to Default Generic Type parameters.

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // You’ll use default type parameters in two main ways:
        // 1. To extend a type without breaking existing code
        // 2. To allow customization in specific cases most users won’t need

    // The standard library’s Add trait is an example of the second purpose: usually, you’ll add two like types, but the Add trait provides the ability to customize beyond that. 
    // The first purpose is similar to the second but in reverse: if you want to add a type parameter to an existing trait, you can give it a default to allow extension of the functionality.

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

    // Nothing in Rust prevents a trait from having a method with the same name as another trait’s method. 
    // Also, Rust does not prevent you from implementing two traits with the same name on one type.

    // Example: We’ve defined two traits, Pilot and Wizard, that both have a method called fly. We then implement both traits on a type Human that already has a method named fly implemented on it. Each fly method does something different.

    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    
    // IMPORTANT: When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type.

    let person = Human;
    person.fly();

    // Running this code will print *waving arms furiously*, showing that Rust called the fly method implemented on Human directly.

    // To call the fly methods from either the Pilot trait or the Wizard trait, we need to specify the trait name before the method name:
    Pilot::fly(&person);
    Wizard::fly(&person);

    // However, associated functions that are not methods don’t have a self parameter. When there are multiple types or traits that define non-method functions 
    // with the same function name, Rust doesn't always know which type you mean unless you use fully qualified syntax.

    // Lets say we have a trait Animal and a struct Dog. We will implement a funciton baby_name() for the struct Dog by implementing the Animal trait for it.
    // We will implement the function baby_name() for the struct Dog directly using an impl block as well. 

    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // Now, We want to call the baby_name function that is part of the Animal trait that we implemented on Dog so the code prints A baby dog is called a puppy:

    // println!("A baby dog is called a {}", Animal::baby_name()); ERROR: there is no self param for the baby name function so this call is ambiguous
    // Rust can’t figure out which implementation of Animal::baby_name to call, because there can be other types which implement the Animal trait that will have the baby_name() function

    // Tell Rust that we want to use the implementation of Animal for Dog as opposed to the implementation of Animal for some other type, we need to use FULLY QUALIFIED SYNTAX:
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
    // Type annotation within the angle brackets, which indicates we want to call the baby_name() method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call.

    // FULLY QUALIFIED SYNTAX: <Type as Trait>::function(receiver_if_method, next_arg, ...);

    // Using SuperTraits to Require One Trait's Functionality Within Another Trait

    // You might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait.
    // In this case, the parent trait that your second trait is relying on is called the supertrait.

    // Example: Pretty Printing
    // Let’s say we want to make an OutlinePrint trait with an outline_print() method that will print a given value formatted so that it's framed in asterisks. 
    // That is, given a Point struct that implements the standard library trait Display to result in (x, y), when we call outline_print() on a Point instance:

    /*
    **********
    *        *
    * (1, 3) *
    *        *
    **********
    */

    // In the implementation of the outline_print() method, we want to use the Display trait’s functionality. Therefore, we need to specify that the OutlinePrint trait will 
    // work only for types that also implement Display and provide the functionality that OutlinePrint needs.

    use std::fmt;

    trait OutlinePrint: fmt::Display { // Here Display is the supertrait to outline print
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string() function that is automatically implemented for any type that implements Display.

    // We try to implement OutlinePrint on a type that doesn’t implement Display, such as the Point struct:

    struct PointII {
        x: i32,
        y: i32,
    }

    /*  ERROR:

        impl OutlinePrint for Point {}

    */
    // ERROR: `Point` cannot be formatted with the default formatter = help: the trait `std::fmt::Display` is not implemented for `Point`

    // To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires, like so:
    
    impl fmt::Display for PointII {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for PointII {}

    let p = PointII {x: 4, y: 5};
    p.outline_print();

    // Using the Newtype Pattern to Implement External Traits on External Types

    // RULE: We’re only allowed to implement a trait on a type if EITHER the Trait or the Type are local to our crate. 
    // It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.

    // The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. 
    // Then the wrapper type is local to our crate, and we can implement the trait on the wrapper.

    // Example: Let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate.
    // We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value:

    struct Wrapper(Vec<String>); // Newtype pattern; this Wrapper is the tuple struct over the type Vec<String>

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // The implementation of Display uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. 
    // We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>.

    // If we wanted the new type to have every method the inner type has, implementing the Deref trait on the Wrapper to return the inner type would be a solution. 
    // If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.



}
