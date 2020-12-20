sixtyfps::sixtyfps! {
    // model struct for tiles
    struct TileData := {
        image: resource,
        image_visible: bool,
        solved: bool,
    }
    // a tile for the game
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
        width: 326px;
        height: 326px;
        property <[TileData]> memory_tiles: [
            { image: img!"icons/at.png" },
            { image: img!"icons/balance-scale.png" },
            { image: img!"icons/bicycle.png" },
            { image: img!"icons/bus.png" },
            { image: img!"icons/cloud.png" },
            { image: img!"icons/cogs.png" },
            { image: img!"icons/motorcycle.png" },
            { image: img!"icons/video.png" },
        ];
        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                tile.image_visible = !tile.image_visible;
            }
        }
    }
}

fn main() {
    MainWindow::new().run();
}
