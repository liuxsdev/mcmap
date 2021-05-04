use std::env;
extern crate nbt;
use nbt::Blob;
use nbt::Value;
use std::fs::File;
use std::io::BufWriter;
extern crate stopwatch;
use stopwatch::Stopwatch;
mod colors;

fn main() {
    let sw = Stopwatch::start_new();
    let args: Vec<String> = env::args().collect();
    let param = &args[1];
    println!("input: {} ", param);
    mcmap_to_png(param.to_string());
    println!("Done! Took {} ms", sw.elapsed_ms());
}

fn mcmap_to_png(filepath: String) {
    let savepath = filepath.replace(".dat", ".png");
    println!("output: {} ", savepath);
    let mut input_file = File::open(filepath).expect("Failed to open file. ");
    let png_file = File::create(savepath).unwrap();
    //read map.dat
    let blob = Blob::from_gzip_reader(&mut input_file).unwrap();
    let all_data = blob.get("data").unwrap();
    if let Value::Compound(map) = all_data {
        let map_colors = &map["colors"];
        if let Value::ByteArray(color_arr) = map_colors {
            // color table
            let color_tab = colors::get_colors();
            let mut v: Vec<u8> = Vec::new();
            for i in 0..16384 {
                let num = color_arr[i] as u8;
                v.push(color_tab[&num][0]);
                v.push(color_tab[&num][1]);
                v.push(color_tab[&num][2]);
            }
            // println!("{:?}", v.len());
            let ref mut w = BufWriter::new(png_file);
            let mut encoder = png::Encoder::new(w, 128, 128);
            encoder.set_color(png::ColorType::RGB);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&v).unwrap();
        }
    }
}
