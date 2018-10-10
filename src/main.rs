extern crate image;
extern crate nalgebra as na;

use image::JPEG;
use image::PNG;
use image::math;
use image::Rgb;
use image::{GenericImage,GenericImageView,Pixel};
use std::thread;
use std::time::Duration;
use na::base::Matrix3;
use na::base::Matrix3x1;

fn opt_anaglyph_gen(li_p:&image::Rgb<u8>, ri_p:&image::Rgb<u8>) -> Vec<u8> {
    let _l_vec = li_p.clone();
    let _r_vec = ri_p.clone();
    let mut out_vec = Vec::new();
    let vec_lef = Matrix3x1::new(_l_vec[0] as f32,_l_vec[1] as f32 ,_l_vec[2] as f32);
    let vec_rig = Matrix3x1::new(_r_vec[0] as f32,_r_vec[1] as f32 ,_r_vec[2] as f32);

    let _li_m = Matrix3::new(0.0,0.7,0.3,
                              0.0,0.0,0.0,
                              0.0,0.0,0.0);
    let _ri_m = Matrix3::new(0.0,0.0,0.0,
                              0.0,1.0,0.0,
                              0.0,0.0,1.0);
    
    let o_m = _li_m * vec_lef + _ri_m * vec_rig;
    out_vec.push(o_m[0] as u8);
    out_vec.push(o_m[1] as u8);
    out_vec.push(o_m[2] as u8);

    out_vec


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

    let mut out_img = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    let top_thread = thread::spawn(move || {
        for _i in 0..width {
            for _it in 0..top_sec { 
                let temp_vec = opt_anaglyph_gen(_l_img.get_pixel(_i,_it),_r_img.get_pixel(_i,_it));
                out_img.get_pixel_mut(_i,_it).data = [temp_vec[0],temp_vec[1],temp_vec[2]];
                thread::sleep(Duration::from_millis(1));
            }
            
        }
    });

    let mid_thread = thread::spawn(move || {
        for _j in 0..width {
            
            for _jm in top_sec + 1..mid_sec {
                let temp_vec = opt_anaglyph_gen(_l_img.get_pixel(_j,_jm),_r_img.get_pixel(_j,_jm));
                out_img.get_pixel_mut(_j,_jm).data = [temp_vec[0],temp_vec[1],temp_vec[2]];
                thread::sleep(Duration::from_millis(1));
            }
            
        }
    });

    let bot_thread = thread::spawn(move || {
        for _k in 0..width {
            for _kb in mid_sec + 1..height {
                let temp_vec = opt_anaglyph_gen(_l_img.get_pixel(_k,_kb),_r_img.get_pixel(_k,_kb));
                out_img.get_pixel_mut(_k,_kb).data = [temp_vec[0],temp_vec[1], temp_vec[2]];
                thread::sleep(Duration::from_millis(1));
            }
            
        }
    });

    bot_thread.join().unwrap();
    mid_thread.join().unwrap();
    top_thread.join().unwrap();
    out_img.save("output.png").unwrap();
}
