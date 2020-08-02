pub struct ArgValues {
    pub width: f32,
    pub height: f32,

    // None if using default fragment shader
    pub shaderpath: Option<String>,

    // None if using default textures
    pub texture0path: Option<String>,
    pub texture1path: Option<String>,
    pub texture2path: Option<String>,
    pub texture3path: Option<String>,

    // Some(name) if running an example
    pub examplename: Option<String>,

    // Some(id) if downloading a shader
    pub getid: Option<String>,

    // true if also running downloaded shader
    pub andrun: bool,
}

impl ArgValues {

      pub fn new()->Self {
        ArgValues {
            width: 1000.0,
            height: 1000.0,
        
            // None if using default fragment shader
            shaderpath: Some(String::from("./shaders/default.frag")),
        
            // None if using default textures
            texture0path: None,
            texture1path: None,
            texture2path: None,
            texture3path: None,
        
            // Some(name) if running an example
            examplename: None,
        
            // Some(id) if downloading a shader
            getid: None,
        
            // true if also running downloaded shader
            andrun: false,
        }
      }
}

