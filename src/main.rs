use macroquad::prelude::*;
use random::prelude::*;

fn window_conf() -> Conf
{
    Conf {
        window_title: "Splotch".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main()
{
    const TILES_X: f32 = 17.0;
    const TILES_Y: f32 = 16.0;

    let mut rng = thread_rng();
    let texture: Texture2D = load_texture("res/Items.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    let mut images = Vec::new();
    let texture_data = texture.get_texture_data();

    let tile_width = (texture_data.width() / TILES_X as usize) as f32;
    let tile_height = (texture_data.height() / TILES_Y as usize) as f32;

    for y in 0 .. TILES_Y as usize
    {
        let y = y as f32;
        for x in 0 .. TILES_X as usize
        {
            let x = x as f32;
            let tile = texture_data.sub_image(Rect::new(
                x * tile_width,
                y * tile_height,
                tile_width,
                tile_height,
            ));
            images.push(tile);
        }
    }

    let mut tiles = Vec::with_capacity(images.len());
    let mut entities = Vec::with_capacity(images.len());

    for image in images
    {
        tiles.push(Texture2D::from_image(&image));
        let vector =
            vec2(rng.gen_range(-1.0 .. 1.0), rng.gen_range(-1.0 .. 1.0))
                .normalize();
        let origin = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let target = origin + vector * 256.0;
        entities.push((origin, target, 0.0));
    }

    let mut time = 0.0;
    let duration = 5.0;

    loop
    {
        time += get_frame_time();

        clear_background(LIGHTGRAY);

        for (i, entity) in entities.iter_mut().enumerate()
        {
            let origin = entity.0;
            let target = entity.1;
            let progress = &mut entity.2;
            *progress += ((time / duration) + rng.gen_range(-32.0 .. 64.0))
                * 0.005
                * get_frame_time();

            draw_texture(
                tiles[i],
                origin.x + (target.x - origin.x) * *progress,
                origin.y + (target.y - origin.y) * *progress,
                WHITE,
            );

            // draw_line(origin.x, origin.y, target.x, target.y, 2.0, RED);
        }

        next_frame().await
    }
}
