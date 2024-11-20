mod graph;
mod parallel;

use image::{Rgb, RgbImage};
use graph::image::load_graph_from_image;
use crate::graph::kruskal::Kruskal;
use crate::parallel::algo::Algo;

fn main() {
    let image_path = "static/4.1.05.tiff"; // Replace with your image path
    let threshold = 13f32;
    let contrast = -3f32;

    match load_graph_from_image(image_path) {
        Ok(graph) => {
            let mut algo = Kruskal::new(&graph, threshold, contrast);
            let ind = algo.apply_threshold();

            let segmented_image1 = algo.relabel(&graph.pixel);
            algo.apply(ind);
            let segmented_image2 = algo.relabel(&graph.pixel);

            let mut img1 = RgbImage::new(graph.dimensions.0, graph.dimensions.1);

            // Set the pixel values in the image buffer
            for y  in 0..graph.dimensions.1 {
                for x in 0..graph.dimensions.0 {
                    let (r, g, b) = segmented_image1[y as usize][x as usize];
                    img1.put_pixel(y, x, Rgb([255-r, 255-g, 255-b]));
                }
            }

            // Save the segmented image to a file (e.g., segmented_image.png)
            img1.save("segmented_image_serial_1.png").expect("Failed to save the image");

            let mut img2 = RgbImage::new(graph.dimensions.0, graph.dimensions.1);

            // Set the pixel values in the image buffer
            for y  in 0..graph.dimensions.1 {
                for x in 0..graph.dimensions.0 {
                    let (r, g, b) = segmented_image2[y as usize][x as usize];
                    img2.put_pixel(y, x, Rgb([255-r, 255-g, 255-b]));
                }
            }

            // Save the segmented image to a file (e.g., segmented_image.png)
            img2.save("segmented_image_serial.png").expect("Failed to save the image");
            println!("DSU algorithm applied successfully.");
        },
        Err(e) => {
            eprintln!("Failed to load graph from image: {}", e);
        }
    }

    match parallel::graph::load_graph_from_image_with_tiles(image_path, 64, 64, threshold, contrast) {
        Ok(mut graph) => {
            let algo: Algo = Algo::new();
            algo.threshold_merge(&mut graph);
            algo.border_edges_merge(&mut graph);
            let mid_seg = algo.relabel(&graph);
            algo.compute_credit(&mut graph);
            algo.apply_heuristic(&mut graph);
            algo.delay_queue(&mut graph);
            let segmented_image = algo.relabel(&graph);
            let mut img2 = RgbImage::new(graph.width as u32, graph.height as u32);
            for y in 0..graph.height {
                for x in 0..graph.width {
                    let (r, g, b) = mid_seg[y][x];
                    img2.put_pixel(x as u32, y as u32, Rgb([255-r, 255-g, 255-b]));
                }
            }

            img2.save("segmented_image_parallel_1.png").expect("Failed to save the image");
            let mut img = RgbImage::new(graph.width as u32, graph.height as u32);
            for y in 0..graph.height {
                for x in 0..graph.width {
                    let (r, g, b) = segmented_image[y][x];
                    img.put_pixel(x as u32, y as u32, Rgb([255-r, 255-g, 255-b]));
                }
            }

            img.save("segmented_image_parallel.png").expect("Failed to save the image");
            println!("Parallel algorithm applied successfully.");
        },
        Err(e) => {
            eprintln!("Failed to load graph from image with tiles: {}", e);
        }
    }
}