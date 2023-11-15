# Multimedia File Cleanup Rust Script

## Overview

This Rust script is designed to help you clean up multimedia files within a specified directory and its subdirectories. It identifies and optionally deletes multimedia files based on their file extensions.

## Features

- Recursively scans a directory and its subdirectories.
- Identifies multimedia files based on their extensions.
- Deletes identified multimedia files (optional).
- Provides statistics on the number of files and folders processed.

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.
- A directory path as a command-line argument.

## Usage

1. Clone the repository or download the script.
2. Open your terminal or command prompt.
3. Navigate to the directory containing the script.
4. Run the script by providing the directory path as an argument:

   ```bash
   cargo run --release -- /path/to/directory
