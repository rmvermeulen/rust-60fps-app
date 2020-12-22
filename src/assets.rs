sixtyfps::sixtyfps! {
  MainWindow := Window {
    width: 500px;
    height: 500px;
  }
}

pub fn main() {
  let main_window = MainWindow::new();
  main_window.run();
}
