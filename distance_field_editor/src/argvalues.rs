pub struct ArgValues {
    pub width: f32,
    pub height: f32,

    // None if using default fragment shader
    pub shaderpath: Option<String>,

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
              
            // Some(name) if running an example
            examplename: None,
        
            // Some(id) if downloading a shader
            getid: None,
        
            // true if also running downloaded shader
            andrun: false,
        }
      }
}

