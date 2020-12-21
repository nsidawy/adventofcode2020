use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Lines};
use regex::Regex;

#[derive(Debug, Clone)]
enum BorderType {
    Edge(Tile),
    Shared(Tile, Tile),
}

#[derive(Debug, Clone)]
struct Border {
    id: u32,
    base: String
}

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    image: Vec<Vec<char>>,
    border_top: Border, 
    border_bottom: Border, 
    border_left: Border, 
    border_right: Border, 
}

impl Border {
    pub fn build(text: String) -> Border {
        Border {
            id: Border::parse_border(&text),
            base: text
        }
    }

    pub fn reverse(&self) -> Border {
        let text = &self.base.chars().rev().collect();
        Border {
            id: Border::parse_border(text),
            base: String::from(text)
        }
    }

    fn parse_border(border: &String) -> u32 {
        let mut value1 = 0;
        for (i, c) in border.chars().enumerate() {
            if c == '#' {
                value1 += 2u32.pow(i as u32);
            }
        }
        value1
    }
}

impl Tile {
    pub fn parse(lines: &mut Lines<BufReader<File>>) -> Option<Tile> {
        let line = lines.next();
        let line = line.unwrap_or(Ok(String::from(""))).unwrap();
        if line.len() == 0 {
            return None;
        }

        let reg = Regex::new(r"Tile (\d+):").unwrap();
        let id = reg.captures(&line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();

        let mut line_list: Vec<String> = Vec::new();
        loop {
            let line = lines.next().unwrap().unwrap();
            if line.len() == 0 {
                break;
            }
            line_list.push(line);
        }

        let border_top = line_list[0].clone();
        let border_bottom = line_list[line_list.len()-1].clone();
        let border_left = line_list.iter().fold(String::from(""), |mut s,v| {s.push_str(&v[..1]); s});
        let border_right = line_list.iter().fold(String::from(""), |mut s,v| {s.push_str(&v[v.len()-1..]); s});

        let image = line_list.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();

        Some(Tile {
            id,
            image,
            border_left: Border::build(border_left),
            border_right: Border::build(border_right),
            border_top: Border::build(border_top),
            border_bottom: Border::build(border_bottom),
        })
    }

    pub fn flip_vertical(&self) -> Tile {
        let mut image_next = Vec::new();
        for i in 0..self.image.len() {
            image_next.push(vec!());
                for j in (0..self.image[0].len()).rev() {
                image_next[i].push(self.image[i][j]);
            }
        }
        Tile {
            id: self.id,
            image: Tile::flip_grid_vertical(&self.image),
            border_top: self.border_top.reverse(),
            border_bottom: self.border_bottom.reverse(),
            border_right: self.border_left.clone(),
            border_left: self.border_right.clone(),
        }
    }

    pub fn flip_horizontal(&self) -> Tile {
        Tile {
            id: self.id,
            image: Tile::flip_grid_horizontal(&self.image),
            border_top: self.border_bottom.clone(),
            border_bottom: self.border_top.clone(),
            border_right: self.border_right.reverse(),
            border_left: self.border_left.reverse(),
        }
    }

    pub fn rotate_right(&self) -> Tile {
        Tile {
            id: self.id,
            image: Tile::rotate_grid_right(&self.image),
            border_top: self.border_left.reverse(),
            border_right: self.border_top.clone(),
            border_bottom: self.border_right.reverse(),
            border_left: self.border_bottom.clone(),
        }
    }

    pub fn rotate_left(&self) -> Tile {
        self.rotate_right().rotate_right().rotate_right()
    }

    pub fn get_borders(&self) -> Vec<Border> {
        vec!(
            self.border_top.clone(),
            self.border_bottom.clone(),
            self.border_right.clone(),
            self.border_left.clone(),
        )
    }

    pub fn flip_grid_vertical(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut image_next = Vec::new();
        for i in 0..image.len() {
            image_next.push(vec!());
                for j in (0..image[0].len()).rev() {
                image_next[i].push(image[i][j]);
            }
        }
        image_next
    }

    pub fn flip_grid_horizontal(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut image_next = Vec::new();
        for i in (0..image.len()).rev() {
            image_next.push(vec!());
                for j in 0..image[0].len() {
                image_next[image.len() - i - 1].push(image[i][j]);
            }
        }
        image_next
    }

    pub fn rotate_grid_right(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut image_next = Vec::new();
        for j in 0..image[0].len() {
            image_next.push(vec!());
            for i in (0..image.len()).rev() {
                image_next[j].push(image[i][j]);
            }
        }
        image_next
    }
}

fn main() {   
    let path = format!("{}\\input\\input.txt", env::current_dir().unwrap().to_str().unwrap()); 
    let tiles = read_lines(path);
    let border_matches = find_border_matches(&tiles);
    let corners: Vec<Tile> = tiles.clone().into_iter().filter(|t| {
        let mut count = 0;
        let borders = t.get_borders();
        for border in borders.iter() {
            if let BorderType::Edge(_) = border_matches.get(&border.id).unwrap() {
                count += 1;
            }
        }
        let t = t.flip_horizontal().flip_vertical();
        let borders = t.get_borders();
        for border in borders.iter() {
            if let BorderType::Edge(_) = border_matches.get(&border.id).unwrap() {
                count += 1;
            }
        }
        count == 4
    }).collect();
    let result = corners.iter().fold(1u64, |s, t| s * t.id as u64);
    println!("{:#?}", result);
    let image_tiles = complete_puzzle(tiles.len(), &corners[0], &border_matches);
    print_image_id(&image_tiles);
    let image = concat_image(&image_tiles);
    for i in &image {
        println!("{}", i.into_iter().collect::<String>())
    }

    let image = Tile::flip_grid_horizontal(&image);
    let monster_count = find_seamonsters(&image);
    if monster_count.is_some() {
        println!("{}", monster_count.unwrap());
        return;
    }

    let image = Tile::rotate_grid_right(&image);
    let monster_count = find_seamonsters(&image);
    if monster_count.is_some() {
        println!("{}", monster_count.unwrap());
        return;
    }

    let image = Tile::rotate_grid_right(&image);
    let monster_count = find_seamonsters(&image);
    if monster_count.is_some() {
        println!("{}", monster_count.unwrap());
        return;
    }

    let image = Tile::rotate_grid_right(&image);
    let monster_count = find_seamonsters(&image);
    if monster_count.is_some() {
        println!("{}", monster_count.unwrap());
        return;
    }
}

fn find_seamonsters(image: &Vec<Vec<char>>) -> Option<u32> {
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

fn check_seasmonster(image: &Vec<Vec<char>>, sea_monster: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
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

fn concat_image(image_tiles: &Vec<Vec<Tile>>) -> Vec<Vec<char>> {
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

fn complete_puzzle(target: usize, corner: &Tile, border_matches: &HashMap<u32, BorderType>) -> Vec<Vec<Tile>> {
    let mut corner = corner.clone();
    loop {
        if let BorderType::Edge(_) = border_matches.get(&corner.border_top.id).unwrap() {
            if let BorderType::Edge(_) = border_matches.get(&corner.border_left.id).unwrap() {
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
            if let BorderType::Shared(s1, s2) = &border_matches[&cur.border_right.id] {
                let mut next = if cur.id == s1.id { s2.clone() } else { s1.clone() };
                if cur.border_right.id == next.border_right.id {
                    next = next.flip_vertical();
                } else if cur.border_right.id == next.border_bottom.id {
                    next = next.rotate_right();
                } else if cur.border_right.id == next.border_top.id {
                    next = next.rotate_left().flip_horizontal();
                }
                image[i].push(next.clone());
                if let BorderType::Edge(_) = border_matches[&next.border_right.id] {
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
            if let BorderType::Shared(s1, s2) = &border_matches[&cur.border_bottom.id] {
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

fn print_image_id(image: &Vec<Vec<Tile>>) {
    let ids: Vec<Vec<u32>> = image.iter().map(|ts| ts.iter().map(|t| t.id).collect()).collect();
    println!("{:#?}", ids);
}

fn find_border_matches(tiles: &Vec<Tile>) -> HashMap<u32,BorderType> {
    let mut matches: HashMap<u32, BorderType> = HashMap::new();
    for tile in tiles {
        let borders = tile.get_borders();
        for border in borders.iter() {
            let entry = matches.get(&border.id);
            if entry.is_none() {
                matches.insert(border.id, BorderType::Edge(tile.clone()));
            } else if let BorderType::Edge(t) = entry.unwrap() {
                matches.insert(border.id, BorderType::Shared(t.clone(), tile.clone()));
            } else {
                println!("Triple match");
            }
        }

        let tile = tile.flip_horizontal().flip_vertical();
        let borders = tile.get_borders();
        for border in borders.iter() {
            let entry = matches.get(&border.id);
            if entry.is_none() {
                matches.insert(border.id, BorderType::Edge(tile.clone()));
            } else if let BorderType::Edge(t) = entry.unwrap() {
                matches.insert(border.id, BorderType::Shared(t.clone(), tile.clone()));
            } else {
                println!("Triple match");
            }
        }
    }

    matches
}

fn read_lines(filename: String) -> Vec<Tile> {
    let file = File::open(filename).unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut tiles = Vec::new();
    loop {
        let tile = Tile::parse(&mut lines);
        if tile.is_none() {
            break;
        }
        tiles.push(tile.unwrap());
    }

    tiles
}