# Banana Script

This guide will walk you through the process of installing Rust on Windows, running the Banana Game from Steam, and using Cargo to run the macro with three flags: hours, minutes, and seconds.

## Prerequisites

Before you begin, make sure you have the following:

- Windows operating system
- Steam installed on your computer
- Internet connection

## Installation

1. **Install Rust:**
    - Visit the official Rust website at [https://www.rust-lang.org/](https://www.rust-lang.org/)
    - Click on the "Install" button and follow the instructions for Windows installation.
    - Once the installation is complete, open the command prompt.

2. **Run Banana Game from Steam:**
    - Launch the Steam application on your computer.
    - Search for "Banana" in the Steam store and click on it.
    - Click on the "Play" button to start the game.

3. **Use Cargo to run the macro:**
    - Open the command prompt and navigate to the directory where you have the Banana Script code.
    - Run the following command to compile and run the macro with the desired flags:
      ```
      cargo run -- --hours <hours> --minutes <minutes> --seconds <seconds>
      ```
      Replace `<hours>`, `<minutes>`, and `<seconds>` with the desired values.

## Example

To run the macro with 2 hours, 30 minutes, and 45 seconds, use the following command:
```
cargo run -- --hours 2 --minutes 30 --seconds 45
```

**Note:** The macro will automatically click the banana every 2 seconds. You can terminate the macro process by pressing `Ctrl+C`.

That's it! You have successfully installed Rust, ran the Banana Game from Steam, and used Cargo to run the macro with the specified flags. Enjoy the Banana Script!
