use prelude::v1::*;
use base::*;
use shim::*;
use units::*;
use isr::*;

unsafe impl<T: Sized> Send for BoxQueue<T> {}
unsafe impl<T: Sized> Sync for BoxQueue<T> {}

/// A queue with a finite size. The items are owned by the queue and are
/// copied.
#[derive(Debug)]
pub struct BoxQueue<T: Sized> {
    queue: FreeRtosQueueHandle,
    item_type: PhantomData<T>,
}

impl<T: Sized> BoxQueue<T> {
    pub fn new(max_size: usize) -> Result<BoxQueue<T>, FreeRtosError> {
        let item_size = mem::size_of::<*mut T>();
        let handle = unsafe { freertos_rs_queue_create(max_size as u32, item_size as u32) };

        if handle == 0 as *const _ {
            return Err(FreeRtosError::OutOfMemory);
        }

        Ok(BoxQueue {
            queue: handle,
            item_type: PhantomData,
        })
    }

    /// Send an item to the end of the queue. Wait for the queue to have empty space for it.
    pub fn send<D: DurationTicks>(&self, item: Box<T>, max_wait: D) -> Result<(), FreeRtosError> {
        let mut ptr = Box::into_raw(item) as FreeRtosVoidPtr;
        let ptr_to_ptr =
            unsafe { ::core::intrinsics::transmute::<*mut _, FreeRtosMutVoidPtr>(&mut ptr) };

        let ret = unsafe { freertos_rs_queue_send(self.queue, ptr_to_ptr, max_wait.to_ticks()) };

        if ret != 0 {
            return Err(FreeRtosError::QueueSendTimeout);
        }

        Ok(())
    }

    /// Wait for an item to be available on the queue.
    pub fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<Box<T>, FreeRtosError> {
        let mut ptr = ptr::null_mut();
        let ptr_to_ptr =
            unsafe { ::core::intrinsics::transmute::<*mut _, FreeRtosMutVoidPtr>(&mut ptr) };

        let ret = unsafe { freertos_rs_queue_receive(self.queue, ptr_to_ptr, max_wait.to_ticks()) };

        if ret != 0 {
            return Err(FreeRtosError::QueueReceiveTimeout);
        }

        if ptr.is_null() {
            return Err(FreeRtosError::InvalidPointer);
        }

        let boxed = unsafe { Box::from_raw(ptr as *mut _) };
        Ok(boxed)
    }
}

impl<T: Sized> Drop for BoxQueue<T> {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_queue_delete(self.queue);
        }
    }
}
