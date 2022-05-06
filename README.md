# Overview

This project was developed so that users can analyze the speed at which their system can 
sequentially multiple two large matrixes. This can be used to test single-core system speeds, 
and in future versions, it can be used to test and time multicore system speeds. The test is 
simple and easy to use,  only open the program, and run the test, as two random matrixes will 
be generated and multiplied together for the user.


[Software Demo Video](https://www.loom.com/share/992477535c2e477bbac04c4ce6b0c050)

# Development Environment

For this project, I used Visual Studio Code utilizing the Rust-analyzer to debug the systems. 
Some of the cargo dependencies include clap version 3.1.14, rayon version 1.5.1, and rand 
version 0.8.4. And lastly, I am using the 64-bit version of Rust on my system.

# Useful Websites

{Make a list of websites that you found helpful in this project}
* [Web Site Name](https://users.rust-lang.org/)
* [Web Site Name](https://stackoverflow.com/)
* [Web Site Name](https://www.tutorialspoint.com/rust/rust_iterator_and_closure.htm)

# Future Work

* Implement Multithreading
* Add a timer to measure the time it takes to multiple both sequentially and by parallel
* Add a UI so the user can enter matrix test sizes.
