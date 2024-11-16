mod graph;

use image::{Rgb, RgbImage};
use graph::image::load_graph_from_image;
use crate::graph::kruskal::Kruskal;

fn main() {
    let image_path = "static/4.1.07.tiff"; // Replace with your image path
    let threshold = 13f32;
    let contrast = 3f32;

    match load_graph_from_image(image_path) {
        Ok(graph) => {
            println!("Here");
            let mut algo = Kruskal::new(&graph, threshold, contrast);
            let ind = algo.apply_threshold();

            let segmented_image1 = algo.relabel(&graph.pixel);
            algo.apply(ind);
            let segmented_image2 = algo.relabel(&graph.pixel);

            let mut img = RgbImage::new(graph.dimensions.0, graph.dimensions.1);

            // Set the pixel values in the image buffer
            for y  in 0..graph.dimensions.1 {
                for x in 0..graph.dimensions.0 {
                    let (r, g, b) = segmented_image1[y as usize][x as usize];
                    img.put_pixel(y, x, Rgb([r, g, b]));
                }
            }

            // Save the segmented image to a file (e.g., segmented_image.png)
            img.save("segmented_image1.png").expect("Failed to save the image");

            let mut img2 = RgbImage::new(graph.dimensions.0, graph.dimensions.1);

            // Set the pixel values in the image buffer
            for y  in 0..graph.dimensions.1 {
                for x in 0..graph.dimensions.0 {
                    let (r, g, b) = segmented_image2[y as usize][x as usize];
                    img2.put_pixel(y, x, Rgb([r, g, b]));
                }
            }

            // Save the segmented image to a file (e.g., segmented_image.png)
            img2.save("segmented_image2.png").expect("Failed to save the image");
            println!("DSU algorithm applied successfully.");
        },
        Err(e) => {
            eprintln!("Failed to load graph from image: {}", e);
        }
    }
}