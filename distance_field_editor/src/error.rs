use failure;

use std::io;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;


// Custom error for failing to load shaders
#[derive(Fail, Debug)]
#[fail(display = "Error loading shader {}: {}", shadername, error)]
pub struct LoadShaderError {
    shadername: String,
    error: io::Error,
}

impl LoadShaderError {
    pub fn new(shadername: &str, error: io::Error) -> LoadShaderError {
        LoadShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}

// Custom error for failing to find example shaders
#[derive(Fail, Debug)]
#[fail(display = "Failed to find example shader {}", example)]
pub struct FindExampleShaderError {
    example: String
}
impl FindExampleShaderError {
    pub fn new(example: &str) -> FindExampleShaderError {
        FindExampleShaderError {
            example: example.to_string(),
        }
    }
}

// Custom error for specifying invalid shader id
#[derive(Fail, Debug)]
#[fail(display = "Invalid shader ID: {}", id)]
pub struct InvalidShaderIdError {
    id: String
}
impl InvalidShaderIdError {
    pub fn new(id: &str) -> InvalidShaderIdError {
        InvalidShaderIdError {
            id: id.to_string()
        }
    }
}

// Custom error for failing to save downloaded shader
#[derive(Fail, Debug)]
#[fail(display = "Error saving shader {}: {}", shadername, error)]
pub struct SaveShaderError {
    shadername: String,
    error: io::Error,
}
impl SaveShaderError {
    pub fn new(shadername: &str, error: io::Error) -> SaveShaderError {
        SaveShaderError {
            shadername: shadername.to_string(),
            error,
        }
    }
}


