mod minecraft_block;
mod minecraft_colour;

use image::{
    imageops::{dither, ColorMap},
    Rgb, RgbImage,
};
use minecraft_colour::{build_colour_lookup_hashmap, get_colour_index_and_shade, ColourShade};

use crate::{
    minecraft_block::get_block_for_colour_index_hashmap, minecraft_colour::get_minecraft_colours,
};
use minecraft_block::{ColumnBlock, ColumnPosition, MinecraftBlock};

use std::{cmp::Ordering, collections::HashMap, iter};

fn main() {
    let path = "/home/sarunint/Downloads/Pack.png";
    let path_out = "/home/sarunint/Downloads/Pack.png.nbt";
    let colours = get_minecraft_colours();
    let colour_index_to_block_hashmap = get_block_for_colour_index_hashmap();
    let dithered_image = get_dithered_image(path, &colours).unwrap();
    assert!(dithered_image.dimensions().0 == 128 && dithered_image.dimensions().1 == 128);
    write_dithered_image_to_nbt(
        path_out,
        dithered_image,
        &colours,
        &colour_index_to_block_hashmap,
    );
}

fn write_dithered_image_to_nbt(
    path: &str,
    dithered_image: RgbImage,
    colours: &Vec<Rgb<u8>>,
    block_mapping: &HashMap<i32, MinecraftBlock>,
) {
    for x in 0..128 {
        let column: Vec<&Rgb<u8>> = (0..128)
            .into_iter()
            .map(|z| dithered_image.get_pixel(x, z))
            .collect();
        let blocks = convert_column_to_blocks(column, colours, block_mapping);
    }
}

