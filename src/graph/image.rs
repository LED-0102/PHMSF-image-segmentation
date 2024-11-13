use image::GenericImageView;
// Generate graph from an image
use crate::graph::graph::{Graph};

pub fn load_graph_from_image (path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let image = image::open(path)?;

    let (width, height) = image.dimensions();

    let mut graph = Graph::new(width*height);

    for x in 0..height {
        for y in 0..width {
            let neighbours: Vec<(u32, u32)> = vec![(x+1, y), (x, y+1), (x+1, y+1), (x+1, y-1)];

            for i in neighbours {
                if (i.0 < height) && (i.1 < width) {
                    let pixel1 = image.get_pixel(x, y);
                    let pixel2 = image.get_pixel(i.0, i.1);
                    let mut weight: f32 = (pixel1[0] as f32 - pixel2[0] as f32).pow(2) + (pixel1[1] as f32 - pixel2[1] as f32).pow(2) + (pixel1[2] as f32 - pixel2[2] as f32).pow(2);
                    weight = weight.sqrt();
                    graph.add_edge(x*width + y, i.0*width + i.1, weight);
                }
            }
        }
    }

    Ok(graph)
}