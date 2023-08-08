use font_kit::font::Font;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;

#[cfg(any(target_family = "windows", target_os = "macos"))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "ArialMT";
#[cfg(not(any(target_family = "windows", target_os = "macos")))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "DejaVuSans";

fn main() {
    query_all_fonts();
}

/// 获取所有字体
fn query_all_fonts() -> Vec<Handle> {
    let source = SystemSource::new();
    let fonts = source.all_fonts().unwrap();
    println!("{}", fonts.len());
    for font in &fonts {
        if let Ok(font) = font.load() {
            let properties = font.properties();
            println!("postscript_name = {}, name = {}, family_name = {}, style = {}, weight.0 = {}, stretch.0 = {}", 
                font.postscript_name().unwrap_or_else(|| "".to_owned()), 
                font.full_name(), 
                font.family_name(),
                &properties.style.to_string(),
                properties.weight.0.to_string(),
                properties.stretch.0.to_string()
            );
        }
    }
    fonts
}

/// 获取指定字体
fn query_defalut_font(postscript_name: &str) -> Font {
    let font = SystemSource::new()
        .select_by_postscript_name(&postscript_name)
        .expect("Font not found")
        .load()
        .unwrap();
    font
}