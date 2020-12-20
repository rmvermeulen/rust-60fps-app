sixtyfps::sixtyfps! {
    MemoryTile := Rectangle {
        width: 64px;
        height: 64px;
        color: #3960D5;

        Image {
            source: img!"icons/bus.png";
            width: parent.width;
            height: parent.height;
        }
    }
    MainWindow := Window {
        MemoryTile {}
    }
}

fn main() {
    MainWindow::new().run();
}
