mod todo_data_worker;
mod todo_visualizer;

use crate::todo_data_worker::TodoData;
use crate::todo_visualizer::TodoVisual;

use fltk::{app, prelude::*, window::Window, input::Input, group::{Flex, Scroll}};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

#[derive(Copy, Clone)]
pub enum TaskMessage {
    Done(&'static str),
    Reset(&'static str),
    Remove(&'static str, &'static str),
    Add(),
}

pub struct ToDoApp {
    app: app::App,
    window: Window,
    receiver: app::Receiver<TaskMessage>,
    sender: app::Sender<TaskMessage>,
    pub todo_list: Vec<String>,
}

impl ToDoApp {
    
    pub fn new () -> Self {
        let todo_list = ToDoApp::read_from_db().unwrap();
        
        let app = app::App::default();

        let widget_themes = WidgetTheme::new(ThemeType::Aero);
        widget_themes.apply();

        let (sender, receiver) = app::channel();
        let window = Window::default().with_size(800, 600).with_label("ToDo List");

        Self {
            app,
            window,
            sender,
            receiver,
            todo_list
        }
    }

    pub unsafe fn run(&mut self) {
        self.view();

        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                match msg {
                    TaskMessage::Done(task_name) => {
                        self.change_task_state(task_name, "Done");
                    },
                    TaskMessage::Reset(task_name) => {
                        self.change_task_state(task_name, "Not Done");
                    },
                    TaskMessage::Remove(task_name, task_state) => {
                        self.remove_task(task_name, task_state);
                    },
                    TaskMessage::Add() => {
                        let scrool = Scroll::from_widget(self.window.child(0).unwrap());
                        let flex_add = Flex::from_widget(scrool.child(0).unwrap());
                        let add_input = Input::from_widget(flex_add.child(0).unwrap());
   
                        self.add_task(&add_input.value(), "Not Done");
                    }
                };
                self.view();
            }
        }
    }
}




#[test]
fn test_add_task() {
    let mut todo_app = ToDoApp::new();
    todo_app.add_task("Task 1", "Not Done");
    assert_eq!(todo_app.todo_list.len(), 1);
    assert_eq!(todo_app.todo_list[0], "Task 1 -> Not Done");
}

#[test]
fn test_change_task_state() {
    let mut todo_app = ToDoApp::new();
    todo_app.add_task("Task 1", "Not Done");
    todo_app.change_task_state("Task 1", "Done");
    assert_eq!(todo_app.todo_list[0], "Task 1 -> Done");
}

#[test]
fn test_remove_task() {
    let mut todo_app = ToDoApp::new();
    todo_app.add_task("Task 1", "Not Done");
    todo_app.remove_task("Task 1", "Not Done");
    assert_eq!(todo_app.todo_list.len(), 0);
}




