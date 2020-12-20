sixtyfps::sixtyfps! {
    MemoryTile := Rectangle {
        signal clicked;
        property <bool> open_curtain;
        property <bool> solved;
        property <resource> icon;

        width: 64px;
        height: 64px;
        color: solved? #34CE57 : #3960D5;
        animate color { duration: 800ms; }

        Image {
            source: img!"icons/bus.png";
            width: parent.width;
            height: parent.height;
        }
        left_curtain := Rectangle {
            color: #193076;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
        }
        right_curtain := Rectangle {
            color: #193076;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }

        TouchArea {
            clicked => { root.clicked() }
        }
    }
    MainWindow := Window {
        MemoryTile {
            icon: img!"icons/bus.png";
            clicked => { self.open_curtain = !self.open_curtain; }
        }
    }
}

fn main() {
    MainWindow::new().run();
}
