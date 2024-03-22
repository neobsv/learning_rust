// Object Oriented Programming Features

// Lets explore some of the characteristics that are commonly considered object orientd and how to implement object oriented
// design patterns in Rust, and discuss the tradeoffs of doing so versus implementing a solution using other constructs which are considered Rust's strengths.

// OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.

// Objects Contain Data and Behavior

// According to the Gang of Four book,
// An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.

// Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums. 
// Even though structs and enums with methods aren’t called objects, they provide the same functionality.

// Encapsulation that Hides Implementation Details

// We can use the pub keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private.

// Example: A struct with a vector and an "average" i32 variable, which always holds the average of the values in the vector.

// The struct is marked pub so that other code can use it, but the fields within the struct remain private. 
// This is important in this case because we want to ensure that whenever a value is added or removed from the list, the average is also updated.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Encapsulation: The public methods add, remove, and average are the only ways to access or modify data in an instance of AveragedCollection.
// We leave the list and average fields private so there is no way for external code to add or remove items to or from the list field directly; otherwise, the average field might become out of sync when the list changes.
// The implementation details of AveragedCollection are free to change, for example, we can replace Vec<T> with a HashSet<T>, and as long as the signatures add, remove and average remain the same, we don't need to change any code
// that uses AveragedCollection. However, if the inner list was public, then changing Vec<T> to HashSet<T> would break code that is using the public member list directly.

// Inheritance as a Type System and Code Sharing

// Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior.
// There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

/* Code Reuse WITHOUT Inheritance

You would choose inheritance for two main reasons. 

1. CODE REUSE: You can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type. 
You can do this in a limited way in Rust code using default trait method implementations. Any type implementing the Summary trait would have the same summarize() method available on it without any further code. 
This is similar to a parent class having an implementation of a method and an inheriting child class also having the implementation of the method.
We can also override the default implementation of the summarize() method when we implement the Summary trait, which is similar to a child class overriding the implementation of a method inherited from a parent class.

2. POLYMORPHISM: To enable a child type to be used in the same places as the parent type. Which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.
Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called BOUNDED PARAMETRIC POLYMORPHISM.

*/

// NOTE: Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary.
// NOTE: Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance. 
// NOTE: This can make a program’s design less flexible. It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass.

// For these reasons, Rust takes the different approach of using TRAIT OBJECTS instead of inheritance. Let’s look at how trait objects enable polymorphism in Rust.

// Using Trait Objects That Allow for Values of Different Types

// A workaround defined a SpreadsheetCell enum that had variants to hold integers, floats, and text. This meant we could store different types of data in each cell and still have a vector that represented a row of cells.
// This is a perfectly good solution when our interchangeable items are a FIXED SET of types 

// Sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.
// We’ll create an example graphical user interface (GUI) tool that iterates through a list of items, calling a draw() method on each one to draw it to the screen—a common technique for GUI tools. 
// This crate might include some types for people to use, such as Button or TextField. In addition, lets say we want Image and SelectBox.
// We do know that gui needs to keep track of many values of different types, and it needs to call a draw() method on each of these differently typed values, but doesn't need to know what draw() should do for each type.

// To do this in a language with inheritance, we might define a class named Component that has a method named draw on it. The other classes, such as Button, Image, and SelectBox, would inherit from Component and thus inherit the draw method.
// Rust doesn't have inheritance!

// Defining a Trait for Common Behavior

// To implement [ (GUI) tool that iterates through a list of items, calling a draw() method on each one ], we’ll define a trait named Draw that will have one method named draw. Then we can define a vector that takes a trait object. 
// A TRAIT OBJECT points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.
// We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait.

/* IMPORTANT: TRAIT OBJECTS

Syntax: Box<dyn TRAIT_NAME>

We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects. 
In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object. 
However, trait objects are more like objects in other languages in the sense that they combine data and behavior. 
But trait objects differ from traditional objects in that we can’t add data to a trait object. 
Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

*/

