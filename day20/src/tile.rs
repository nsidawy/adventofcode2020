use std::fs::File;
use std::io::{BufReader, Lines};
use regex::Regex;

#[path = "border.rs"]
mod border;
#[path = "image.rs"]
mod image;

#[derive(Debug, Clone)]
pub enum BorderType {
    Edge(Tile),
    Shared(Tile, Tile),
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u32,
    pub image: image::Image,
    pub border_top: border::Border, 
    pub border_bottom: border::Border, 
    pub border_left: border::Border, 
    pub border_right: border::Border, 
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
            border_left: border::Border::build(border_left),
            border_right: border::Border::build(border_right),
            border_top: border::Border::build(border_top),
            border_bottom: border::Border::build(border_bottom),
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
            image: image::flip_image_vertical(&self.image),
            border_top: self.border_top.reverse(),
            border_bottom: self.border_bottom.reverse(),
            border_right: self.border_left.clone(),
            border_left: self.border_right.clone(),
        }
    }

    pub fn flip_horizontal(&self) -> Tile {
        Tile {
            id: self.id,
            image: image::flip_image_horizontal(&self.image),
            border_top: self.border_bottom.clone(),
            border_bottom: self.border_top.clone(),
            border_right: self.border_right.reverse(),
            border_left: self.border_left.reverse(),
        }
    }

    pub fn rotate_right(&self) -> Tile {
        Tile {
            id: self.id,
            image: image::rotate_image_right(&self.image),
            border_top: self.border_left.reverse(),
            border_right: self.border_top.clone(),
            border_bottom: self.border_right.reverse(),
            border_left: self.border_bottom.clone(),
        }
    }

    pub fn rotate_left(&self) -> Tile {
        self.rotate_right().rotate_right().rotate_right()
    }

    pub fn get_borders(&self) -> Vec<border::Border> {
        vec!(
            self.border_top.clone(),
            self.border_bottom.clone(),
            self.border_right.clone(),
            self.border_left.clone(),
        )
    }
}