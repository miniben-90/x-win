use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};
use std::thread;
use std::time::Duration;
use x_win::{empty_entity, get_active_window, WindowInfo};

fn subscribe_active_window<F>(interval: u64, callback: F) -> mpsc::Sender<()>
where
  F: Fn(Result<WindowInfo, String>) + Send + 'static,
{
  let (sender, receiver) = mpsc::channel::<()>();

  thread::spawn(move || {
    let mut current_window = empty_entity();

    loop {
      match receiver.try_recv() {
        Ok(_) | Err(TryRecvError::Disconnected) => {
          break;
        }
        _ => {
          match get_active_window() {
            Ok(new_window) => {
              if new_window.id.ne(&current_window.id)
                || new_window.title.ne(&current_window.title)
                || new_window
                  .info
                  .process_id
                  .ne(&current_window.info.process_id)
              {
                current_window = new_window.clone();
                callback(Ok(new_window));
              }
            }
            Err(e) => {
              eprintln!("Err: {:?}", e);
            }
          }
          thread::sleep(Duration::from_millis(interval));
        }
      }
    }
  });

  sender
}

fn main() {
  println!("Start watching active window...");
  println!("Press 'q' and 'Enter' to exit.\n");

  let running = Arc::new(AtomicBool::new(true));
  let running_clone = Arc::clone(&running);

  let stop_signal = subscribe_active_window(500, |result| match result {
    Ok(window) => println!("New Window: {:?}", window),
    Err(e) => eprintln!("Err: {}", e),
  });

  // Thread de lecture de l'entrée utilisateur
  let input_stop_signal = stop_signal.clone();
  thread::spawn(move || {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      match line {
        Ok(ref input) if input.trim() == "q" => {
          println!("Exiting watch mode.");
          input_stop_signal.send(()).unwrap();
          running_clone.store(false, Ordering::SeqCst);
          break;
        }
        Ok(_) => {
          println!("Press 'q' and 'Enter' to exit.");
        }
        Err(e) => {
          eprintln!("stdin Err: {}", e);
          break;
        }
      }
    }
  });

  while running.load(Ordering::SeqCst) {
    thread::sleep(Duration::from_secs(1));
  }
}
