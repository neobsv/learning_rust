// Building a Multithreaded Webserver

// Single Threaded webserver:

// We need to look at TCP and HTTP protocols, and both these are request-response protocols,
// meaning a client initiates requests and a server listens to the requests and provides responses.

// TCP is the lower level protocol, it works by establishing connections between operating system ports of two machines,
// where on the server side the server "binds" (attaches/connects) to the port and listens for connections while on the other side the client sends
// messages through the open port to the server. This runs over IP, which takes the messages through the physical network.

// HTTP builds on top of TCP and works by communicating from the applicaiton to the operating system port which is handled by TCP.

// Listening to the TCP Connection:

// The std::net module allows us to create a tcp listener, which binds to port 7878, and we chose this port because it is a non
// standard port that doesn't conflict with anything, and also "7-8-7-8" is "r-u-s-t" when typed on a telephone dialer!

use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {

    // NOTE: unwrap() is used a lot here and this hides possible errors and just fails, ideally, for production ready code all errors must be handled properly.

    // The bind() function in this scenario works like the new function because it returns a new TcpListener instance.
    
    // The bind function returns a Result<T, E>, which indicates that it’s possible for binding to fail, for example we can't use
    // ports 0-1023 without admin privileges on most systems, so it may fail.

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Using unwrap to stop the program if errors happen
    
    // The incoming() method on TcpListener returns an iterator that gives us a sequence of TcpStream s. A single stream represents an open connection between the client and the server.
    // A connection is the name for the full request and response process in which a client connects to the server, the server generates a response, 
    // and the server closes the connection. As such, we will read from the TcpStream to see what the client sent and then write our response to the stream
    // to send data back to the client. Overall, this for loop will process each connection in turn and produce a series of streams for us to handle.

    // We are actually iterating over connection attempts rather than actual connections. The connection might not be successful for a number of reasons, 
    // many of them operating system specific, hence we call unwrap over here to fail in case any error happens.
    
    /*
        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection Established!");
        }
    */

    // When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the Drop trait implementation. 

    // Navigate to localhost:7878 in a browser, you'll see the "Connection Established!" message however the webpage will be blank.


    // Reading the Request

    // In this new handle_connection() function, we’ll read data from the TCP stream and print it so we can see the data being sent from the browser.

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection_with_validation(stream);
    }

    // A Closer Look at an HTTP Request

    /*
        Method Request-URI HTTP-Version CRLF
        headers CRLF
        message-body 
    */

    // The first line is the request line which contains the HTTP method being used, the URI (not URL which contains hostname) and the HTTP version
    // The last part before the message is the CRLF (carriage return line feed) (terms come from typewriters lol!), which is the sequence "\r\n" which acts as a seperator
    // After the request line, the remaining lines starting from Host: onward are headers. GET requests have no body.
    // Try sending a request to an invalid URI like localhost:7878/test , to see how the response changes.

    // Writing a Response

    // HTTP responses have the format:
    /* 
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
    */

    // The version is specified, then a status code which indicates the success or failure of the request received along with phrase as to why it succeeded/failed.
    // Status Code: 200 and Reason Phrase: OK is the standard response for a GET request that succeeds.
    // Once the server starts sending a response, you'll see a blank page instead of an error on the browser.

    // Returning Real HTML

    // Create a file named index.html in the crate root.

    // To return this document, we will make some modifications to the handle connection function.

    // We’re ignoring the request data in http_request and just sending back the contents of the HTML file unconditionally. That means if you try requesting 127.0.0.1:7878/something-else in your browser, you’ll still get back this same HTML response.


    // Validating the Request and Selectively Responding

    // Refer to the handle_connection_with_validation function below

    // Currently, our server runs in a single thread, meaning it can only serve one request at a time. Let’s examine how that can be a problem by simulating some slow requests.




}


fn _handle_connection(mut stream: TcpStream) {
    // We create a new BufReader instance that wraps a mutable reference to the stream. BufReader adds buffering by managing calls to the std::io::Read trait methods for us.
    // BufReader implements the std::io::BufRead trait, which provides the lines method. The lines method returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte.
    let buf_reader = BufReader::new(&mut stream);
    

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())// unwrap the Result<String, std::io::Error> into just String
        .take_while(|line| !line.is_empty())// The browser signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    // fs is the std lib's filesystem module, and it allows us to read files
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    // We use format! to add the file’s contents as the body of the success response. To ensure a valid HTTP response, we add the Content-Length header set to len(index.html)
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // We call as_bytes() on our response to convert the string data to bytes. The write_all method on stream takes a &[u8] and sends those bytes directly down the connection.
    stream.write_all(response.as_bytes()).unwrap();

}


fn handle_connection_with_validation(mut stream: TcpStream) {

    // We’re only going to be looking at the first line of the HTTP request, so rather than reading the entire request into a vector, we’re calling next to get the first item from the iterator. 
    // The first unwrap takes care of the Option and stops the program if the iterator has no items. The second unwrap handles the Result unpacks it into a String, fails if it can't be read as a String.

    // Check the request line to see if it matches the GET / path and then return index.html, if it does not, then return a 404 NOT FOUND as the response line and a 404.html

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    /* Bad Code: Lots of repetition, we need to keep it DRY
        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("index.html").unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
    */

    // Refactoring

    // Above, the if and else blocks are both reading files and writing the contents of the files to the stream. The only differences are the status line and the filename.


    // Now the if and else blocks only return the appropriate values for the status line and filename in a tuple; we then use destructuring to assign these two values to status_line and filename using a let pattern.
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // The previously duplicated code is now outside the if and else blocks and uses the status_line and filename variables
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();



}