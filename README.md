# Calculator Application
---
A simple calculator built using the Slint UI toolkit and Rust. This calculator provides basic arithmetic operations and a user-friendly interface.
---
## Features
- Arithmetic Operations: Supports addition, subtraction, multiplication, and division.

- Special Buttons:
  - C: Clears all inputs.

  - CE: Clears the last entered digit.

  - %: Calculates the percentage.

  - =: Computes the result.



- Decimal Support: Includes a decimal point for fractional calculations.

- Responsive Design: Interactive UI with hover and click effects on buttons.

---
## UI Components
  ### Buttons
  - Number buttons (0-9, 00): Input numbers.

  - Operator buttons (+, -, x, /): Perform calculations.

  - Special buttons (C, CE, %, .): Provide additional functionalities.
   ### Display

   - Displays the current input or the result of the calculation.
---
## Code Structure

### Slint UI Code

The UI is defined using the Slint language:

- Grid Layout: Organizes buttons and display.

- Custom Button Component: Defines button styles and click behaviors.

- Dynamic Styling: Buttons change color on hover and press.

### Rust Logic

- State Management: Handles current value, operator, decimal status, and new input flag.

- Event Handling: Processes button clicks using callbacks.

- Calculation Logic: Performs arithmetic operations and updates the display.

  ---
## Setup and Usage

 Prerequisites

- Rust installed on your system. Install Rust

- cargo package manager (bundled with Rust).
### Installation
1. Clone the repository:
   ``` bash
   git clone https://github.com/Adez017/Calculator-using-Rust-with-Slin
    cd <repository-folder>
   ```
2. Build the project:
   ``` bash
   cargo build
   ```
 ### Run the Application
 1. Start the application:
    ``` bash
      cargo run
    ```
 2. Interact with the calculator using the UI.
--- 
### Preview:
![Example Image](https://github.com/Adez017/Calculator-using-Rust-with-Slint/blob/main/Screenshot%202024-12-10%20212413.png)

---

### Future Improvements

- Add advanced operations like square root and power.

- Implement history tracking for past calculations.

- Enhance UI with additional themes and customizations.

---

### Contributing

Contributions are welcome! Please fork the repository and submit a pull request.






