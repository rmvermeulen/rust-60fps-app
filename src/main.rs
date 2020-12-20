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
        border-radius: 8px;
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
        signal check_if_pair_solved();
        property <bool> disable_tiles;
        // dimensions
        width: 326px;
        height: 326px;
        // to create the model
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
                if (!root.disable_tiles) {
                    tile.image_visible = !tile.image_visible;
                    root.check_if_pair_solved();
                }
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

    // handlers
    let close_delay = std::time::Duration::from_millis(500);
    let mw_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        let pair = (flipped_tiles.next(), flipped_tiles.next());
        match pair {
            (Some((id1, mut tile1)), Some((id2, mut tile2))) => {
                let is_pair_solved = tile1 == tile2;
                if is_pair_solved {
                    tile1.solved = true;
                    tiles_model.set_row_data(id1, tile1.clone());
                    tile2.solved = true;
                    tiles_model.set_row_data(id2, tile2.clone());
                } else {
                    let main_window = mw_weak.unwrap();
                    main_window.set_disable_tiles(true);
                    let tiles_model = tiles_model.clone();
                    sixtyfps::Timer::single_shot(close_delay, move || {
                        main_window.set_disable_tiles(false);
                        tile1.image_visible = false;
                        tiles_model.set_row_data(id1, tile1);
                        tile2.image_visible = false;
                        tiles_model.set_row_data(id2, tile2);
                    });
                }
            }
            _ => {
                // there's no pair of tiles
            }
        }
    });

    // run app
    main_window.run();
}
