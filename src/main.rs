use chart_rs::render_engine::Canvas;
use chart_rs::colors::Color;


fn main() {
    println!("Hello, world!");

    test_canvas();
}


fn test_canvas() {
    let mut canvas: Canvas = Canvas::new();
    canvas.set_width(1920.0);
    canvas.set_height(1080.0);
    canvas.set_color(Color::Hex(String::from("#ffffff")));
    canvas.set_opacity(0.0);
    
    let result: Result<(), anyhow::Error> = canvas.export_to_image("test.png");


    println!("{:?}", result);
}