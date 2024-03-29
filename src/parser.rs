use crate::scene::{Scene, Vertex};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

pub struct Parser {
    lines: Vec<String>,
}

enum LineType {
    Error(String),
    Unimplemented,
    Comment,
    Vertices,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(Self { lines })
    }

    fn get_line_type(&self, line: &String) -> LineType {
        match line.as_str() {
            "#" => return LineType::Comment,
            "v" => return LineType::Vertices,
            "" => return LineType::Error("Line ended".to_string()),
            _ => return LineType::Unimplemented,
        }
    }

    pub fn generate_scene(&self) -> Result<Scene, Error> {
        let mut vertices: Vec<Vertex> = Vec::new();

        for line in self.lines.iter() {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            match self.get_line_type(&split_line[0].to_string()) {
                LineType::Unimplemented => println!("Unimplemented feature {}", split_line[0]),
                LineType::Error(_) => {
                    return Err(Error::new(ErrorKind::UnexpectedEof, line.to_string()))
                }
                LineType::Comment => continue,
                LineType::Vertices => vertices.push(Vertex::new(1., 1., 1., 1.)),
            }
        }
        Ok(Scene {
            vertices
        })
    }
}
