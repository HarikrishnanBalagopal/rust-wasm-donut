// pub fn Radians(degrees: f64) -> f64 {
//     return std::f64::consts::PI * degrees / 180.0;
// }

fn calc_pixel_color(x: f64, y: f64, t: f64) -> f64 {
    // println!("{x} {y} {t}");

    let nx = 2.0 * x / (crate::constants::W as f64) - 1.0;
    let ny = 2.0 * y / (crate::constants::H as f64) - 1.0;

    // println!("{nx} {ny} {t}");

    let cam_pos = crate::matrix::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 4.0,
    };
    let cam_dir = crate::matrix::Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let screen_dist = 1.0;
    let screen_cen = cam_pos.add(cam_dir.scale(screen_dist));

    let vec_up = crate::matrix::Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let cam_right = cam_dir.cross(vec_up).normalize();
    let cam_up = cam_right.cross(cam_dir).normalize();
    let screen_pos = screen_cen.add(cam_right.scale(nx)).add(cam_up.scale(ny));
    let light_pos = cam_pos;

    let ray_dir = screen_pos.sub(cam_pos).normalize();

    let mut p = cam_pos;
    let mut d = 10000.0; // max distance
    let r = crate::sdf::get_rot_mat_at_time(t);
    for _ in 0..crate::constants::MAX_ITERATIONS {
        d = crate::sdf::scene_sd(p, r);
        if d < crate::constants::EPS {
            break;
        }
        p = p.add(ray_dir.scale(d));
    }

    if d >= crate::constants::EPS {
            return 0.0;
    }

    // https://en.wikipedia.org/wiki/Phong_reflection_model
    let normal = crate::sdf::normals_sd(p, r);
    let light_dir = light_pos.sub(p).normalize();

    let _dot = light_dir.dot(normal);
    let dot = if _dot < 0.0 { 0.0 } else { _dot };
    //	dot2 := 0 // specular component
    let ip = crate::constants::AMBIENT_REFLECTION_CONSTANT
        * crate::constants::AMBIENT_LIGHT_INTENSITY
        + crate::constants::DIFFUSE_REFLECTION_CONSTANT
            * (dot)
            * crate::constants::DIFFUSE_LIGHT_INTENSITY; //+ Ks*pow(dot2,alpha)*Ims
    return ip;
}

#[no_mangle]
pub extern fn haha(t: f64) -> f64 {
    println!("you gave me {t}");
    42.0
}

#[no_mangle]
pub extern fn step(buf: &mut crate::constants::Buffer, t: f64) {
    for y in 0..crate::constants::H {
        for x in 0..crate::constants::W {
            let color = calc_pixel_color(x as f64, y as f64, t);
            let idx = ((crate::constants::ASCII_LEN as f64) * color / 256.0) as u8
                % crate::constants::ASCII_LEN as u8;
            // fmt.Println("color", color, "idx", idx)
            buf[y][x] = crate::constants::ASCII[idx as usize] as u8;
        }
    }
}

#[no_mangle]
pub extern fn step_global(t: f64) {
    for y in 0..crate::constants::H {
        for x in 0..crate::constants::W {
            let color = calc_pixel_color(x as f64, y as f64, t);
            let idx = ((crate::constants::ASCII_LEN as f64) * color / 256.0) as u8
                % crate::constants::ASCII_LEN as u8;
            // fmt.Println("color", color, "idx", idx)
            unsafe {
                crate::constants::GLOBAL_BUFFER[y][x] = crate::constants::ASCII[idx as usize] as u8;
            }
        }
    }
}

#[no_mangle]
pub extern fn initialize_global_buffer() {
    for i in 0..crate::constants::H {
        unsafe {
            crate::constants::GLOBAL_BUFFER[i][crate::constants::W] = '\n' as u8
        }
    }
}

#[no_mangle]
pub extern fn get_global_address() -> *const crate::constants::Buffer {
    unsafe {        
        &crate::constants::GLOBAL_BUFFER
    }
}

pub fn draw(buf: &crate::constants::Buffer) {
    let mut s = String::new();
    for row in buf {
        let ss = std::str::from_utf8(row).unwrap();
        s += ss;
    }
    println!("{s}")
}

// fn GetBufferAddress() -> *const [[u8; crate::constants::W + 1]; crate::constants::H] {
//     return &crate::constants::BUFFER_1;
// }

pub fn initialize_buffer(buf: &mut crate::constants::Buffer) {
    for i in 0..crate::constants::H {
        buf[i][crate::constants::W] = '\n' as u8
    }
}
