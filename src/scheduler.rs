use deque::{ BufferPool, Stealer, Stolen, Worker };
use rand::Rng;
use SchedulerTrait;
use TaskBox;
use TaskBoxIntoIterator;

pub struct Scheduler<TRng>
    where TRng: Rng +
                Send {
    worker: Worker<TaskBox<Scheduler<TRng>>>,
    stealers: Vec<Stealer<TaskBox<Scheduler<TRng>>>>,
    rng: TRng,
}

impl <TRng> Scheduler<TRng>
    where TRng: Rng +
                Send {
    pub fn new<TStealerIterator>(worker: Worker<TaskBox<Scheduler<TRng>>>,
                                 stealers: TStealerIterator,
                                 rng: TRng) -> Scheduler<TRng>
        where TStealerIterator: IntoIterator<Item = Stealer<TaskBox<Scheduler<TRng>>>> {
        Scheduler { worker: worker,
                    stealers: stealers.into_iter().collect(),
                    rng: rng }
    }

    pub fn new_batch<TRangeIntoIterator>(rngs: TRangeIntoIterator) -> Vec<Scheduler<TRng>>
        where TRangeIntoIterator: IntoIterator<Item = TRng> {
        let buffer_pool = BufferPool::<TaskBox<Scheduler<TRng>>>::new();

        let (rng_and_worker_vec, stealers): (Vec<(TRng, Worker<TaskBox<Scheduler<TRng>>>)>,
                                             Vec<Stealer<TaskBox<Scheduler<TRng>>>>) = {
            rngs.into_iter()
                .map(
                    |rng| {
                        let (worker, stealer) = buffer_pool.deque();
                        ((rng, worker), stealer)
                    }
                ).unzip()
        };
        let schedulers = rng_and_worker_vec.into_iter()
                                           .map(|(rng, worker)| Scheduler::new(worker, stealers.clone(), rng))
                                           .collect();
        schedulers
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
        let num_stealers = self.stealers.len();
        let start_index = self.rng.gen_range(0, num_stealers);

        self.stealers.iter()
                     .cycle()
                     .skip(start_index)
                     .take(num_stealers)
                     .any(|stealer| self.try_run_stealer(stealer))
    }

    fn try_run_stealer(&self, stealer: &Stealer<TaskBox<Scheduler<TRng>>>) -> bool {
        while {
            match stealer.steal() {
                Stolen::Empty => {
                    // No tasks to steal. Stop trying for this stealer.
                    false
                },
                Stolen::Abort => {
                    // Thread contention. Keep trying until the other thread yields.
                    true
                },
                Stolen::Data(data) => {
                    // Successful steal. Run task and stop trying.
                    data.call_box((&self,));
                    return true;
                },
            }
        } { }
        false
    }
}

impl <TRng> SchedulerTrait for Scheduler<TRng>
    where TRng: 'static +
                Rng +
                Send {
    type TTaskBoxParam = Scheduler<TRng>;
    type TScheduleReturn = ();
    type TScheduleMultipleReturn = ();

    fn schedule(&self,
                task_box: TaskBox<Self::TTaskBoxParam>) -> Self::TScheduleReturn {
        self.worker.push(task_box)
    }

    fn schedule_multiple<TTaskBoxIntoIterator>(&self,
                                               task_boxes: TTaskBoxIntoIterator) -> Self::TScheduleMultipleReturn
        where TTaskBoxIntoIterator: TaskBoxIntoIterator<Self::TTaskBoxParam> {
        for task_box in task_boxes {
            self.worker.push(task_box)
        }
    }
}
