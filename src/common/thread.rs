#![deny(unused_imports)]

use std::{
  collections::HashMap,
  sync::{mpsc, Arc, Mutex},
  thread,
};

pub struct ThreadManager {
  id: Arc<Mutex<u32>>,
  threads: Arc<Mutex<HashMap<u32, mpsc::Sender<()>>>>,
}

impl ThreadManager {
  pub fn new() -> Self {
    ThreadManager {
      id: Arc::new(Mutex::new(1)),
      threads: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn start_thread<F>(&self, work: F) -> Result<u32, String>
  where
    F: Fn(mpsc::Receiver<()>) + Send + 'static,
  {
    let key = {
      let mut id = self.id.lock().unwrap();
      let key = *id;
      *id += 1;
      key
    };
    let (sender, receiver) = mpsc::channel::<()>();
    let threads_clone = Arc::clone(&self.threads);

    let sender_ = sender.clone();

    let handle = thread::spawn(move || {
        work(receiver);
    });
    threads_clone.lock().unwrap().insert(key, sender_);
    let threads_clone_for_cleanup = Arc::clone(&self.threads);
    thread::spawn(move || {
      let _ = handle.join();
      threads_clone_for_cleanup.lock().unwrap().remove(&key);
    });
    Ok(key)
  }

  pub fn stop_thread(&self, key: u32) -> Result<(), String> {
    let sender_mutex = {
      let threads = self.threads.lock().unwrap();
      threads
        .get(&key)
        .cloned()
        .ok_or_else(|| "Thread not found.".to_string())?
    };
    sender_mutex
      .send(())
      .map_err(|_| "Failed to send stop signal.".to_string())?;
    Ok(())
  }

  pub fn stop_all_threads(&self) -> Result<(), String> {
    let threads = self.threads.lock().unwrap();
    for (_, sender) in threads.iter() {
      sender.send(()).map_err(|_| "Failed to send stop signal.".to_string())?;
    }
    Ok(())
  }
}
