### **Assignment: Fibonacci Calculator - Desktop Application in Rust**  

#### **Objective:**  

Create a cross-platform desktop application that takes a user-inputted number and computes its Fibonacci value using Rust and WebAssembly (WASM).  

#### **Requirements:**  

1. **User Interface:**  
   - The application should have a simple UI with an input field, a button, and a section to display the Fibonacci result.  

2. **Rust & WASM Implementation:**  
   - Implement the Fibonacci calculation in Rust.  
   - Use **Tauri** to create the desktop app (Tauri uses Rust as the backend and lightweight web technologies for the frontend).  
   - Call the Rust function from the UI to compute Fibonacci.  

3. **Error Handling:**  
   - Ensure the user can only input valid numbers.  
   - Display appropriate messages for invalid input.  

#### **Bonus Challenge (Optional):**  

- Optimize Fibonacci calculation for performance.  
- Add a button to compute multiple Fibonacci numbers in sequence.  
- Package the application as an installable `.exe`, `.dmg`, or `.AppImage`.
