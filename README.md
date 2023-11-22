# Modulo-Machine

## Description
Rust-based program that effectively elucidates the concepts of multi-threading and modular arithmetic operations. This application underscores its functionality by employing high precision numerical operations using the num_bigint and num_traits crates, uniquely catering to your data computation needs.

The core aspect of the project is the calculate_modulus function which handles arbitrary precision unsigned integer parameters x and p, performing a precise modulus operation on x with respect to p.

At the heart of the program - the main function simultaneously encapsulates multiple functionalities:

It sets the x variable as a shared reference wrapped in a Mutex, holding a 300-bit number.
The variable p is assigned a fixed massive value.
It initializes o to 0 and encapsulates it within a Mutex.
Boasting a unique feature, the program spawns a background thread running an infinite loop via thread::spawn. It cleverly simulates a clock signal every second and continually calculates the modulus of x, assigning the result to o. Simultaneously, the main acts as a watchdog for a 'reset' signaling mechanism.

If a reset signal is detected, the system dynamically updates the values of x and p with the reset values. Mutexes are efficiently utilized to ensure concurrent yet safe access to shared data across threads.

To sum up, this program serves as an expert tool for the computation of the modulus of a 300-bit number x with respect to a persistent value p. It further incorporates reset logic for dynamic data value adjustments, providing an enriching experience for exploring multi-threading and modular arithmetic.

# Getting started

# Features
Modular Arithmetic Functionality:

The project includes a modular arithmetic function (calculate_modulus) that efficiently computes the modulus of two arbitrary precision unsigned integers.

Concurrency with Threads:

The main function initializes key variables (x, p, and o) wrapped in Mutexes to ensure thread safety.
A dedicated background thread simulates a clock signal, calculating the modulus of x with respect to p at one-second intervals.

Dynamic Reset Logic:

The code incorporates a flexible reset mechanism, allowing for the dynamic adjustment of x and p values upon receiving a reset signal.

User Interaction:

The project includes a simple user interaction component, allowing the user to trigger a reset signal through console input.

## Requirements
Make sure you have all the necessary packages to run a rust based program installed.

Install Dependencies:
Ensure you have Rust installed. If not, follow the instructions at https://www.rust-lang.org/tools/install.

## Quickstart

```
git clone https://github.com/Depauli1/Modulo-Machine
cd Modulo Machine
```
Compile the Program:
```
cargo build
```
Run the program:
```
cargo run
```
Adjust the initial values of x and p in the main function as needed.

## Usage
Simulating Clock Signal:

The background thread simulates a clock signal, recalculating the modulus every second.

Resetting Values:

To trigger a reset, input "reset" in the console. This updates the values of x and p with the specified reset values.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

The project makes use of the num_bigint and num_traits crates.
Inspired by the need for a concurrent and modular arithmetic solution.

## Author

Bless Hukporti
