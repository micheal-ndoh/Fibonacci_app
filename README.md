
# Fibonacci Calculator

## Overview

The Fibonacci Calculator is a cross-platform desktop application built using Rust and Tauri. It allows users to input a number and computes its Fibonacci value,

## Features

- User friendly interface with an input field and a button to calculate fibonacci numbers.
- Error handling to ensure only valid numbers are processed.
- Built with Rust for performance and reliability.
- Utilizes Tauri for a lightweight desktop application.

## Setup Instructions

1. **Clone the repository:**

   ```
   git clone https://github.com/micheal-ndoh/Fibonacci_app.git
   cd fibonacci-app
   ```

2. **Install Rust and Tauri:**
   Ensure you have Rust installed. Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust. Then, install Tauri by following the [Tauri setup guide](https://tauri.studio/docs/getting-started/intro).

3. **Install frontend dependencies:**
   Navigate to the **`src`** directory and run:

   ```
   npm install
   ```

4. **Run the application:**
   From the **`src  tauri`** directory, run:

   ```
   cargo tauri dev
   ```

or

    ```rust
    npm run tauri dev
    ```
    
5. **Build the application:**
   From the **`src`** directory, run:

   ```
   cargo tauri build
   ```

or

```rust
npm run tauri build
```

6. **Run the application:**
   Navigate to the **`src-tauri/target/release`** directory and run the generated executable.

   ```
   ./fibonacci-calculator
   ```

...

7. **Package the application:**
   Firstly list all the targets for which you want to build the application using.

   ```rust
    rustup target list
   ```

   Then from the **`src`** directory, run the following commands for each target:

   For macOS (.dmg):
   ```
   cargo tauri build --target x86_64-apple-darwin
   ```

   For Windows (.exe):
   ```
   cargo tauri build --target x86_64-pc-windows-gnu
   ```

   For Linux (.AppImage):
   ```
   cargo tauri build --target x86_64-unknown-linux-gnu
   ```

...

## Usage

- Enter a number in the input field and click the "Calculate" button to calculate the Fibonacci number.
- The result will be displayed below the input field.
- You can input multiple numbers in sequence but seoparate them by commas or spaces.

## Error Handling

The application ensures that only valid numbers are processed and if an invalid number is entered, an error message is displayed but it was handle so that the user can try again by entering a valid number.

## Bonus Features

- Added multiple calculationfeature to compute multiple Fibonacci numbers in sequence.
- This application can be packaged as an installable `.exe`, `.dmg`, or `.AppImage`.
- The application can be optimized for performance.
