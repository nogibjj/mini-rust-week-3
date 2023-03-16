Title: Simple Task Manager

Description: This mini project is a command-line interface (CLI) application that allows users to manage a list of tasks. Users can add, list, and remove tasks.

Usage:

Add a new task: cargo run -- add "Example task"
List tasks: cargo run -- list
Remove a task by ID: cargo run -- remove 1
This application stores tasks in a tasks.json file. It uses Serde for serialization
