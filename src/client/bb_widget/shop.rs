use iced_core::image;
const DEFAULT_PRESET_WIDTH: f32 = 389.0 * SCALE;
const DEFAULT_PRESET_HEIGHT: f32 = 377.0 * SCALE;
const SCALE: f32 = 1.0;

pub struct ShopWidget <Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    image: Option<image::Image<<Renderer as iced_core::image::Renderer>::Handle>>,
    title: String,
    width: f32,
    height: f32,
    on_pressed: Option<Message>,
    font: Option<<Renderer as iced_core::text::Renderer>::Font>
}

impl<Message, Renderer> ShopWidget< Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{

    pub fn on_press(mut self, message: Message) -> Self{
        self.on_pressed = Some(message);
        self
    }

    pub fn set_image(mut self, img: image::Image<<Renderer as iced_core::image::Renderer>::Handle>) -> Self {
        self.image = Some(img);
        self
    }
    pub fn set_font(mut self, font: <Renderer as iced_core::text::Renderer>::Font) -> Self{
        self.font = Some(font);
        self
    }

    pub fn set_title(mut self, title: String) -> Self{
        self.title = title;
        self
    }
}

impl<Message, Renderer> Default for ShopWidget <Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    fn default() -> Self {
        ShopWidget {
            image: None,
            title: "Random epic pet-egg".to_string(),
            width: DEFAULT_PRESET_WIDTH,
            height: DEFAULT_PRESET_HEIGHT,
            on_pressed: None,
            font: None
        }
    }
}
