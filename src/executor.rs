use crate::future::OurFuture;
use cortex_m::asm;
use heapless::mpmc::Queue;
use rtt_target::rprintln;

static TASK_ID_READY: Queue<usize, 4> = Queue::new();

pub fn wake_task(task_id: usize) {
    rprintln!("Waking task {}", task_id);
    if TASK_ID_READY.enqueue(task_id).is_err() {
        panic!("Cannot enqueue task_id: {}, task queue full", task_id);
    }
}

pub fn run_tasks(tasks: &mut [&mut dyn OurFuture<Output = ()>]) -> ! {
    for task_id in 0..tasks.len() {
        TASK_ID_READY.enqueue(task_id).ok();
    }
    loop {
        while let Some(task_id) = TASK_ID_READY.dequeue() {
            if task_id >= tasks.len() {
                rprintln!("Invalid task ID {}!", task_id);
                continue;
            }
            rprintln!("Running task {}", task_id);
            tasks[task_id].poll(task_id);
        }
        rprintln!("No tasks ready, going to sleep...");
        asm::wfi();
    }
}