// Create the TRAIT first,
pub trait Draw {
    fn draw(&self);
}

// A struct named Screen that holds a vector named components. This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // This is a TRAIT OBJECT
}

// This works differently from defining a struct that uses a generic type parameter with trait bounds. 
// A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*

This is what we would have done to implement it using TRAIT BOUNDS:

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

This RESTRICTS us to a Screen instance that has a list of components all of type Button or all of type TextField. 
If you’ll only ever have HOMOGENEOUS collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.

*/

// With the method using trait objects, one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>!

// Implementing the Trait

// Now we’ll add some types that implement the Draw trait. We’ll provide the Button type.

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("draw a button |-OK-| !!");
    }
}

// The width, height, and label fields on Button will differ from the fields on other components; for example, a TextField type might have those same fields plus a placeholder field. 
// Each of the types we want to draw on the screen will implement the Draw trait but will use different code in the draw method to define how to draw that particular type, as Button has here.

// Implement the Draw trait on the SelectBox type as well:

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("draw me a select-[___]-box!!")
    }
}

fn main() {

    // Our library’s user can now write their main function to create a Screen instance. 
    // To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object. 
    // They can then call the run method on the Screen instance, which will call draw on each of the components. 

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // When we wrote the library, we didn’t know that someone might add the SelectBox type, 
    // but our Screen implementation was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it implements the draw() method.

    // This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of duck typing in dynamically typed languages: 
    // If it walks like a duck and quacks like a duck, then it must be a duck! This is what python does everywhere, and part of the reason why it is not performant!

    // By specifying Box<dyn Draw> as the type of the values in the components vector, we’ve defined Screen to need values that we can call the draw method on.

    /* ERROR: Won't compile because Box<String> doesn't implement the Draw trait!
        let screen = Screen {
            components: vec![Box::new(String::from("Hi"))],
        };
    */

    // Trait Objects Perform Dynamic Dispatch
    
    // Dynamic Dispatch: which is when the compiler can’t tell at compile time which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
    // When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. 
    // Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn’t occur with static dispatch.

    main2();

    main3();

}

// Implementing an Object Oriented Design Pattern

// State Pattern == Finite State Machine, can be represented by a Directed Graph!

// The state pattern is an object-oriented design pattern. 
// The crux of the pattern is that we define a set of states a value can have internally. 
// The states are represented by a set of state objects, and the value’s behavior changes based on its state. 
// We’re going to work through an example of a blog post struct that has a field to hold its state, which will be a state object from the set "draft", "review", or "published".
// The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.


// The advantage of using the state pattern is that, when the business requirements of the program change, we won’t need to change the code of the value holding the state or the code that uses the value. 
// We’ll only need to update the code inside one of the state objects to change its rules or perhaps add more state objects.

// Functionality will look like this:
    // 1. A blog post starts as an empty draft.
    // 2. When the draft is done, a review of the post is requested.
    // 3. When the post is approved, it gets published.
    // 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.


// We know we need a public Post struct that holds some content
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // We want to be able to call a method named add_text and pass it a &str that is then added as the text content of the blog post. 
    // We implement this as a method, rather than exposing the content field as pub, so that later we can implement a method that will control how the content field’s data is read.
    // The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling add_text on.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Even after we’ve called add_text and added some content to our post, we still want the content method to return an empty string slice because the post is still in the draft state.
    pub fn content(&self) -> &str {
        // We want the value returned from content to depend on the current state of the Post, so we’re going to have the Post delegate to a content method defined on its state.
        // Call the content method on the value in state and pass the post instance (that is, self) as an argument. Then we return the value that’s returned from using the content method on the state value.
        // We need to add content to the State trait definition, and that is where we’ll put the logic for what content to return depending on which state we have
        self.state.as_ref().unwrap().content(self)
    }

    // Requesting review of a post changes its state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // We call an internal request_review method on the current state of Post, and this second request_review method consumes the current state and returns a new state.
            self.state = Some(s.request_review())
            // To consume the old state, the request_review method needs to take ownership of the state value. This is where the Option in the state field of Post comes in: 
            // we call the take() method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. 
            // This lets us move the state value out of Post rather than borrowing it. Then we’ll set the post’s state value to the result of this operation.
        }

        // We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value. 
        // This ensures Post can’t use the old state value after we’ve transformed it into a new state.

    }
    
    // Adding approve() to Change the Behavior of content
    // The approve method will be similar to the request_review method: it will set state to the value that the current state says it should have when that state is approved
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

}

