use crate::argvalues::ArgValues;
use crate::error::{self, LoadShaderError, FindExampleShaderError};

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Default shaders
pub static DEFAULT_VERT_SRC_BUF: &'static [u8] = include_bytes!("../shaders/default.vert");
pub static DEFAULT_FRAG_SRC_STR: &'static str  = include_str!("../shaders/default.frag");

// Fragment shader prefix
const PREFIX: &str = "
    #version 150 core

    uniform float     iGlobalTime;
    uniform float     iTime;
    uniform vec3      iResolution;
    uniform vec4      iMouse;
    uniform int       iFrame;
    uniform sampler2D iChannel0;
    uniform sampler2D iChannel1;
    uniform sampler2D iChannel2;
    uniform sampler2D iChannel3;

    in vec2 fragCoord;
    out vec4 fragColor;
";

// Fragment shader suffix
const SUFFIX: &str = "
    void main() {
        mainImage(fragColor, fragCoord);
    }
";

fn return_load_shader_error<E>(shaderpath: &str, err: io::Error) -> error::Result<E> {
    Err(LoadShaderError::new(shaderpath, err).into())
}

pub fn format_shader_src(src: &str) -> Vec<u8> {
    format!("{}\n{}\n{}", PREFIX, src, SUFFIX).into_bytes()
}

pub fn load_fragment_shader(av: &ArgValues) -> error::Result<Vec<u8>> {
    let frag_src_str = if let Some(ref example) = av.examplename {
        match example.as_ref() {
            "sea"       => "".to_string(),//EXAMPLE_SEASCAPE_STR.to_string(),
            //"elemental-ring" => EXAMPLE_ELEMENTAL_RING_STR.to_string(),
            _                => return Err(FindExampleShaderError::new(example.as_str()).into()),
        }
    } else {
        // Read fragment shader from file into String buffer
        match av.shaderpath {
            Some(ref shaderpath) => {
                let mut frag_src_str = String::new();

                //println!("lol1");
                File::open(&Path::new(&shaderpath)).or_else(|err| {
                    return_load_shader_error(shaderpath, err)
                })?.read_to_string(&mut frag_src_str).or_else(|err| {
                    return_load_shader_error(shaderpath, err)
                })?;

                frag_src_str
            },
            None => {
                String::from(DEFAULT_FRAG_SRC_STR)
            }
        }
    };

    Ok(format_shader_src(&frag_src_str))
}

pub fn load_vertex_shader() -> Vec<u8> {
    DEFAULT_VERT_SRC_BUF.to_vec()
}

pub static ORG_FRAG_SRC_STR: &'static str  = include_str!("../shaders/template.frag");

pub fn generate_shader_from_template(inner:String)->String {
     String::from(ORG_FRAG_SRC_STR).replace("[GENERATED_SHADER_CODE]", &inner[..])  
}


use std::io::prelude::*;

pub fn save_to_file(file_path:&str,shader:String) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    write!(file, "{}", &shader[..])?;
    Ok(())

}
