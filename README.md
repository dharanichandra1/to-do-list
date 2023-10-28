Rust To-Do List App
Introduction
The Rust To-Do List App is a command-line application designed to help users manage their tasks and stay organized. It is built using the Rust programming language, taking advantage of Rust’s safety, performance, and cross-platform capabilities.

Features
Task Management: Users can create, update, delete, and complete tasks. Each task is associated with a description and a status (active or completed).

List Tasks: The app allows users to list all tasks, view only active tasks, or filter tasks based on their completion status. This feature provides users with a clear overview of their to-do list.

Persistent Storage: Task data is stored in a local JSON file. This ensures that tasks persist across application sessions.

Usage
Adding a Task
Users can add a new task to the list by using the add command followed by the task description.
> add Buy groceries
Task added: 'Buy groceries'

Updating a Task
To update an existing task, users can use the update command with the task’s ID and the new task description.
> update 1 Buy milk
Task updated: 'Buy milk'

Completing a Task
Users can mark a task as completed using the complete command, specifying the task’s ID.
> complete 1
Task completed: 'Buy milk'


Listing Tasks
To view tasks, users can use the list command. This displays a list of all tasks or can be customized to show only active or completed tasks.
> list
Tasks:
[x] Buy milk
2. [ ] Go for a run

Deleting a Task
To delete a task, users can use the delete command followed by the task’s ID.
> delete 2
Task deleted: 'Go for a run'
