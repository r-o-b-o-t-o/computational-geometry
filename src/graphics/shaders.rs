pub struct Shaders {
    // Vertex shaders
    pub _2d_vs: &'static str,

    // Fragment shaders
    pub basic_fs: &'static str,
}

pub static SHADERS: Shaders = Shaders {
    _2d_vs: include_str!("../../shaders/2d.vs.glsl"),
    basic_fs: include_str!("../../shaders/basic.fs.glsl"),
};
