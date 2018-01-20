use super::*;
use prelude::v1::*;

use freertos_rs::*;

#[derive(Debug)]
struct Msg {
    pub cnt: usize,
    pub msg: String,
}

type MsgQueue = BoxQueue<Msg>;

lazy_static! {
    static ref Q1: MsgQueue = BoxQueue::new(1).expect("Queue creation");
    static ref Q2: MsgQueue = BoxQueue::new(1).expect("Queue creation");
}

#[no_mangle]
pub extern fn test_box_queue2() -> i8 {
	let _task = Task::new().start(|| task1().expect("task1 running")).unwrap();
	let _task = Task::new().start(|| task2().expect("task2 running")).unwrap();

	start_kernel();

	1
}

fn task1() -> Result<(), FreeRtosError> {
    let outq = &Q1;
    let inq  = &Q2;

    let mut msg = Box::new(Msg {
        cnt: 0,
        msg: format!("counter: {:?}", 0),
    });

    while msg.cnt < 10 {
        CurrentTask::delay(Duration::ms(50));

        outq.send(msg, Duration::infinite())?;

        msg = inq.receive(Duration::infinite())?;
        debug_print(&format!("task 1 {:?}", msg));

        msg.cnt += 1;
        msg.msg = format!("counter: {:?}", msg.cnt);
    }

    exit_test(0);
    Ok(())
}

fn task2() -> Result<(), FreeRtosError> {
    let outq = &Q2;
    let inq  = &Q1;

    loop {
        let mut msg = inq.receive(Duration::infinite())?;
        debug_print(&format!("task 2 {:?}", msg));

        msg.cnt += 1;
        msg.msg = format!("counter: {:?}", msg.cnt);

        CurrentTask::delay(Duration::ms(50));

        outq.send(msg, Duration::infinite())?;
    }
}

