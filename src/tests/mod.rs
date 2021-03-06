use fluent::BeginScheduleTrait;
use fluent::EmptyTaskAdderTrait;
use fluent::EndScheduleTrait;
use LooseContinuation;
use rand::StdRng;
use Scheduler;
use SchedulerTrait;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

mod fluent_tests;

#[test]
fn it_works() {
    let num_threads = 8;
    let num_tasks_per_thread = 500000;

    let schedulers = Scheduler::<StdRng>::new_batch(num_threads);

    let shared = Arc::new(AtomicUsize::new(0));
    for scheduler in &schedulers {
        for _ in 0..num_tasks_per_thread {
            let clone = shared.clone();

            let loose_continuation =
                LooseContinuation::<Scheduler<_>>::new()
                    .begin_schedule()
                    .add_task(
                        move |_: &Scheduler<StdRng>| {
                            clone.fetch_add(1, Ordering::Relaxed);
                            let mut _value = 0;
                            for i in 0..1000 {
                                _value = i + 5;
                            }
                        }
                    )
                    .end_schedule();

            scheduler.schedule(loose_continuation);
        }
        break;
    }

    let join_handles: Vec<_> =
        schedulers
            .into_iter()
            .map(
                |mut scheduler| {
                    thread::spawn(
                        move || {
                            scheduler.run();
                        }
                    )
                }
            ).collect();

    for join_handle in join_handles {
        join_handle.join().unwrap();
    }

    let shared_as_usize: usize = shared.load(Ordering::Relaxed);
    println!("{0}", shared_as_usize);
}
