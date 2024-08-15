extern {
    pub fn initwindow(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn settargetfps(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn clearbackground(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawtext(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn iskeydown(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawcircle(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setcameraposition(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setcameratarget(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setcameraup(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setcamerafovy(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setcameraprojection(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn beginmode3d(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawcube(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawcubewires(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawgrid(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn loadmodel(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn unloadmodel(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawmodel(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawmodelex(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn loadtexture(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn setmaterialtexture(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn addfloat(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
}

extern {
    pub fn BeginDrawing();
    pub fn EndDrawing();
    pub fn EndMode3D();
}
