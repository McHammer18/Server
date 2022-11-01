// Creation of ThreadPool in library crate
use std::{thread, sync::{mpsc, Mutex, Arc}};


pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

//Helps sort and implement the execute method to accept differnt type of jobs/requests
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

	//set size of thread pool
	//Panic if the size is set less than 0
	pub fn new(size: usize) -> ThreadPool{
		assert!(size>0);

		let (sender, receiver) = mpsc::channel();

		let reciever = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size{
			//create threads
			workers.push(Worker::new(id, Arc::clone(&reciever)));
		}
		ThreadPool {workers, sender}
	}

	// Simulate thread::spaen
	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static
		{
			let job = Box::new(f);
			self.sender.send(job).unwrap();
		}
}


//block that associates a thread as a worker and assigns to a job once a request is recieved
struct Worker{
	id: usize,
	thread: thread::JoinHandle<()>
}

impl Worker{
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		//set in a loop so that the threads are always looking for a new job as the server is running
		let thread = thread::spawn(move|| loop{
			let job = receiver.lock().unwrap().recv().unwrap();

			println!("Worker {} got a job; executing.", id);

			job();
		});

		Worker {id, thread}
	}
}
