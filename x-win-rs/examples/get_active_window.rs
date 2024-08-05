use x_win::{get_active_window, XWinError};

fn main() {
  match get_active_window() {
    Ok(active_window) => {
      println!("active window: {:#?}", active_window);
    }
    Err(XWinError) => {
      println!("error occurred while getting the active window");
    }
  }
}
