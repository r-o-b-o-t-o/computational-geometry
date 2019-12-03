pub struct Shaders {
    // Vertex shaders
    pub _2d_vs: &'static str,
    pub _3d_vs: &'static str,
    pub obj_vs: &'static str,

    // Fragment shaders
    pub basic_fs: &'static str,
    pub phong_fs: &'static str,
    pub obj_fs: &'static str,
}

pub static SHADERS: Shaders = Shaders {
    _2d_vs: include_str!("../../shaders/2d.vs.glsl"),
    _3d_vs: include_str!("../../shaders/3d.vs.glsl"),
    obj_vs: include_str!("../../shaders/obj.vs.glsl"),

    basic_fs: include_str!("../../shaders/basic.fs.glsl"),
    phong_fs: include_str!("../../shaders/phong.fs.glsl"),
    obj_fs: include_str!("../../shaders/obj.fs.glsl"),
};
