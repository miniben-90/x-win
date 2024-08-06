use x_win::{get_open_windows, XWinError};

fn main() {
  match get_open_windows() {
    Ok(open_windows) => {
      println!("open windows: {:#?}", open_windows);
    }
    Err(XWinError) => {
      println!("error occurred while getting open windows");
    }
  }
}
