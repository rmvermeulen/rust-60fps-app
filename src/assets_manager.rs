extern crate globwalk;

sixtyfps::sixtyfps! {

  LabeledInput := GridLayout {
    property <string> label <=> Label.text;
    initial_focus: input;
    Row {
        Label := Text {}
        input := TextInput {}
    }
}
  MainWindow := Window {
    property <string> title: "Assets Manager App";
    width: 500px;
    height: 500px;
    initial_focus: second_input_field;

    GridLayout {
      maximum_width: 250px;
      maximum_height: 80px;
      padding: 16px;
        Row {
            first_input_field := LabeledInput {
              label: "first";
            }
        }
        Row {
            second_input_field := LabeledInput {
              label: "second";
            }
        }
    }
  }
}

pub fn main() {
  use sixtyfps::SharedString;
  let app = MainWindow::new();
  app.set_title(SharedString::from("Assets Manager App"));
  app.run();
}
