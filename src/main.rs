extern crate image;
extern crate nalgebra as na;

use image::JPEG;
use image::PNG;
use image::Rgb;
use image::{GenericImage,GenericImageView,Pixel};
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use na::base::Matrix3;
use na::base::Matrix3x1;

fn opt_anaglyph_gen(li_p:&image::Rgb<u8>, ri_p:&image::Rgb<u8>) -> Vec<u8> {
    //println!("Building pixel value");
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
    let mut handles = vec![];
    // The dimensions method returns the images width and height.
    //let top_sec = (_r_img.height() as f32 * 0.25) as u32;
    let width = _l_img.width();
    let height = _l_img.height();
    
    let mut out_img = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    let _mod_img = Arc::new(Mutex::new(out_img));
    let _l_img_mut = Arc::new(Mutex::new(_l_img));
    let _r_img_mut = Arc::new(Mutex::new(_r_img));
    let loop_counter = 11;
    let _h_ = height.clone();

    for el in 1..loop_counter {
        
        let _mod_img_a = Arc::clone(&_mod_img);
        let _l_img_a = Arc::clone(&_l_img_mut);
        let _r_img_a = Arc::clone(&_r_img_mut);
        let t_l = (el as f32/loop_counter as f32);
        let _height_a = (_h_ as f32 * t_l) as u32;
        println!("Height: {:?}",_height_a);
        let handle = thread::spawn(move || {
            for _i in 0..width {
                for _it in 0.._height_a {
                    let mut _img  = _mod_img_a.lock().unwrap();
                    let _l_o = _l_img_a.lock().unwrap();
                    let _r_o = _r_img_a.lock().unwrap();
                    let temp_vec = opt_anaglyph_gen(_l_o.get_pixel(_i,_it),_r_o.get_pixel(_i,_it));
                    _img.get_pixel_mut(_i,_it).data = [temp_vec[0],temp_vec[1],temp_vec[2]];
                }
                
            }
        });
        handles.push(handle);
}
   

    for handle in handles {
        handle.join().unwrap();

    }

   let out_val = _mod_img.lock().unwrap();
   out_val.save("outputImage/output.png").unwrap();
}
