use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread::{self, JoinHandle}
};


type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    workers: Vec<Worker>,
    senders: Option<mpsc::Sender<Job>>
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
        ThreadPool { workers, senders:Some(senders) }
    }

    pub fn execute<F>(&self, f: F)
        where 
        F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.senders.as_ref().unwrap().send(job).unwrap();
        }
}

struct Worker{
    id: usize, 
    thread : JoinHandle<()>
}

impl Worker {
    
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv();
            match message{
                Ok(job) =>{
                    println!("worker {id} got a job; executing;");
                    job();
                },
                Err(_) =>{
                    println!("worker {id} disconnected, shutting down;");
                    break;
                }
            }
            
        });
        Worker{id, thread}
    }
}

// implmenting drop trait for the ThreadPool
impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.senders.take());
        for worker in self.workers.drain(..){
            println!("Shutting down the worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}