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
pub extern fn test_box_queue1() -> i8 {
	let _task = Task::new().start(|| task1().expect("task1 running")).unwrap();
	let _task = Task::new().start(|| task2().expect("task2 running")).unwrap();

	start_kernel();

	1
}

fn task1() -> Result<(), FreeRtosError> {
    let mut cnt = 0;
    let outq = &Q1;
    let inq  = &Q2;

    while cnt < 10 {
        CurrentTask::delay(Duration::ms(50));

        let out_msg = Msg {
            cnt: cnt,
            msg: format!("counter: {:?}", cnt),
        };
        outq.send(Box::new(out_msg), Duration::infinite())?;

        let in_msg = inq.receive(Duration::infinite())?;
        cnt = in_msg.cnt + 1;

        debug_print(&format!("task 1 {:?}", in_msg));
    }

    exit_test(0);
    Ok(())
}

fn task2() -> Result<(), FreeRtosError> {
    let outq = &Q2;
    let inq  = &Q1;

    loop {
        let in_msg = inq.receive(Duration::infinite())?;
        let cnt = in_msg.cnt + 1;

        debug_print(&format!("task 2 {:?}", in_msg));
        drop(in_msg); // not necessary, just to force free memory

        CurrentTask::delay(Duration::ms(50));

        let out_msg = Msg {
            cnt: cnt,
            msg: format!("counter: {:?}", cnt),
        };
        outq.send(Box::new(out_msg), Duration::infinite())?;
    }
}
