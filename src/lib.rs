use std::{thread, sync::{mpsc::{self, Receiver}, Mutex, Arc}};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
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
        assert!(size>0);
        
        let (sender,reciever) = mpsc::channel();        
        let mut workers: Vec<Worker> = Vec::new();
        let receiver = Arc::new(Mutex::new(reciever));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool{workers, sender}
    }
    
    pub fn execute<F>(&self, f: F)  
    where
    F: FnOnce()+Send+'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
    id: usize
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker { thread: thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        }), id}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;