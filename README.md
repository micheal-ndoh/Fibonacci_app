
# Fibonacci Calculator

## Overview

The Fibonacci Calculator is a cross-platform desktop application built using Rust and Tauri. It allows users to input a number and computes its Fibonacci value,

## Features

- User friendly interface with an input field and a button to calculate Fibonacci numbers.
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

## Usage

- Enter a number in the input field and click the "Calculate" button to calculate the Fibonacci number.
- The result will be displayed below the input field.
- You can input multiple numbers in sequence.

## Error Handling

The application ensures that only valid numbers are processed. If an invalid input is detected, an appropriate error message will be displayed.

## Bonus Features

- The application can be optimized for performance.
- A feature to compute multiple Fibonacci numbers in sequence can be added.
- This application can be packaged as an installable `.exe`, `.dmg`, or `.AppImage`.
