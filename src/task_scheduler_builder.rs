// use deque::{ BufferPool, Stealer, Worker };
// use rand::Rng;
// use std::vec::Vec;
// use Task;
// use TaskScheduler;

// pub struct TaskSchedulerBuilder {
//     task_schedulers: Vec<TaskScheduler>,
// }

// impl TaskSchedulerBuilder {
//     pub fn new(number_of_task_schedulers: usize, mut rngs: Vec<Box<Rng>>) -> Option<TaskSchedulerBuilder> {
//         let mut buffer_pool = BufferPool::<Task>::new();
//         let mut worker_vector = Vec::<Worker<Task>>::with_capacity(number_of_task_schedulers);
//         let mut stealer_vector = Vec::<Stealer<Task>>::with_capacity(number_of_task_schedulers);

//         for i in 0..number_of_task_schedulers {
//             let (worker, stealer) = buffer_pool.deque();
//             worker_vector.push(worker);
//             stealer_vector.push(stealer);
//         };

//         let mut task_schedulers = Vec::<TaskScheduler>::with_capacity(number_of_task_schedulers);

//         for worker_index in (0..worker_vector.len()).rev() {
//             let worker = match worker_vector.pop() {
//                 Some(result) => { result },
//                 None => return None,
//             };

//             let mut stealers = Vec::<Stealer<Task>>::with_capacity(number_of_task_schedulers);

//             for stealer_index in 0..stealer_vector.len() {
//                 if worker_index == stealer_index {
//                     continue;
//                 }

//                 let stealer = match stealer_vector.get(stealer_index as usize) {
//                     Some(result) => result.clone(),
//                     None => return None,
//                 };

//                 stealers.push(stealer);
//             };

//             let rng = match rngs.pop() {
//                 Some(result) => { result },
//                 None => return None,
//             };

//             let task_scheduler = TaskScheduler::new(worker, stealers, rng);
//             task_schedulers.push(task_scheduler);
//         };

//         Some(TaskSchedulerBuilder{ task_schedulers: task_schedulers })
//     }

//     pub fn get_task_schedulers(self) -> Vec<TaskScheduler> {
//         self.task_schedulers
//     }
// }
