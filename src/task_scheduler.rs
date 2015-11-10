use DecayPtr;
use deque::{ Stealer, Stolen, Worker };
use rand::Rng;
use std::sync::Arc;
use Task;
use TaskIntoIterator;

use std::boxed::FnBox;

pub struct TaskScheduler {
    worker: Worker<Task>,
    stealers: Vec<Stealer<Task>>,
    rng: Box<Rng>,
}

fn cyclic_inc(value: usize, max: usize) -> usize {
    (value + 1) % max
}

impl TaskScheduler {
    pub fn new(worker: Worker<Task>,
               stealers: Vec<Stealer<Task>>,
               rng: Box<Rng>) -> TaskScheduler {
        TaskScheduler { worker: worker,
                        stealers: stealers,
                        rng: rng, }
    }

    pub fn run(&mut self) {
        while {
            while self.try_run_worker() { }
            self.try_run_stealers()
        } { }
    }

    fn try_run_worker(&mut self) -> bool {
        match self.worker.pop() {
            Some(task) => {
                task.call_box((&self,));
                true
            },
            None => {
                false
            },
        }
    }

    fn try_run_stealers(&mut self) -> bool {
            let mut has_error = false;
            let mut has_stolen = false;

            let num_stealers = self.stealers.len();
            let cycle_index = self.rng.gen_range(0, num_stealers);
            let mut current_index = cycle_index;

            while !has_error &&
                  !has_stolen &&
                  current_index != cycle_index {
                match self.stealers.get(cycle_index) {
                    Some(stealer) => {
                        has_stolen = self.try_run_stealer(stealer);
                        current_index = cyclic_inc(current_index, num_stealers);
                    },
                    None => has_error = true,
                };
            }

            has_stolen
    }

    fn try_run_stealer(&self, stealer: &Stealer<Task>) -> bool {
        let mut has_any = true;
        let mut has_stolen = false;

        while has_any &&
              !has_stolen {
            match stealer.steal() {
                Stolen::Empty => has_any = false,
                Stolen::Abort => { },
                Stolen::Data(data) => {
                    has_stolen = true;
                    data.call_box((&self,));
                },
            }
        }

        has_stolen
    }

    pub fn add_task_0<TFn: 'static + FnBox(&TaskScheduler) + Sync + Send>(&self, task: TFn) {
        self.worker.push(Box::new(task));
    }

    pub fn add_task(&self, task: Task) {
        self.worker.push(task);
    }

    pub fn add_tasks<TTaskIntoIterator: 'static + TaskIntoIterator>(&self,
                                                                    tasks: TTaskIntoIterator) {
        for task in tasks.into_iter() {
            self.add_task(task);
        }
    }

    pub fn add_task_with_continuation(&self,
                                      task: Task,
                                      continuation: Task) {
        let complete_task = Box::new(move |task_scheduler: &TaskScheduler| {
            task.call_box((&task_scheduler,));
            continuation.call_box((&task_scheduler,));
        });
        self.add_task(complete_task);
    }

    pub fn add_tasks_with_continuation<TTaskIntoIterator: 'static + TaskIntoIterator>(&self,
                                                                                      tasks: TTaskIntoIterator,
                                                                                      continuation: Task) {
        let decaying_continuation = unsafe { DecayPtr::new(continuation) };

        for task in tasks {
            let sub_decaying_continutation = unsafe { decaying_continuation.clone() };
            let sub_complete_task = Box::new(move |task_scheduler: &TaskScheduler| {
                task.call_box((&task_scheduler,));
                match unsafe { sub_decaying_continutation.decay() } {
                    Some(continuation) => continuation.call_box((&task_scheduler,)),
                    None => { },
                };
            });
            self.add_task(sub_complete_task);
        };
    }

    pub fn add_task_with_continuations<TTaskIntoIterator: 'static + TaskIntoIterator>(&self,
                                                                                      task: Task,
                                                                                      continuations: TTaskIntoIterator) {
        let complete_task = Box::new(move |task_scheduler: &TaskScheduler| {
            task.call_box((&task_scheduler,));
            task_scheduler.add_tasks(continuations);
        });
        self.add_task(complete_task);
    }

    pub fn add_tasks_with_continuations<TTaskIntoIterator: 'static + TaskIntoIterator>(&self,
                                                                                       tasks: TTaskIntoIterator,
                                                                                       continuations: TTaskIntoIterator) {
        let decaying_continuations = unsafe { DecayPtr::new(continuations) };

        for task in tasks {
            let sub_decaying_continutations = unsafe { decaying_continuations.clone() };
            let sub_complete_task = Box::new(move |task_scheduler: &TaskScheduler| {
                task.call_box((&task_scheduler,));
                match unsafe { sub_decaying_continutations.decay() } {
                    Some(continuations) => task_scheduler.add_tasks(continuations),
                    None => { },
                };
            });
            self.add_task(sub_complete_task);
        };
    }
}

unsafe impl Send for TaskScheduler { }
