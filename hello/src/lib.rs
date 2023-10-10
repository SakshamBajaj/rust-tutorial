use std::thread;
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};

pub struct Job {
    pub func: Box<dyn FnOnce() + Send + 'static>,
}
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {} got a job; executing.", id);
                        (job.func)();
                    }
                    Err(_) => break,
                }
                
            }
        });
        Worker {id: id, thread: Some(thread)}
    }
   
}
pub struct ThreadPool{
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());

        // wait on all receivers to finish

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let recv_mutex = Arc::new(Mutex::new(rx));
        for i in 0..size{
            workers.push(Worker::new(i, recv_mutex.clone()));
        }
        ThreadPool {sender: Some(tx), workers: workers}
    }
    pub fn execute<F>(&mut self, f: F) 
    where F: FnOnce() + Send + 'static
    {
        
        self.sender.as_ref().unwrap().send(Job{func: Box::new(f)}).unwrap();
            
    }

    
}