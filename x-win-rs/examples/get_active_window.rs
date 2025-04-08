use x_win::get_active_window;

fn main() {
  match get_active_window() {
    Ok(active_window) => {
      println!("active window: {:#?}", active_window);
    }
    Err(_) => {
      println!("error occurred while getting the active window");
    }
  }
}
