use image::imageops::grayscale;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() {
    //store all the images in the vector

    grayscale_folder("raw_image".to_string(), "content".to_string());

}

fn grayscale_folder(read_path: String, write_path: String) {
    let mut write_path = fs::canonicalize(PathBuf::from(&read_path)).unwrap();
    let dir_path = fs::canonicalize(PathBuf::from(&write_path)).unwrap();
    let mut current_path = dir_path.clone();
    if !write_path.is_dir() || !dir_path.is_dir() {
        panic!("Error folders given are not folders");
    }
    let mut new_vector: Vec<String> = Vec::new();

    for x in fs::read_dir(dir_path).unwrap() {
        if let Ok(x) = x {
            let new = x.file_name();
            let xer = format!("{}", new.into_string().unwrap());
            new_vector.push(xer);
        }
    }
    for x in new_vector {
        if x.ends_with(".jpg") || x.ends_with(".jpeg") || x.ends_with(".png") {
            //update the current path
            current_path.push(x.to_owned());
            //open it and init to a var
            let img = image::open(&current_path).unwrap();

            //create a new grayscale image from the normal image
            let new_image = grayscale(&img);
            //now init current path removing the last image
            current_path.pop();
            //update write path adding the new image
            write_path.push(x.to_owned());
            //save the image and reupdate write_path
            new_image.save(&write_path).unwrap();
            //removing the last image
            write_path.pop();
        }
    }
}

#[warn(dead_code)]
fn grayscale_image(path: String) -> Result<PathBuf, Box<dyn Error>> {
    let image = image::open(&path).unwrap();
    let image = grayscale(&image);
    let mut save = PathBuf::from(&path);
    let binding = PathBuf::from(&path);
    let filename = binding.file_name().unwrap();
    save.pop();
    save.push("grayscale");
    save.push(filename);
    let _ = image.save(&save);

    Ok(save)
}