use crate::client::gui::bb_theme;
use crate::client::backend::mascot::mascot::Mascot;
use crate::client::gui::user_interface::Message;
use iced::advanced::{
    layout::Layout,
    widget::Widget,
    {renderer, Clipboard, Shell}
};
use iced::{mouse, Element};
use iced::{Event, Renderer};
use iced::{Length, Rectangle, Size};
use iced::widget::{row, Space};
use iced_core::event::Status;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Quad;
use iced_core::widget::Tree;
use iced_core::{alignment, Image, Point, Theme};
use iced_core::{image, Border, Shadow};
use iced_core::alignment::Vertical;
use iced_core::image::Handle;
use crate::client::gui::bb_theme::custom_button::{create_element_button, ButtonStyle};
use crate::client::gui::bb_theme::text_format;
use crate::client::gui::bb_theme::text_format::format_button_text;

const INDENT: f32 = DEFAULT_PRESET_HEIGHT/13.0 * SCALE;
const TITLE_FONT_SIZE: f32 = 27.5 * SCALE;
const DEFAULT_PRESET_WIDTH: f32 = 389.0 * SCALE;
const DEFAULT_PRESET_HEIGHT: f32 = 415.0 * SCALE;
const IMAGE_WIDTH: f32 = 184.0 * SCALE;
const IMAGE_HEIGHT: f32 = 256.0 * SCALE;
const SCALE: f32 = 0.9;

pub struct ShopWidget <'a,Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    image: image::Image<<Renderer as iced_core::image::Renderer>::Handle>,
    title: String,
    width: f32,
    height: f32,
    buy_element: Element<'a,Message,Theme,Renderer>,
    font: Option<<Renderer as iced_core::text::Renderer>::Font>,
    active_mascot: Mascot
}

impl<'a> ShopWidget<'a,  Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer + 'a, Message: std::clone::Clone + 'a
{

    pub fn update_active_mascot (mut self, mascot: Mascot) -> Self{
        self.active_mascot = mascot;
        self
    }

    pub fn set_image(mut self, image: image::Image<<Renderer as iced_core::image::Renderer>::Handle>) -> Self {
        self.image = image;
        self
    }
    pub(crate) fn new(name: String, price:usize, mascot: Mascot, message: Message) -> Self {

        let buy_button: iced::widget::Button<'_, Message, Theme, iced::Renderer> =
            create_element_button(mascot.clone(),
                               row![format_button_text(iced::widget::text("Buy")),
                                        Space::with_width(Length::Fill),
                                        row![format_button_text(iced::widget::text(price.to_string())),
                                            iced::widget::image(Handle::from_path("assets/images/coin.png"))
                                            .width(25)
                                            .height(25)]
                                        .align_y(Vertical::Center)
                                        .spacing(5)]
                                   .align_y(Vertical::Center)
                                   .into(),
                               ButtonStyle::Active, None)
                .width(182)
                .height(35);


        ShopWidget {
            image: Image::new(Handle::from_path("assets/images/rare_gacha.png")),
            title: name,
            width: DEFAULT_PRESET_WIDTH,
            height: DEFAULT_PRESET_HEIGHT,
            buy_element: buy_button.on_press(message).into(),
            font: text_format::FIRA_SANS_EXTRABOLD.into(),
            active_mascot: mascot
        }
    }
}

impl<'a,Message, Renderer> Widget<Message, Theme, Renderer> for ShopWidget<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced_core::text::Renderer + iced_core::image::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {

        let mut child_node =
            self.buy_element
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits);

        let widget_size = Size {width: self.width, height: self.height};
        let child_size = child_node.size();

        let child_x = (widget_size.width - child_size.width) / 2.0;
        let child_y = widget_size.height - child_size.height - 18.0;

        child_node = child_node.move_to(Point::new(child_x, child_y));

        Node::with_children(widget_size, vec![child_node])

    }

    fn draw(&self, tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: bb_theme::color::DARKER_CONTAINER_COLOR,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            bb_theme::color::CONTAINER_COLOR,
        );
            renderer.draw_image(self.image.clone(), Rectangle{
                x: layout.bounds().x + DEFAULT_PRESET_WIDTH/2.0 - IMAGE_WIDTH/2.0,
                y: layout.bounds().y + INDENT,
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
            });

        renderer.fill_text(iced_core::text::Text {
            content: self.title.to_string(),
            bounds: layout.bounds().size(),
            size: iced_core::Pixels(TITLE_FONT_SIZE),
            line_height: Default::default(),
            font: self.font.unwrap_or(renderer.default_font()),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            shaping: Default::default(),
            wrapping: Default::default(),
        }, Point {
            x: layout.bounds().x + DEFAULT_PRESET_WIDTH / 2.0,
            y: layout.bounds().y + 2.0 * INDENT + IMAGE_HEIGHT,
        }, bb_theme::color::TEXT_COLOR, *viewport);

        self.buy_element.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.buy_element.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.buy_element));
    }


    fn on_event(&mut self,
                state: &mut Tree,
                event: Event,
                layout: Layout<'_>,
                cursor: Cursor,
                renderer: &Renderer,
                clipboard: &mut dyn Clipboard,
                shell: &mut Shell<'_, Message>,
                viewport: &Rectangle) -> Status
    {
        if state.children.is_empty() {
            return Status::Ignored;
        }
        let child_layout = match layout.children().next() {
            None => return Status::Ignored,
            Some(layout) => layout
        };

        let child_tree = &mut state.children[0];
            self.buy_element
                .as_widget_mut()
                .on_event(child_tree,event,child_layout,cursor,renderer,clipboard,shell,viewport)
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {

        let child_layout = match layout.children().next() {
            None => return mouse::Interaction::None,
            Some(layout ) => layout
        };

        let is_mouse_over = cursor.is_over(child_layout.bounds());

        if is_mouse_over  {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: 'a, Renderer> From<ShopWidget<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a +
      iced_core::image::Renderer
      + iced_core::text::Renderer,
{
    fn from(gacha: ShopWidget<'a,Message, Renderer>) -> Self {
        Self::new(gacha)
    }
}