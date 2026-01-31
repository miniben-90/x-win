use x_win::{get_active_window, get_browser_url};

fn main() {
  match get_active_window() {
    Ok(active_window) => match get_browser_url(&active_window) {
      Ok(browser_url) => {
        println!("browser url: {:#?}", browser_url);
      }
      Err(_) => {
        println!("error occurred while getting the browser url of active window");
      }
    },
    Err(_) => {
      println!("error occurred while getting the active window");
    }
  }
}