fn convert_column_to_blocks<'a>(
    column: Vec<&Rgb<u8>>,
    colours: &Vec<Rgb<u8>>,
    block_mapping: &HashMap<i32, MinecraftBlock>,
) -> Vec<ColumnBlock<'a>> {
    let mut out: Vec<ColumnBlock> = vec![];
    let lookup_table = build_colour_lookup_hashmap(colours);

    // The idea of this rewrite is that we should do the water first, and check if the next block is dark non-water, and this block is light water, we add another support block for it.

    // Pass 1: detect all water segments in to `segments` vector

    let mut segments: Vec<usize> = vec![];
    let mut seen_water = true;

    segments.push(0);

    for (index, pixel) in column.iter().enumerate() {
        let colour_index_and_shade = get_colour_index_and_shade(pixel, &lookup_table);
        match colour_index_and_shade {
            Some((colour_index, _shade)) => {
                if colour_index == 11 {
                    if !seen_water {
                        segments.push(index);
                        seen_water = true;
                    }
                } else if seen_water {
                    segments.push(index);
                    seen_water = false;
                }
            }
            None => {}
        }
    }

    segments.push(column.len());

    if seen_water {
        segments.push(column.len());
    }

    println!("{:?}", segments);

    // Pass 2: iterate over water then non-water segment pairs

    for item in segments.windows(3).step_by(2) {
        match item {
            [water_start, non_water_start, next_water_start] => {
                println!("{} {} {}", water_start, non_water_start, next_water_start);
                // In this context,
                // `water_blocks` is all water blocks excepts the last one
                // `noobline` is the noobline (duh!), or the last water
                // `non_water_blocks` is the rest of the blocks

                let mut water_blocks: Vec<ColumnBlock> = vec![];
                let mut noobline: Vec<ColumnBlock> = vec![];
                let mut non_water_blocks: Vec<ColumnBlock> = vec![];

                // Create `water_blocks`

                if non_water_start - water_start >= 2 {
                    for i in water_start.clone()..non_water_start - 1 {
                        water_blocks.push(ColumnBlock {
                            position: ColumnPosition { z: i as i32, y: 0 },
                            block: block_mapping.get(&-1).unwrap().clone(),
                        });
                        let colour = column[i];
                        let (_index, shade) =
                            get_colour_index_and_shade(colour, &lookup_table).unwrap();
                        let water_height = match shade {
                            ColourShade::Light => 1,
                            ColourShade::Normal => 5,
                            ColourShade::Dark => 10,
                        };

                        for j in 0..water_height {
                            water_blocks.push(ColumnBlock {
                                position: ColumnPosition {
                                    z: i as i32,
                                    y: j + 1 as i32,
                                },
                                block: block_mapping.get(&11).unwrap().clone(),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Rewrite the stuff~

    // let mut current_y = 0;
    // let mut current_z = 0;
    // // Noobline
    // let mut current_run: Vec<ColumnBlock> = vec![];
    // current_run.push(ColumnBlock {
    //     position: ColumnPosition { y: 0, z: 0 },
    //     block: MinecraftBlock {
    //         name: "minecraft:cobblestone",
    //         state: HashMap::new(),
    //     },
    // });
    // current_run.push(ColumnBlock {
    //     position: ColumnPosition { y: -1, z: 0 },
    //     block: MinecraftBlock {
    //         name: "minecraft:cobblestone",
    //         state: HashMap::new(),
    //     },
    // });
    // let lookup_table = build_colour_lookup_hashmap(colours);
    // // Extra support block for noobline
    // let first_colour_index_and_shade = get_colour_index_and_shade(column[0], &lookup_table);
    // if matches!(first_colour_index_and_shade, Some((index, shade)) if index != 11 && matches!(shade, ColourShade::Dark))
    // {
    //     current_run.push(ColumnBlock {
    //         position: ColumnPosition { z: 0, y: -2 },
    //         block: MinecraftBlock {
    //             name: "minecraft:cobblestone",
    //             state: HashMap::new(),
    //         },
    //     })
    // }
    // current_z += 1;
    // let mut last_water_height: Option<i32> = None;
    // for window in iter::once::<&[&Rgb<u8>]>(&[&Rgb([0u8, 0u8, 0u8]), &column[0], &column[1]])
    //     .chain(column.windows(3))
    // {
    //     let colour_index_and_shade_this_block =
    //         get_colour_index_and_shade(window[1], &lookup_table);
    //     let colour_index_and_shade_next_block =
    //         get_colour_index_and_shade(window[2], &lookup_table);

    //     match colour_index_and_shade_this_block.zip(colour_index_and_shade_next_block) {
    //         Some(((index_this, shade_this), (index_next, shade_next))) => {
    //             if index_this != 11 {
    //                 match shade_this {
    //                     ColourShade::Light => {
    //                         current_y += 1;
    //                     }
    //                     ColourShade::Normal => {}
    //                     ColourShade::Dark => {
    //                         current_y -= 1;
    //                     }
    //                 }
    //                 // The block
    //                 current_run.push(ColumnBlock {
    //                     position: ColumnPosition {
    //                         z: current_z,
    //                         y: current_y,
    //                     },
    //                     block: MinecraftBlock {
    //                         name: "minecraft:that_block",
    //                         state: HashMap::new(),
    //                     },
    //                 });
    //                 // The support block
    //                 current_run.push(ColumnBlock {
    //                     position: ColumnPosition {
    //                         z: current_z,
    //                         y: current_y - 1,
    //                     },
    //                     block: MinecraftBlock {
    //                         name: "minecraft:cobblestone",
    //                         state: HashMap::new(),
    //                     },
    //                 });
    //                 // Extra support block
    //                 if (matches!(shade_this, ColourShade::Light)
    //                     || matches!(shade_next, ColourShade::Dark))
    //                 {
    //                     current_run.push(ColumnBlock {
    //                         position: ColumnPosition {
    //                             z: current_z,
    //                             y: current_y - 2,
    //                         },
    //                         block: MinecraftBlock {
    //                             name: "minecraft:cobblestone",
    //                             state: HashMap::new(),
    //                         },
    //                     })
    //                 }
    //             } else {
    //                 // We get water

    //                 // 1 End current Run

    //                 // 1.1 Sort current run by Lower Z, Higher Y

    //                 current_run.sort_by(|a, b| -> Ordering {
    //                     if a.position.z < b.position.z {
    //                         Ordering::Less
    //                     } else if a.position.z > b.position.z {
    //                         Ordering::Greater
    //                     } else if a.position.y > b.position.y {
    //                         Ordering::Less
    //                     } else if a.position.y < b.position.y {
    //                         Ordering::Greater
    //                     } else {
    //                         Ordering::Equal
    //                     }
    //                 });

    //                 // 2. Detect plateaus

    //                 let mut has_ascended = false;
    //                 let mut last_position = current_run[0].position.clone();
    //                 let mut plateaus: Vec<(usize, usize)> = vec![];
    //                 let mut current_plateau_start_index = 0;

    //                 plateaus.push((0, 0));

    //                 for (index, block) in current_run.iter().enumerate() {
    //                     if block.position.z > last_position.z {
    //                         if has_ascended && block.position.y < last_position.y {
    //                             has_ascended = false;
    //                             plateaus.push((current_plateau_start_index, index));
    //                         } else if block.position.y > last_position.y {
    //                             has_ascended = true;
    //                             current_plateau_start_index = index;
    //                         }
    //                         last_position = block.position.clone();
    //                     }
    //                 }

    //                 plateaus.push((current_run.len(), current_run.len()));

    //                 // 1.3 Pulldowns

    //                 let mut non_plateau_pulldown_heights =
    //                     [None as Option<i32>, None as Option<i32>];
    //                 while plateaus.len() > 1 {
    //                     let next_valley_pulldown_height = current_run[plateaus[0].1..plateaus[1].0]
    //                         .iter()
    //                         .min_by_key(|block| block.position.y)
    //                         .unwrap()
    //                         .position
    //                         .y;
    //                     let pulldown_height = match last_water_height {
    //                         Some(i) => {
    //                             if
    //                         }
    //                     }
    //                     for i in plateaus[0].1..plateaus[1].0 {
    //                         current_run[i].position.y -= pulldown_height;
    //                     }
    //                     non_plateau_pulldown_heights[1] = Some(pulldown_height);
    //                     let plateau_pulldown_height = match non_plateau_pulldown_heights[0] {
    //                         None => non_plateau_pulldown_heights[1].unwrap(),
    //                         Some(i) => {
    //                             let j = non_plateau_pulldown_heights[0].unwrap();
    //                             if i < j {
    //                                 i
    //                             } else {
    //                                 j
    //                             }
    //                         }
    //                     };
    //                     for i in plateaus[0].0..plateaus[0].1 {
    //                         current_run[i].position.y -= plateau_pulldown_height;
    //                     }
    //                     non_plateau_pulldown_heights[0] = non_plateau_pulldown_heights[1];
    //                     plateaus.remove(0);
    //                 }

    //                 // 1.4 Add to out

    //                 out.append(&mut current_run);
    //                 current_run.clear();

    //                 // We process water

    //                 // Support block

    //                 out.push(ColumnBlock {
    //                     position: ColumnPosition { z: current_z, y: 0 },
    //                     block: MinecraftBlock {
    //                         name: "minecraft:cobblestone",
    //                         state: HashMap::new(),
    //                     },
    //                 });

    //                 let height = match shade_this {
    //                     ColourShade::Light => 2,
    //                     ColourShade::Normal => 5,
    //                     ColourShade::Dark => 10,
    //                 };

    //                 for i in 0..height {
    //                     out.push(ColumnBlock {
    //                         position: ColumnPosition {
    //                             z: current_z,
    //                             y: i + 1,
    //                         },
    //                         block: MinecraftBlock {
    //                             name: "minecraft:oak_leaves",
    //                             state: [("waterlogged", "true")].into(),
    //                         },
    //                     });
    //                 }
    //                 last_water_height = Some(height - 1);
    //             }
    //         }
    //         None => {}
    //     }
    //     current_z += 1;
    // }
    out
}

fn get_dithered_image(path: &str, colours: &Vec<Rgb<u8>>) -> Option<RgbImage> {
    // const PATH: &str = "/home/sarunint/Downloads/Pack.png";
    // const PATH_OUT: &str = "/home/sarunint/Downloads/Pack_dithered_correct_colour.png";
    let img = image::open(path);

    match img {
        Ok(img) => {
            let mut buffer = img.to_rgb8();
            let the_map = MinecraftColourMap::init(colours);
            dither(&mut buffer, &the_map);
            // match buffer.save(PATH_OUT) {
            //     Ok(_) => {
            //         println!("Saved to {}", PATH_OUT);
            //     }
            //     Err(s) => {
            //         println!("Error {:?}", s);
            //     }
            // }
            Some(buffer)
        }
        Err(_) => None,
    }
}

struct MinecraftColourMap {
    map_cache: HashMap<Rgb<u8>, usize>,
    allowed_colours: Vec<Rgb<u8>>,
}

fn calculate_distance(colour1: &Rgb<u8>, colour2: &Rgb<u8>) -> i32 {
    let distance_r: i32 = colour1.0[0].abs_diff(colour2.0[0]) as i32;
    let distance_g: i32 = colour1.0[1].abs_diff(colour2.0[1]) as i32;
    let distance_b: i32 = colour1.0[2].abs_diff(colour2.0[2]) as i32;
    distance_r * distance_r + distance_g * distance_g + distance_b * distance_b
}

impl MinecraftColourMap {
    fn init(allowed_colours: &Vec<Rgb<u8>>) -> MinecraftColourMap {
        MinecraftColourMap {
            map_cache: HashMap::new(),
            allowed_colours: allowed_colours.clone(),
        }
    }
}

impl ColorMap for MinecraftColourMap {
    type Color = Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let cached_value = self.map_cache.get(&color);
        match cached_value {
            Some(i) => i.clone(),
            None => {
                struct MinimumInfo {
                    index: usize,
                    distance: i32,
                }
                let mut minimum_info: Option<MinimumInfo> = None;
                for (index, other_color) in self.allowed_colours.iter().enumerate() {
                    let distance = calculate_distance(color, other_color);
                    match &minimum_info {
                        Some(x) => {
                            if distance < x.distance {
                                minimum_info = Some(MinimumInfo { index, distance });
                            }
                        }
                        None => {
                            minimum_info = Some(MinimumInfo { index, distance });
                        }
                    }
                }
                match minimum_info {
                    Some(i) => i.index,
                    None => 0,
                }
            }
        }
    }

    fn map_color(&self, color: &mut Self::Color) {
        let out_colour = self.allowed_colours[self.index_of(color)];
        color.0[0] = out_colour.0[0];
        color.0[1] = out_colour.0[1];
        color.0[2] = out_colour.0[2];
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        if index < self.allowed_colours.len() {
            Some(self.allowed_colours[index])
        } else {
            None
        }
    }

    fn has_lookup(&self) -> bool {
        true
    }
}
