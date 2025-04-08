use x_win::get_open_windows;

fn main() {
  match get_open_windows() {
    Ok(open_windows) => {
      println!("open windows: {:#?}", open_windows);
    }
    Err(_) => {
      println!("error occurred while getting open windows");
    }
  }
}
