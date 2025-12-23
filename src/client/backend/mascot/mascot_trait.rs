use iced::Color;

pub trait MascotTrait {
    fn get_name(&self) -> &str;
    fn get_file_path(&self) -> &str;
    fn get_primary_color(&self) -> Color;
    fn get_secondary_color(&self) -> Color;
    fn get_dark_color(&self) -> Color;

}