// The State trait defines the behavior shared by different post states. 
// The state objects are Draft, PendingReview, and Published, and they will all implement the State trait.
trait State {
    // We add the request_review method to the State trait; all types that implement the trait will now need to implement the request_review method.
    // We have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type. This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;


    // We add a default implementation for the content method that returns an empty string slice. That means we don’t need to implement content on the Draft and PendingReview structs.
    // The Published struct will override the content method and return the value in post.content.
    // We’re taking a reference to a post as an argument and returning a reference to part of that post, so the lifetime of the returned reference is related to the lifetime of the post argument.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// We’ll start by defining just the Draft state because that is the state we want a post to start in.
struct Draft {}

impl State for Draft {
    // The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review. 
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Similar to the way request_review on PendingReview works, if we call the approve method on a Draft, it will have no effect because approve will return self.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview{}

impl State for PendingReview {
    // The PendingReview struct also implements the request_review method but doesn’t do any transformations, it just returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // When we call approve on PendingReview, it returns a new, boxed instance of the Published struct. The Published struct implements the State trait, and for both the request_review method and the approve method, it returns itself.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Now we can start seeing the advantages of the state pattern: the request_review method on Post is the same no matter its state value. Each state is responsible for its own rules.

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// When we create a new Post, we set its state field to a Some value that holds a Box. 
// This Box points to a new instance of the Draft struct. This ensures whenever we create a new instance of Post, 
// it will start out as a draft. Because the state field of Post is private, there is no way to create a Post in any other state! 

// In the Post::new function, we set the content field to a new, empty String.

fn main2() {

    let mut post = Post::new();

    // We want to allow the user to create a new draft blog post with Post::new. We want to allow text to be added to the blog post. 
    // If we try to get the post’s content immediately, before approval, we shouldn’t get any text because the post is still a draft.

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // We want to enable a request for a review of the post, and we want content to return an empty string while waiting for the review.
    post.request_review();
    assert_eq!("", post.content());

    // The only type we’re interacting with from the crate is the Post type. This type will use the state pattern and will hold a value that will be 
    // one of three state objects representing the various states a post can be in—draft, waiting for review, or published. 

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}

// Why didn't we use an enum?
// One disadvantage of using an enum is every place that checks the value of the enum will need a match expression or similar to handle every possible variant. This could get more repetitive than this trait object solution.

// Trade offs of the State Pattern

// Merits:
// The way we organized the code, we have to look in only one place to know the different ways a published post can behave: the implementation of the State trait on the Published struct.
// Using match expressions instead of the state pattern would be lengthy and messy and have a lot of match arms!
// And to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.

// The implementation using the state pattern is easy to extend to add more functionality. To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:
    // 1. Add a reject() method that changes the post’s state from PendingReview back to Draft. (just need to add it in the State trait and the states where it is needed)
    // 2. Require two calls to approve before the state can be changed to Published. (modify PendingReview to have a call counter and use it in the request_review() method)
    // 3. Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

// Demerits:
// Tight Coupling: Because the states implement the transitions between states, some of the states are coupled to each other. If we add another state between PendingReview and Published, such as Scheduled, we would have to change the code in PendingReview to transition to Scheduled instead.
// Duplicated Logic: To eliminate some of the duplication, we might try to make default implementations for the request_review() and approve methods on the State trait that return self; however, this would violate object safety, because the trait doesn’t know what the concrete self will be exactly.
// Duplicated method implementation: Other duplication includes the similar implementations of the request_review and approve methods on Post. Both methods delegate to the implementation of the same method on the value in the state field of Option and set the new value of the state field to the result. 
// If we had a lot of methods on Post that followed this pattern, we might consider defining a macro to eliminate the repetition.

// By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could

// Encoding States and Behaviors as Types

// We’ll show you how to rethink the state pattern to get a different set of trade-offs. Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, 
// We’ll encode the states into different types. Consequently, Rust’s type checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

// We still enable the creation of new posts in the draft state using Post::new and the ability to add text to the post’s content. But instead of having a content method on a draft post that returns an empty string, 
// we’ll make it so draft posts don’t have the content method at all. That way, if we try to get a draft post’s content, we’ll get a compiler error telling us the method doesn’t exist. As a result, it will be impossible for us to accidentally display draft post content in production.

pub struct PostII {
    content: String,
}

pub struct DraftPostII {
    content: String,
}

// Both the Post and DraftPost structs have a private content field that stores the blog post text. 
// The structs no longer have the state field because we’re moving the encoding of the state to the types of the structs. 
// The Post struct will represent a published post, and it has a content method that returns the content.

impl PostII {

