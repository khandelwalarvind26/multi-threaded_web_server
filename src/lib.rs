use std::{thread, sync::{mpsc::{self, Receiver}, Mutex, Arc}};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
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
        
        ThreadPool{workers, sender : Some(sender)}
    }
    
    pub fn execute<F>(&self, f: F)  
    where
    F: FnOnce()+Send+'static
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

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {

        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
            
        });

        Worker { thread: Some(thread), id}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;