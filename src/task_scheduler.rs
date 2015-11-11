use DecayPtr;
use deque::{ BufferPool, Stealer, Stolen, Worker };
use rand::Rng;
use std::sync::Arc;
use Task;
use TaskBox;
use TaskBoxIntoIterator;

pub struct TaskScheduler {
    worker: Worker<TaskBox>,
    stealers: Vec<Stealer<TaskBox>>,
    rng: Box<Rng + Send>,
}

fn cyclic_inc(value: usize, max: usize) -> usize {
    (value + 1) % max
}

impl TaskScheduler {
    pub fn new(worker: Worker<TaskBox>,
               stealers: Vec<Stealer<TaskBox>>,
               rng: Box<Rng + Send>) -> TaskScheduler {
        TaskScheduler { worker: worker,
                        stealers: stealers,
                        rng: rng, }
    }

    pub fn new_batch(number_of_task_schedulers: usize,
                     rngs: Vec<Box<Rng + Send>>) -> Option<Vec<TaskScheduler>> {
        let mut worker_vector = Vec::<Worker<TaskBox>>::with_capacity(number_of_task_schedulers);
        let mut stealer_vector = Vec::<Stealer<TaskBox>>::with_capacity(number_of_task_schedulers);

        let buffer_pool = BufferPool::<TaskBox>::new();
        for i in 0..number_of_task_schedulers {
            let (worker, stealer) = buffer_pool.deque();
            worker_vector.push(worker);
            stealer_vector.push(stealer);
        };

        let mut task_schedulers = Vec::<TaskScheduler>::with_capacity(number_of_task_schedulers);
        let mut rngs_mut = rngs;

        for worker_index in (0..worker_vector.len()).rev() {
            let worker = match worker_vector.pop() {
                Some(result) => { result },
                None => return None,
            };

            let mut stealers = Vec::<Stealer<TaskBox>>::with_capacity(number_of_task_schedulers);

            for stealer_index in 0..stealer_vector.len() {
                if worker_index == stealer_index {
                    continue;
                }

                let stealer = match stealer_vector.get(stealer_index as usize) {
                    Some(result) => result.clone(),
                    None => return None,
                };

                stealers.push(stealer);
            };

            let rng = match rngs_mut.pop() {
                Some(result) => { result },
                None => return None,
            };

            let task_scheduler = TaskScheduler::new(worker, stealers, rng);
            task_schedulers.push(task_scheduler);
        };

        Some(task_schedulers)
    }

    pub fn run(&mut self) {
        while {
            while self.try_run_worker() { }
            self.try_run_stealers()
        } { }
    }

    fn try_run_worker(&mut self) -> bool {
        match self.worker.pop() {
            Some(task_box) => {
                task_box.call_box((&self,));
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

    fn try_run_stealer(&self, stealer: &Stealer<TaskBox>) -> bool {
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

    pub fn add_task_box(&self, task_box: TaskBox) {
        self.worker.push(task_box);
    }

    pub fn add_task<TTask: 'static + Task>(&self, task: TTask) {
        let task_box = Box::new(task);
        self.add_task_box(task_box);
    }

    pub fn add_task_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self,
                                                                               task_boxes: TTaskBoxIntoIterator) {
        for task_box in task_boxes.into_iter() {
            self.add_task_box(task_box);
        }
    }

    pub fn add_task_box_with_continuation_box(&self,
                                              task_box: TaskBox,
                                              continuation_box: TaskBox) {
        let complete_task = move |task_scheduler: &TaskScheduler| {
            task_box.call_box((&task_scheduler,));
            continuation_box.call_box((&task_scheduler,));
        };
        self.add_task(complete_task);
    }

    pub fn add_task_with_continuation<TTask: 'static + Task, 
                                      TContinuation: 'static  + Task>(&self,
                                                                      task: TTask,
                                                                      continuation: TContinuation) {
        let task_box = Box::new(task);
        let continuation_box = Box::new(continuation);
        self.add_task_box_with_continuation_box(task_box, continuation_box);
    }

    pub fn add_task_box_with_continuation<TContinuation: 'static  + Task>(&self,
                                                                          task_box: TaskBox,
                                                                          continuation: TContinuation) {
        let continuation_box = Box::new(continuation);
        self.add_task_box_with_continuation_box(task_box, continuation_box);
    }

    pub fn add_task_with_continuation_box<TTask: 'static + Task>(&self,
                                                                 task: TTask,
                                                                 continuation_box: TaskBox) {
        let task_box = Box::new(task);
        self.add_task_box_with_continuation_box(task_box, continuation_box);
    }

    pub fn add_task_boxes_with_continuation_box<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self,
                                                                                                      task_boxes: TTaskBoxIntoIterator,
                                                                                                      continuation_box: TaskBox) {
        let decaying_continuation_box = unsafe { DecayPtr::new(continuation_box) };

        for task_box in task_boxes {
            let sub_decaying_continutation_box = unsafe { decaying_continuation_box.clone() };
            let sub_complete_task = move |task_scheduler: &TaskScheduler| {
                task_box.call_box((&task_scheduler,));
                match unsafe { sub_decaying_continutation_box.decay() } {
                    Some(continuation_box) => continuation_box.call_box((&task_scheduler,)),
                    None => { },
                };
            };
            self.add_task(sub_complete_task);
        };
    }

    pub fn add_task_boxes_with_continuation<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator,
                                            TContinuation: 'static  + Task>(&self,
                                                                            task_boxes: TTaskBoxIntoIterator,
                                                                            continuation: TContinuation) {
        let continuation_box = Box::new(continuation);
        self.add_task_boxes_with_continuation_box(task_boxes, continuation_box);
    }

    pub fn add_task_box_with_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self,
                                                                                                     task_box: TaskBox,
                                                                                                     continuation_boxes: TTaskBoxIntoIterator) {
        let complete_task = move |task_scheduler: &TaskScheduler| {
            task_box.call_box((&task_scheduler,));
            task_scheduler.add_task_boxes(continuation_boxes);
        };
        self.add_task(complete_task);
    }

    pub fn add_task_with_continuation_boxes<TTask: 'static + Task,
                                            TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self,
                                                                                                 task: TTask,
                                                                                                 continuation_boxes: TTaskBoxIntoIterator) {
        let task_box = Box::new(task);
        self.add_task_box_with_continuation_boxes(task_box, continuation_boxes);
    }

    pub fn add_task_boxes_with_continuation_boxes<TTaskBoxIntoIterator: 'static + TaskBoxIntoIterator>(&self,
                                                                                                       task_boxes: TTaskBoxIntoIterator,
                                                                                                       continuation_boxes: TTaskBoxIntoIterator) {
        let decaying_continuation_boxes = unsafe { DecayPtr::new(continuation_boxes) };

        for task_box in task_boxes {
            let sub_decaying_continutation_boxes = unsafe { decaying_continuation_boxes.clone() };
            let sub_complete_task = move |task_scheduler: &TaskScheduler| {
                task_box.call_box((&task_scheduler,));
                match unsafe { sub_decaying_continutation_boxes.decay() } {
                    Some(continuation_boxes) => task_scheduler.add_task_boxes(continuation_boxes),
                    None => { },
                };
            };
            self.add_task(sub_complete_task);
        };
    }
}
