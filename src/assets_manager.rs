sixtyfps::sixtyfps! {
  MainWindow := Window {
    width: 500px;
    height: 500px;
    Text {
      text: "Assets Manager App";
    }
  }
}
pub fn main() {
  MainWindow::new().run();
}
