# TP2 Simple Round Robin Scheduler and GPIO Interrupt

In this lab, we will implement a simple scheduler that manages tasks using the round robin scheduling algorithm. We will also implement an interrupt handler for GPIO to change the state of an LED on each button press.

## 1. Round Robin Scheduler

### 1.1 Implementation

To implement the scheduler, we have created a `Task` struct that contains information about each task. This struct contains the following fields:

```rust
pub struct Task {
    pub id: u32,
    callback: fn(),
}
```

You must implement a constructor for this struct and a `run` method that executes the task.

Next, we have created a `Sched` struct that contains information about the scheduler. This struct contains the following fields:

```rust
pub struct Sched {
    tasks: Option<[Task; MAX_TASKS]>,
    current_task: usize,
    last_task: usize,
}
```

You must implement a constructor for this struct, an `add_task` method that adds a task to the scheduler, and a `run` method that executes the current task.

This scheduler uses an array of tasks to store the tasks. Why we not make a circular linked list of tasks?

### 1.2 Testing the Scheduler

To test the scheduler, you must create three tasks that display different messages and mark a time delay each time.

* The first task displays the message "Task 1" and marks a time delay of 1 second.
* The second task displays the message "Task 2" and marks a time delay of 2 seconds.
* The third task displays the message "Task 3" and marks a time delay of 3 seconds.

You must add these tasks to the scheduler and run the scheduler. You must also add a message to indicate how long each task took to execute.

## 2. GPIO Interrupt

In this part, you must generate and handle an interrupt. You will use the button and LED on the ESP.

### 2.1 Implementation

First, you need to declare tow `static` variables to store the LED and the button. We will use the `RefCell` type for these variables.


+ What is the `RefCell` type?
+ Why do we need to use the `RefCell` type?
+ Is using `RefCell` enough? Do we need to embed it in another type? If so, why?

You must configure the LED and button. Then, you must configure the interrupt for the button. When the button is pressed, the interrupt must change the state of the LED.

To do this, you must implement a function that changes the state of the LED and apply the `handler` macro to it.

+ What are the constraints for accessing the LED and BUTTON static variables?

### 2.2 Testing the Interrupt

To test the interrupt, you must press the button and verify that the state of the LED changes.

### 2.3 Impact on the Scheduler

Add a busy wait to the interrupt function. 

+ What happens? Why?
