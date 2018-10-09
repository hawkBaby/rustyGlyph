extern crate image;
extern crate nalgebra as na;

use image::JPEG;
use image::PNG;
use image::math;
use image::{GenericImage,GenericImageView};
use std::thread;
use std::time::Duration;
use na::base::Matrix3;
use na::base::Vector3;

fn opt_anaglyph_gen(li_p:Vector3, ri_p:Vector3) -> Vector3 {
    let li_m = Matrix3::new(0.0,0.7,0.3,
                              0.0,0.0,0.0,
                              0.0,0.0,0.0);
    let ri_m = Matrix3::new(0.0,0.0,0.0,
                              0.0,1.0,0.0,
                              0.0,0.0,1.0);
    
    let ra: Vector3 = li_m * li_p + ri_m * ri_p;

}

fn main() {
    println!("....Rusty Anaglyph generator....");
    let _l_img = image::open("./inputImages/imageLeft.jpg").unwrap().to_rgb();
    let _r_img = image::open("./inputImages/imageRight.jpg").unwrap().to_rgb();
    // The dimensions method returns the images width and height.
    let top_sec = (_r_img.height() as f32 * 0.25) as u32;
    let mid_sec = (_r_img.height() as f32 * 0.50) as u32;
    let bot_sec = (_r_img.height() as f32 * 0.75) as u32;
    let width = _l_img.width();
    let height = _l_img.height();

    println!("Top: {:?} Mid: {:?} Bot: {:?}", top_sec, mid_sec, bot_sec);
    println!("Left: {:?} Right: {:?}",_l_img.get_pixel(0,0),_r_img.get_pixel(0,0));

    let top_thread = thread::spawn(move || {
        for i in 0..width {
            for it in 0..top_sec {
                
            }
            //thread::sleep(Duration::from_millis(1));
        }
    });

    let mid_thread = thread::spawn(move || {
        for j in 0..width {
            //thread::sleep(Duration::from_millis(1));
            for jm in top_sec + 1..mid_sec {

            }
        }
    });

    let bot_thread = thread::spawn(move || {
        for k in 0..width {
            for kb in mid_sec + 1..height {

            }
            //thread::sleep(Duration::from_millis(1));
        }
    });

    bot_thread.join().unwrap();
    mid_thread.join().unwrap();
    top_thread.join().unwrap();
}
