use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(Resource, Default)]
struct BoardState {
    count: usize,
    finished: bool,
}

#[derive(Component)]
struct CellState {
    x: usize,
    y: usize,
    state: usize,
}

fn main() {
    App::new()
        .init_resource::<BoardState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(350.0, 400.0),
                        title: "Tic Tac Toe".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        ) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, check_board)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2d);

    for x in 0..3 {
        for y in 0..3 {
            commands
                .spawn((
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 0,
                        },
                    ),
                    Transform::from_xyz(x as f32 * 110.0 - 110.0, y as f32 * 110.0 - 135.0, 0.0),
                    Pickable::default(),
                    CellState {
                        x: x as usize,
                        y: y as usize,
                        state: 0,
                    },
                ))
                .observe(handle_click_event); // Use the new function here
        }
    }

    commands.spawn((
        Text2d::new("Blue Plays"),
        Transform::from_xyz(0.0, 175.0, 0.0),
    ));
}

// New function that handles the click event
fn handle_click_event(
    event: Trigger<Pointer<Pressed>>,
    mut counter: ResMut<BoardState>,
    mut texts: Query<&mut Text2d>,
    mut sprites: Query<(&mut Sprite, &mut CellState)>,
) {
    if let Ok((mut sprite, mut cell_state)) = sprites.get_mut(event.target) {
        let atlas = sprite.texture_atlas.as_mut().unwrap();
        if counter.finished == false && atlas.index == 0 {
            counter.count += 1;
            // Calculate the new index (will alternate between 1 and 2 for the player number)
            let new_index = (counter.count % 2) + 1;
            atlas.index = new_index;
            cell_state.state = new_index;
            println!(
                "Clicked entity {:?}, counter: {}, new index: {}",
                event.target, counter.count, atlas.index
            );
            for mut text in texts.iter_mut() {
                text.0 = if new_index == 1 {
                    "Blue Plays"
                } else {
                    "Red Plays"
                }
                .to_string();
            }
        }
    } else {
        println!(
            "Clicked entity {:?}, counter: {}, but couldn't get TextureAtlas.",
            event.target, counter.count
        );
    }
}

fn get_winner(cells: Query<&CellState>) -> usize {
    let mut board = [[0; 3]; 3];

    for sprite in cells.iter() {
        let x = sprite.x;
        let y = sprite.y;
        board[x][y] = sprite.state;
    }

    // Check rows and columns
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != 0 {
            return board[i][0];
        }
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
            return board[0][i];
        }
    }

    // Check diagonals
    if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[1][1] != 0)
        || (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[1][1] != 0)
    {
        return board[1][1];
    }
    
    // No winner
    0
}

fn check_board(
    cells: Query<&CellState>,
    mut texts: Query<&mut Text2d>,
    mut counter: ResMut<BoardState>,
) {
    if counter.finished {
        return;
    }
    let winner = get_winner(cells);
    if winner != 0 {
        println!("Winner is {}", winner);
        counter.finished = true;
        let winner_text = if winner == 1 { "Red Wins! (R)eset." } else { "Blue Wins! (R)eset." };
        texts.iter_mut().next().unwrap().0 = winner_text.to_string();
    }
    else if counter.count >= 9 {
        counter.finished = true;
        texts.iter_mut().next().unwrap().0 = "Game Over. (R)eset.".to_string();
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Sprite>,
    mut counter: ResMut<BoardState>,
    mut texts: Query<&mut Text2d>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        println!("Resetting counter");
        counter.count = 0;
        counter.finished = false;
        for mut sprite in query.iter_mut() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = 0;
            }
        }
        texts.iter_mut().next().unwrap().0 = "Blue Plays".to_string();
    }
}
