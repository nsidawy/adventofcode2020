use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};

mod border;
mod tile;
mod image;

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let tiles = read_lines(path);
    let border_matches = find_border_matches(&tiles);
    let corners: Vec<tile::Tile> = tiles.clone().into_iter().filter(|t| {
        let mut count = 0;
        let borders = t.get_borders();
        for border in borders.iter() {
            if let tile::BorderType::Edge(_) = border_matches.get(&border.id).unwrap() {
                count += 1;
            }
        }
        let t = t.flip_horizontal().flip_vertical();
        let borders = t.get_borders();
        for border in borders.iter() {
            if let tile::BorderType::Edge(_) = border_matches.get(&border.id).unwrap() {
                count += 1;
            }
        }
        count == 4
    }).collect();
    let result = corners.iter().fold(1u64, |s, t| s * t.id as u64);
    println!("{:#?}", result);
    let image_tiles = stitch_tiles(tiles.len(), &corners[0], &border_matches);
    print_image_id(&image_tiles);
    let mut image = concat_image(&image_tiles);
    for i in &image {
        println!("{}", i.into_iter().collect::<String>())
    }
    let wave_count = find_seamonsters(&mut image);
    println!("Wave count: {}", wave_count);
}

fn find_seamonsters(image: &mut image::Image) -> u32 {
    for x in 0..4 {
        for _ in 0..4 {
            *image = image::rotate_image_right(image);
            let monster_count = find_seamonsters_helper(&image);
            if monster_count.is_some() {
                return monster_count.unwrap();
            }
        }
        if x == 0 {
            *image = image::flip_image_horizontal(image);
        } else if x == 1 {
            *image = image::flip_image_vertical(image);
        } else if x == 2 {
            *image = image::flip_image_horizontal(image);
        }
    }
    0
}

fn find_seamonsters_helper(image: &image::Image) -> Option<u32> {
    let sea_monster = vec!(
        "                  # ".chars().collect::<Vec<char>>(),
        "#    ##    ##    ###".chars().collect::<Vec<char>>(),
        " #  #  #  #  #  #   ".chars().collect::<Vec<char>>());
    
    let mut count = 0;
    for i in 0..(image.len()-sea_monster.len()) {
        for j in 0..(image[0].len()-sea_monster[0].len()) {
            if check_seasmonster(&image, &sea_monster, i, j) {
                count += 1;
            }
        }
    }
    if count == 0 {
        return None;
    }

    let sea_monster_hash = sea_monster.iter().fold(0u32, |s, sm| s + sm.iter().fold(0u32, |s1, sm1| s1 + if *sm1 == '#' { 1 } else { 0 }));
    let image_hash = image.iter().fold(0u32, |s, sm| s + sm.iter().fold(0u32, |s1, sm1| s1 + if *sm1 == '#' { 1 } else { 0 }));

    Some(image_hash - (count * sea_monster_hash))
}

fn check_seasmonster(image: &image::Image, sea_monster: &image::Image, x: usize, y: usize) -> bool {
    for i in 0..sea_monster.len() {
        for j in 0..sea_monster[0].len(){
            if sea_monster[i][j] != '#' {
                continue;
            }
            if image[i+x][j+y] != '#' {
                return false;
            }
        }
    }
    true
}

fn concat_image(image_tiles: &Vec<Vec<tile::Tile>>) -> image::Image {
    let mut image = Vec::new();
    for i1 in 0..image_tiles.len() {
        for i2 in 0..image_tiles[0][0].image.len(){
            if (i2 == 0) || (i2 == image_tiles[0][0].image.len()-1) {
                continue;
            }
            let mut line = Vec::new();
            for j1 in 0..image_tiles[0].len() {
                for j2 in 0..image_tiles[0][0].image[0].len() {
                    if j2 == 0 || j2 == image_tiles[0][0].image[0].len()-1 {
                            continue;
                        }
                    line.push(image_tiles[i1][j1].image[i2][j2]);
                }
            }
            image.push(line);
        }
    }
    image
}

