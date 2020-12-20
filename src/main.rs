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
            source: parent.icon;
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
    use sixtyfps::Model;

    let main_window = MainWindow::new();

    // get tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // duplicate tiles to get pairs of each icon
    tiles.extend(tiles.clone());

    // random mix
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // assign shuffled vec to model
    let tiles_model = std::rc::Rc::new(sixtyfps::VecModel::from(tiles));
    main_window.set_memory_tiles(sixtyfps::ModelHandle::new(tiles_model.clone()));

    // run app
    main_window.run();
}
