use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread::{self, JoinHandle}
};


type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    workers: Vec<Worker>,
    senders: mpsc::Sender<Job>
}

impl ThreadPool{
    ///Creates a new instance of ThreadPool
    /// 
    /// The size is the number of threads in the pool 
    /// 
    /// #Panics 
    /// 
    /// The 'new' function panics if size less than or equal to zero
    pub fn new(size: usize)-> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (senders, receiver)= mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, senders }
    }

    pub fn execute<F>(&self, f: F)
        where 
        F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.senders.send(job).unwrap();
        }
}

struct Worker{
    id: usize, 
    thread : JoinHandle<()>
}

impl Worker {
    
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("worker {id} got a job; executing;");
            job();
        });
        Worker{id, thread}
    }
}
