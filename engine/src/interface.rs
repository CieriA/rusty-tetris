use game::Game;
use matrix::Matrix;
use vecmath::Point;
use std::error::Error as StdError;
use sdl2::{
    pixels::Color,
    rect::{
        Rect,
        Point as SdlPoint,
    },
    render::{
        Canvas,
        TextureQuery
    },
    ttf::Sdl2TtfContext,
    video::Window,
};

const CELL_SIZE: u32 = 52;
const DISPLAY_SIZE: u32 = 360;
const MARGIN: u32 = 20;
const WALLPAPER_COLOR: Color = Color::RGB(50, 50, 50);
const MATRIX_WIDTH: u32 = Matrix::WIDTH as u32 * CELL_SIZE;
const MATRIX_HEIGHT: u32 = Matrix::HEIGHT as u32 * CELL_SIZE;

pub(crate) const WIDTH: u32 = MATRIX_WIDTH
    + DISPLAY_SIZE
    + MARGIN * 3
    - 1;
pub(crate) const HEIGHT: u32 = MATRIX_HEIGHT
    + MARGIN * 2
    - 1;

pub(crate) fn draw(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, game: &mut Game) -> Result<(), Box<dyn StdError>> {
    // 3 rectangles color
    canvas.set_draw_color(WALLPAPER_COLOR);

    // matrix background
    canvas.fill_rect(Rect::new(
        MARGIN as i32,
        MARGIN as i32,
        MATRIX_WIDTH,
        MATRIX_HEIGHT,
    ))?;

    // score background
    canvas.fill_rect(Rect::new(
        MARGIN as i32 * 2 + MATRIX_WIDTH as i32,
        MARGIN as i32 + CELL_SIZE as i32 * 3 / 2,
        DISPLAY_SIZE,
        CELL_SIZE * 2
    ))?;

    // next_tetromino background
    canvas.fill_rect(Rect::new(
        MARGIN as i32 * 2 + MATRIX_WIDTH as i32,
        MARGIN as i32 + CELL_SIZE as i32 * 19 / 2,
        DISPLAY_SIZE,
        DISPLAY_SIZE
    ))?;

    let color = game.cur_tetromino.as_ref().unwrap().get_color();
    // ghost blocks
    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, 40));
    for coord in game.get_ghost().unwrap().position() {
        canvas.fill_rect(draw_block(coord))?;
    }

    // cur_tetromino blocks
    canvas.set_draw_color(color);
    for coord in game.cur_tetromino.as_ref().unwrap().position() {
        canvas.fill_rect(draw_block(coord))?;
    }

    // next_tetromino blocks
    if let Some(next) = game.bag.last() {
        canvas.set_draw_color(next.get_color());
        for coord in next.cells {
            canvas.fill_rect(Rect::new(
                MARGIN as i32 * 2 + MATRIX_WIDTH as i32 + DISPLAY_SIZE as i32 / 3 + MARGIN as i32 + coord.x as i32 * CELL_SIZE as i32,
                MARGIN as i32 + CELL_SIZE as i32 * 19 / 2 + DISPLAY_SIZE as i32 / 3 + MARGIN as i32 + coord.y as i32 * CELL_SIZE as i32,
                CELL_SIZE,
                CELL_SIZE
            ))?;
        }
    } else {
        game.refill_bag();
    }

    // matrix blocks
    for (y, row) in game.matrix.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            let Some(color) = block else { continue };
            canvas.set_draw_color(*color);
            canvas.fill_rect(Rect::new(
                MARGIN as i32 + x as i32 * CELL_SIZE as i32,
                MARGIN as i32 + y as i32 * CELL_SIZE as i32,
                CELL_SIZE,
                CELL_SIZE,
            ))?;
        }
    }

    // # Matrix grid
    canvas.set_draw_color(Color::BLACK);
    // horizontal
    for i in 0..=Matrix::HEIGHT {
        let y = (i as u32 * CELL_SIZE) as i32 + (MARGIN as i32);
        canvas.draw_line(
            SdlPoint::new(MARGIN as i32, y),
            SdlPoint::new(MATRIX_WIDTH as i32 + MARGIN as i32 * 2, y)
        )?;
    }
    // vertical
    for i in 0..=Matrix::WIDTH {
        let x = (i as u32 * CELL_SIZE) as i32 + (MARGIN as i32);
        canvas.draw_line(
            SdlPoint::new(x, MARGIN as i32 - CELL_SIZE as i32),
            SdlPoint::new(x, HEIGHT as i32)
        )?;
    }

    // # text

    // "NEXT"
    text(
        canvas,
        ttf,
        MARGIN as i32 * 2 + MATRIX_WIDTH as i32 + DISPLAY_SIZE as i32 / 7 * 2,
        MARGIN as i32 + CELL_SIZE as i32 * 8,
        "NEXT",
        "/System/Library/Fonts/Supplemental/Arial.ttf"
    )?;

    // "SCORE"
    text(
        canvas,
        ttf,
        MARGIN as i32 * 2 + MATRIX_WIDTH as i32 + DISPLAY_SIZE as i32 / 4,
        MARGIN as i32 * 3 / 2,
        "SCORE",
        "/System/Library/Fonts/Supplemental/Arial.ttf"
    )?;

    // "{score}"
    text(
        canvas,
        ttf,
        MARGIN as i32 * 2 + MATRIX_WIDTH as i32 + DISPLAY_SIZE as i32 / 2,
        MARGIN as i32 + CELL_SIZE as i32 * 2,
        game.score.to_string().as_str(),
        "/System/Library/Fonts/Supplemental/Arial.ttf"
    )?;

    Ok(())
}

fn draw_block(coord: Point) -> Rect {
    Rect::new(
        MARGIN as i32 + coord.x as i32 * CELL_SIZE as i32,
        MARGIN as i32 + coord.y as i32 * CELL_SIZE as i32,
        CELL_SIZE,
        CELL_SIZE,
    )
}

fn text(canvas: &mut Canvas<Window>, ttf: &Sdl2TtfContext, x: i32, y: i32, text: &str, font_path: &str) -> Result<(), Box<dyn StdError>> {
    let texture_creator = canvas.texture_creator();

    let font = ttf.load_font(font_path, 50)?;

    let surface = font
        .render(text)
        .blended(Color::WHITE)?;

    let texture = texture_creator.create_texture_from_surface(&surface)?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(x, y, width, height);

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
