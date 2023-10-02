use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// #panics
    /// 
    /// The `new` function will panic if the size is 0

    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            // generate a number of IDs for the Workers
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    } 
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    /// A Worker can be placed into a ThreadPool
    /// It is idle until it receives a request for a connection from a client 
    /// Always initiates with a worker `id` and `Joinhandle`

    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            
            match message{
                Ok(job) => {
                    println!("Worker {id} got a job! Executing..");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected. Shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) } // returns a worker
    }
}
