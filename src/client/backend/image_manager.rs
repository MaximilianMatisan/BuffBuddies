use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced_core::image::Handle;
use image::DynamicImage;
use std::collections::HashMap;

pub struct ImageManager {
    /// Mascot images but only the top two thirds of the individual image is shown
    pub cropped_mascot_image_handles: HashMap<Mascot, Handle>,

    /// Mascot images but only the head of the individual image is shown
    pub cropped_mascot_head_handles: HashMap<Mascot, Handle>,
}
impl Default for ImageManager {
    fn default() -> Self {
        let cropped_mascot_image_handles: HashMap<Mascot, Handle> =
            crop_top_two_thirds_of_all_mascot_images();

        let cropped_mascot_head_handles: HashMap<Mascot, Handle> = crop_head_of_all_mascot_images();

        ImageManager {
            cropped_mascot_image_handles,
            cropped_mascot_head_handles,
        }
    }
}
/// Crops all Mascot images so that only the top two thirds of the individual
/// image are shown
fn crop_top_two_thirds_of_all_mascot_images() -> HashMap<Mascot, Handle> {
    let mut cropped_mascot_image_handles: HashMap<Mascot, Handle> = HashMap::new();

    for mascot in Mascot::iter() {
        let mascot_image = image::open(mascot.get_file_path());

        match mascot_image {
            Ok(image) => cropped_mascot_image_handles.insert(mascot, top_two_thirds_crop(image)),
            Err(_) => continue,
        };
    }
    cropped_mascot_image_handles
}
/// Crops all Mascot images so that only the head of the individual
/// image are shown
fn crop_head_of_all_mascot_images() -> HashMap<Mascot, Handle> {
    let mut cropped_mascot_head_handles: HashMap<Mascot, Handle> = HashMap::new();

    for mascot in Mascot::iter() {
        let mascot_image = image::open(mascot.get_file_path());

        match mascot_image {
            Ok(image) => cropped_mascot_head_handles.insert(mascot, crop_head(image)),
            Err(_) => continue,
        };
    }
    cropped_mascot_head_handles
}

/// Crops a DynamicImage so that only the top two thirds of it are shown
fn top_two_thirds_crop(image: DynamicImage) -> Handle {
    let goal_width = image.width();
    let goal_height = image.height() / 3 * 2;
    let cropped = image.crop_imm(0, 0, goal_width, goal_height);

    let cropped_rgba8 = cropped.to_rgba8().into_raw();

    Handle::from_rgba(goal_width, goal_height, cropped_rgba8)
}

fn crop_head(image: DynamicImage) -> Handle {
    //Size of the cropped head
    let head_width = 615;
    let head_height = 440;

    let image_center = image.width() / 2;

    //Center head
    let start_crop_x: u32 = image_center - (0.55 * head_width as f32) as u32;
    let start_crop_y = 0;

    let cropped = image.crop_imm(start_crop_x, start_crop_y, head_width, head_height);

    let cropped_rgba8 = cropped.to_rgba8().into_raw();

    Handle::from_rgba(head_width, head_height, cropped_rgba8)
}
