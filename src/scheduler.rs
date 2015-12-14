use deque::{ BufferPool, Stealer, Stolen, Worker };
use rand::Rng;
use SchedulerTrait;
use SimpleBeginScheduleTrait;
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
    pub fn new(worker: Worker<TaskBox<Scheduler<TRng>>>,
               stealers: Vec<Stealer<TaskBox<Scheduler<TRng>>>>,
               rng : TRng) -> Scheduler<TRng> {
        Scheduler { worker: worker,
                    stealers: stealers,
                    rng: rng }
    }

    pub fn new_batch<TRangeIntoIterator>(rngs: TRangeIntoIterator) -> Vec<Scheduler<TRng>>
        where TRangeIntoIterator: IntoIterator<Item = TRng> {
        let buffer_pool = BufferPool::<TaskBox<Scheduler<TRng>>>::new();

        let (rng_and_worker_vec, stealers): (Vec<(TRng,
                                                  Worker<TaskBox<Scheduler<TRng>>>)>,
                                             Vec<Stealer<TaskBox<Scheduler<TRng>>>>) = {
            rngs.into_iter()
                .map(|rng| {
                let (worker, stealer) = buffer_pool.deque();
                ((rng, worker), stealer)
            }).unzip()
        };
        let schedulers = rng_and_worker_vec.into_iter()
                                           .map(|(rng, worker)| Scheduler::new(worker, stealers.clone(), rng))
                                           .collect();
        schedulers
    }
}

impl<'a,
     TRng> SimpleBeginScheduleTrait<'a,
                                    Scheduler<TRng>> for Scheduler<TRng>
    where TRng: 'static +
                Rng +
                Send { }

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
