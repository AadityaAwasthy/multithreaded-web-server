use std::{thread::{JoinHandle, self}, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let current_receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, current_receiver));
        }

        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job!");
                job();
            }
        });

        Worker{id, thread}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
