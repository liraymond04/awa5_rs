extern {
    pub fn initwindow(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn settargetfps(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn clearbackground(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawtext(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn iskeydown(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
    pub fn drawcircle(data: *const u8, out: *mut *mut u8, out_len: *mut usize);
}

extern {
    pub fn BeginDrawing();
    pub fn EndDrawing();
}
