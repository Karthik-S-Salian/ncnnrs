use anyhow::Result;
use image::ImageReader;
use image::imageops::FilterType;
use image::imageops::{crop_imm, resize};
use ncnnrs::{Mat, Net, Option};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_imagenet_labels(path: &str) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

fn print_topk(scores: &[f32], topk: usize, labels: &[String]) {
    let mut indexed: Vec<(usize, f32)> = scores.iter().cloned().enumerate().collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (idx, score) in indexed.into_iter().take(topk) {
        let label = labels
            .get(idx)
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        println!("{} = {:.2}% ({})", idx, score*100.0, label);
    }
}

fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exp: Vec<f32> = logits.iter().map(|&x| (x - max).exp()).collect();
    let sum: f32 = exp.iter().sum();
    exp.iter().map(|x| x / sum).collect()
}

fn main() -> Result<()> {
    let base_dir = env!("CARGO_MANIFEST_DIR");

    // Step 1: Load and resize image
    let img = ImageReader::open(&format!("{}/kite.jpg", base_dir))?.decode()?.to_rgb8();
    let resized = resize(&img, 256, 256, FilterType::Triangle);
    let cropped = crop_imm(&resized, 16, 16, 224, 224).to_image(); // (256-224)/2 = 16

    let flat_rgb: Vec<u8> = cropped.pixels().flat_map(|p| p.0.to_vec()).collect();

    let mut opt = Option::new();
    opt.set_num_threads(16);
    opt.use_vulkan_compute(true);

    let mut net = Net::new();

    net.set_option(&opt);

    
    net.load_param(&format!("{}/opt.param", base_dir))?;
    net.load_model(&format!("{}/opt.bin", base_dir))?;


    // Step 4: Create NCNN input
    let mut mat_in = Mat::from_pixels(&flat_rgb, ncnnrs::MatPixelType::RGB, 224, 224, None)?;
    let mean = [0.485 * 255.0, 0.456 * 255.0, 0.406 * 255.0];
    let std = [0.229 * 255.0, 0.224 * 255.0, 0.225 * 255.0];
    let norm_vals = std.map(|s| 1.0 / s);
    mat_in.substract_mean_normalize(&mean, &norm_vals);

    // Step 5: Inference

    let mut mat_out = Mat::new();

    let mut ex = net.create_extractor();
    ex.input("input", &mut mat_in)?;
    ex.extract("output", &mut mat_out)?;

    println!("output {:?}", mat_out);

    let out_w = mat_out.w();
    let out_h = mat_out.h();
    let out_c = mat_out.c();
    let out_size = (out_w * out_h * out_c) as usize;

    let data_ptr = mat_out.data() as *const f32;
    let scores: &[f32] = unsafe { std::slice::from_raw_parts(data_ptr, out_size) };
    
    let labels = load_imagenet_labels(&format!("{}/imagenet_classes.txt", base_dir))?;
    let probs = softmax(scores);
    print_topk(&probs, 5, &labels);

    Ok(())
}
