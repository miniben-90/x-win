use x_win::{get_active_window, get_window_icon};

fn main() {
  match get_active_window() {
    Ok(active_window) => match get_window_icon(&active_window) {
      Ok(icon_info) => {
        println!("icon info: {:#?}", icon_info);
      }
      Err(_) => {
        println!("error occurred while getting the icon info of active window");
      }
    },
    Err(_) => {
      println!("error occurred while getting the active window");
    }
  }
}
