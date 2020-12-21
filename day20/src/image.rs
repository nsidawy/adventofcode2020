pub type Image = Vec<Vec<char>>;

pub fn flip_image_vertical(image: &Image) -> Image {
    let mut image_next = Vec::new();
    for i in 0..image.len() {
        image_next.push(vec!());
            for j in (0..image[0].len()).rev() {
            image_next[i].push(image[i][j]);
        }
    }
    image_next
}

pub fn flip_image_horizontal(image: &Image) -> Image {
    let mut image_next = Vec::new();
    for i in (0..image.len()).rev() {
        image_next.push(vec!());
            for j in 0..image[0].len() {
            image_next[image.len() - i - 1].push(image[i][j]);
        }
    }
    image_next
}

pub fn rotate_image_right(image: &Image) -> Image {
    let mut image_next = Vec::new();
    for j in 0..image[0].len() {
        image_next.push(vec!());
        for i in (0..image.len()).rev() {
            image_next[j].push(image[i][j]);
        }
    }
    image_next
}