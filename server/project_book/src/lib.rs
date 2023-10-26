use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool{
    tasks: Vec<Operator>,
    sender: Sender<Message>
}

enum Message{
    NewMission(Mission),
    Shutdown
}

struct Operator{
    id: usize,
    task: Option<JoinHandle<()>>
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl Operator{
    pub fn new(id: usize, receptor:  Arc<Mutex<Receiver<Message>>>) -> Operator {
        let task = thread::spawn(move || loop {
            let message = receptor.lock().unwrap().recv().unwrap();


            match message{
                Message::NewMission(mission) => {
                    println!("OP n° {} get the mission and execute it", id);

                    mission();
                }
                Message::Shutdown =>{
                    println!("OP n° {} is shutting down", id);

                    break;
                }
            }
        });
        Operator{
            id,
            task: Some(task)
        }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let mission = Box::new(f);
        self.sender.send(Message::NewMission(mission)).unwrap();
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let (sender, rx) = mpsc::channel();

        let mut tasks = Vec::with_capacity(size);
        let receptor = Arc::new(Mutex::new(rx));

        for id in 0..size {
            tasks.push(Operator::new(id, receptor.clone()));
        }
        ThreadPool {
            tasks,
            sender
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.tasks {
            self.sender.send(Message::Shutdown).unwrap()
        }

        for op in &mut self.tasks {
            println!("{} OP is stopped", op.id);
            if let Some(task) = op.task.take() {
                task.join().unwrap();
            }
        }
    }
}