fn stitch_tiles(target: usize, corner: &tile::Tile, border_matches: &HashMap<u32, tile::BorderType>) -> Vec<Vec<tile::Tile>> {
    let mut corner = corner.clone();
    loop {
        if let tile::BorderType::Edge(_) = border_matches.get(&corner.border_top.id).unwrap() {
            if let tile::BorderType::Edge(_) = border_matches.get(&corner.border_left.id).unwrap() {
                break;
            }
        }
        corner = corner.rotate_right();
    }
    let mut i = 0;
    let mut j = 1;
    let mut count = 1;
    let mut image = vec!(vec!());
    image[i].push(corner);
    while count < target {
        if j > 0 {
            let cur = &image[i][j-1];
            if let tile::BorderType::Shared(s1, s2) = &border_matches[&cur.border_right.id] {
                let mut next = if cur.id == s1.id { s2.clone() } else { s1.clone() };
                if cur.border_right.id == next.border_right.id {
                    next = next.flip_vertical();
                } else if cur.border_right.id == next.border_bottom.id {
                    next = next.rotate_right();
                } else if cur.border_right.id == next.border_top.id {
                    next = next.rotate_left().flip_horizontal();
                }
                image[i].push(next.clone());
                if let tile::BorderType::Edge(_) = border_matches[&next.border_right.id] {
                    image.push(vec!());
                    i += 1;
                    j = 0;
                } else {
                    j += 1;
                }
            }
        }
        else {
            let cur = &image[i-1][j];
            if let tile::BorderType::Shared(s1, s2) = &border_matches[&cur.border_bottom.id] {
                let mut next = if cur.id == s1.id { s2.clone() } else { s1.clone() };
                if cur.border_bottom.id == next.border_right.id {
                    next = next.rotate_left();
                } else if cur.border_bottom.id == next.border_left.id {
                    next = next.rotate_right().flip_vertical();
                } else if cur.border_bottom.id == next.border_bottom.id {
                    next = next.flip_horizontal();
                }
                image[i].push(next.clone());
                j += 1;
            }
        }
        count += 1;
    }

    image.remove(image.len()-1);

    image
}

fn print_image_id(image: &Vec<Vec<tile::Tile>>) {
    let ids: Vec<Vec<u32>> = image.iter().map(|ts| ts.iter().map(|t| t.id).collect()).collect();
    println!("{:#?}", ids);
}

fn find_border_matches(tiles: &Vec<tile::Tile>) -> HashMap<u32,tile::BorderType> {
    let mut matches: HashMap<u32, tile::BorderType> = HashMap::new();
    for tile in tiles {
        let borders = tile.get_borders();
        for border in borders.iter() {
            let entry = matches.get(&border.id);
            if entry.is_none() {
                matches.insert(border.id, tile::BorderType::Edge(tile.clone()));
            } else if let tile::BorderType::Edge(t) = entry.unwrap() {
                matches.insert(border.id, tile::BorderType::Shared(t.clone(), tile.clone()));
            } else {
                println!("Triple match");
            }
        }

        let tile = tile.flip_horizontal().flip_vertical();
        let borders = tile.get_borders();
        for border in borders.iter() {
            let entry = matches.get(&border.id);
            if entry.is_none() {
                matches.insert(border.id, tile::BorderType::Edge(tile.clone()));
            } else if let tile::BorderType::Edge(t) = entry.unwrap() {
                matches.insert(border.id, tile::BorderType::Shared(t.clone(), tile.clone()));
            } else {
                println!("Triple match");
            }
        }
    }

    matches
}

fn read_lines(filename: String) -> Vec<tile::Tile> {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut tiles = Vec::new();
    loop {
        let tile = tile::Tile::parse(&mut lines);
        if tile.is_none() {
            break;
        }
        tiles.push(tile.unwrap());
    }

    tiles
}