    // Post::new function, but instead of returning an instance of Post, it returns an instance of DraftPost. Because content is private and there 
    // aren’t any functions that return Post, it’s not possible to create an instance of Post right now.
    pub fn new() -> DraftPostII {
        DraftPostII {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

// So how do we get a published post? We want to enforce the rule that a draft post has to be reviewed and approved before it can be published. 
// A post in the pending review state should still not display any content. Let’s implement these constraints by adding another struct, PendingReviewPost, 
// defining the request_review() method on DraftPost to return a PendingReviewPost, and defining an approve method on PendingReviewPost to return a Post.

impl DraftPostII {
    // The DraftPost struct has an add_text method, so we can add text to content as before, but note that DraftPost does not have a content method defined! 
    // So now the program ensures all posts start as draft posts, and draft posts don’t have their content available for display
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPostII {
        PendingReviewPostII {
            content: self.content,
        }
    }

}

/*

    IMPORTANT: Encoding the workflow into the methods of the different types

    Workflow:  Post::new() -> DraftPost -> request_review() -> PendingReviewPost -> approve() -> Post

    The request_review and approve methods take ownership of self, thus consuming the DraftPost and PendingReviewPost instances and transforming them into a PendingReviewPost and a published Post, respectively.
    This way, we won’t have any lingering DraftPost instances after we’ve called request_review on them, and so forth. The PendingReviewPost struct doesn’t have a content method defined on it, so attempting to read its 
    content results in a compiler error, as with DraftPost. Because the only way to get a published Post instance that does have a content method defined is to call the approve method on a PendingReviewPost, and the only 
    way to get a PendingReviewPost is to call the request_review method on a DraftPost, we’ve now encoded the blog post workflow into the type system.

*/

pub struct PendingReviewPostII {
    content: String,
}

impl PendingReviewPostII {
    pub fn approve(self) -> PostII {
        PostII {
            content: self.content,
        }
    }
}

fn main3() {

    // But we also have to make some small changes to main. The request_review and approve methods return new instances rather than modifying the struct they’re called on, 
    // so we need to add more let post = shadowing assignments to save the returned instances.

    let mut post = PostII::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}

// Not strictly Object Oriented:
// The changes we needed to make to main to reassign post mean that this implementation doesn’t quite follow the object-oriented state pattern anymore: the transformations between the states are no longer encapsulated entirely within the Post implementation.

// Type System Captures Invalid States (big win)!
// Our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time! This ensures that certain bugs, such as display of the content of an unpublished post, will be discovered before they make it to production.

/*

    THINK DIFFERENT!

    Although you might be very familiar with object-oriented patterns, rethinking the problem to take advantage of Rust’s features can provide benefits, such as preventing some bugs at compile time.
    Object Oriented Patterns won't always be the best solution in Rust due to features like ownership that other OO languages don't have!

*/