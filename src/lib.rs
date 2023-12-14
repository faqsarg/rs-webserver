use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sx: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv().unwrap();
            println!("worker {id}, got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}

#[derive(Debug, Clone)]
pub struct ZeroSizedPoolErr;

impl fmt::Display for ZeroSizedPoolErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size must be higher than 0")
    }
}

impl ThreadPool {
    pub fn build(n: usize) -> Result<ThreadPool, ZeroSizedPoolErr> {
        if n == 0 {
            return Err(ZeroSizedPoolErr);
        }

        let (sx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(n);

        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Ok(ThreadPool { workers, sx })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sx.send(job).unwrap();
    }
}
