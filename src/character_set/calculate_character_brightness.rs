extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

/// Returns the brightness value between 0 and 255 for the input character.
///
/// Based on the algorithm described [here](http://mattmik.com/articles/ascii/ascii.html).
/// The string is rendered in the
/// [Inconsolata font](http://www.levien.com/type/myfonts/inconsolata.html) for the
/// purposes of determining the brightness.
///
/// # Example
/// ```
/// use txtpic_lib::character_set::calculate_character_brightness::calculate_character_brightness;
///
/// let c0 = ' ';
/// let c1 = 'M';
/// 
/// let b0 = calculate_character_brightness(c0);
/// assert_eq!(b0, 0);
///
/// let b1 = calculate_character_brightness(c1);
/// assert_eq!(b1, 54);
pub fn calculate_character_brightness(c: char) -> i32 {
    let string = c.to_string();
    // Generate an in-memory bitmap image of the character
    // Code snippet taken from https://github.com/dylanede/rusttype/blob/master/examples/simple.rs
    // let font_data = include_bytes!("Inconsolata-Regular.ttf");
    let font_data = include_bytes!("NotoSansKR-Regular.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap();

    let height: f32 = 50.0;
    let width = 50;
    let pixel_height = height.ceil() as usize;

    let scale = Scale { x: height * 2.0, y: height };

    // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
    // We don't want to clip the text, so we shift it down with an offset when laying it out.
    // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<PositionedGlyph> = font.layout(&string, scale, offset).collect();
    let glyph: PositionedGlyph = glyphs.last().unwrap().standalone();

    // Represents the character as a vector of "pixel coverage" values
    let mut pixel_data = vec![0.0; width * pixel_height];
    
    // The `v` parameter to the closure in glyph.draw() is a "pixel coverage" value from 0 to 1
    if let Some(bb) = glyph.pixel_bounding_box() {
        glyph.draw(|x, y, v| {
            let positive_v = if v < 0.0 { 0.0 } else { v };
            let scaled_v = positive_v * 255.0;
            let x = x as i32 + bb.min.x;
            let y = y as i32 + bb.min.y;
            if x >= 0 && x < width as i32 &&y >= 0 && y < pixel_height as i32 {
                let x = x as usize;
                let y = y as usize;
                pixel_data[(x + y * width)] = scaled_v;
            }
        })
    }

    let total_brightness = pixel_data.iter().fold(0.0, |mut sum, &value| {sum += value; sum});
    let num_pixels = pixel_data.len() as f32;
    
    if total_brightness == 0.0 || num_pixels == 0.0 {
        0
    }
    else {
        (total_brightness/num_pixels) as i32
    }
}
