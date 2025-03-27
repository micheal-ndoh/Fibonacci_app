
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

   ```rust
   git clone https://github.com/micheal-ndoh/Fibonacci_app.git
   cd fibonacci-app
   ```

2. **Install Rust and Tauri:**
   Ensure you have Rust installed. Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust. Then, install Tauri by following the [Tauri setup guide](https://tauri.studio/docs/getting-started/intro).

    Install rust using:

    ```rust
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

    If it fails then try installing the dependencies manually and install rust again:

    ```rust
    sudo apt update
    sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
    ```

    Install tauri using:

    ```rust
    cargo install tauri-cli
    ```

3. **Install frontend dependencies:**
   Navigate to the **`src`** directory and run:

   ```rs
   npm install
   ```

4. **Run the application:**
   From the **`src  tauri`** directory, run:

   ```rs
   cargo tauri dev
   ```

    alternatively

    ```rs
    npm run tauri dev
    ```

5. **Build the application:**
   From the **`src`** directory, run:

    ```rs
   cargo tauri build
    ```

   alternatively

   ```rust
   npm run tauri build
   ```

6. **Run the application:**
   Navigate to the **`src-tauri/target/release`** directory and run the generated executable.

   ```sh
   ./fibonacci-calculator
   ```

...

7. **Package the application:**
   Firstly list all the targets for which you want to build the application using and install the one you wish to using to build the application.

   ```sh
   rustup target list
   ```

    Install/add using

    ```rust
    rustup target add <name of target>
    ```

   Then from the **`src`** directory, run the following commands for each target:

   For macOS (.dmg):

   ```rs
   cargo tauri build --target x86_64-apple-darwin
   ```

   For Windows (.exe):

   ```rs
   cargo tauri build --target x86_64-pc-windows-gnu
   ```

   For Linux (.AppImage):

   ```rs
   cargo tauri build --target x86_64-unknown-linux-gnu
   ```

8. **Install the application on Linux:**
   Ensure all required dependencies are installed:

   ```sh
   sudo apt update
   sudo apt --fix-broken install
   sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0
   ```

   Then install the `.deb` , `rpm` or appimage package use the following command:

   ```sh
   sudo dpkg -i fibonacci-calculator_0.1.0_amd64.deb
   ```

   ```sh
   sudo rpm -i fibonacci-calculator-0.1.0.x86_64.rpm
   ```

    Make the appimage executable and run;

   ```sh

   sudo chmod +x fibonacci-calculator.AppImage
   ./fibonacci-calculator.AppImage
   ```

9. **Install the application on Windows:**
   Run the `.exe` file.

10. **Install the application on macOS:**
   Run the `.dmg` file.

## Usage

- Enter a number in the input field and click the "Calculate" button to calculate the Fibonacci number.
- The result will be displayed below the input field.
- You can input multiple numbers in sequence but separate them by commas.

## Error Handling

The application ensures that only valid numbers are processed and if an invalid number is entered, an error message is displayed but it was handle so that the user can try again by entering a valid number.

## Bonus Features

- Added multiple calculation feature to compute multiple Fibonacci numbers in sequence.
- This application can be packaged as an installable `.exe`, `.dmg`, or `.AppImage`.
- The application can be optimized for performance.
- The application can be distributed as a desktop application.

## TODO

- Add more features to the application.
- Optimize the application for performance.
- optimize the application for size.
- Add more languages to the application.
- Add an icon for the application.

## DONE

- Add more themes to the application.
  
## Feedback

If you have any feedback or suggestions, please let me know.
reach me via [gmail](michaelndoh9@gmail.com)
leave a Star on this repository.

# Thank you

# Check your applications and click on the fibonacci calculator to get started
