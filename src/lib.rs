use std::{sync::{Arc, Mutex, mpsc},
 thread,
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {  
            workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
}

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn build(size: usize) -> Result<ThreadPool, std::io::Error> {
        if size == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The thread pool size must be greater than zero",
            ));
        }

        Ok(ThreadPool::new(size))
    }
}


struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // using "while let" instead of "loop" can stop other Worker instances receiving jobs. i.e again single thread
           loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
           }
        });
        Worker { id, thread }
    }
}