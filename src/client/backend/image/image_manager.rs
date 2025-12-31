use std::collections::HashMap;
use iced_core::image::Handle;
use image::{DynamicImage};
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;

pub struct ImageManager {
    pub cropped_mascot_image_handles: HashMap<Mascot, Handle>
}
impl Default for ImageManager {
    fn default() -> Self {
        let mut cropped_mascot_image_handles: HashMap<Mascot, Handle> = HashMap::new();

        for mascot in Mascot::iter() {
            let mascot_image = image::open(mascot.get_file_path());

            match mascot_image {
                Ok(image) =>
                    cropped_mascot_image_handles.insert(mascot, top_two_thirds_crop(image)),
                Err(_) => continue,
            };
        }

        ImageManager {
            cropped_mascot_image_handles
        }
    }
}

fn top_two_thirds_crop(image: DynamicImage) -> Handle {
    let goal_width = image.width();
    let goal_height = image.height()/3 * 2;
    let cropped = image.crop_imm(0, 0, goal_width, goal_height);

    let cropped_rgba8 = cropped.to_rgba8().into_raw();

    Handle::from_rgba(goal_width, goal_height, cropped_rgba8)
}
