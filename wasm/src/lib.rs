extern "C" {
    fn draw_circle(x: f64, y: f64, r: f64);
}

static mut DATA: Circle = Circle {x: 100.0, y: 100.0, r: 20.0};

struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

#[no_mangle]
pub fn exported_func() -> i32 {
    unsafe {
        draw_circle(DATA.x, DATA.y, DATA.r);
        update();
    }
    7
}

#[no_mangle]
pub fn update() {
    unsafe {
        DATA.x += 1.0;
        DATA.y += 1.0;
    }
}
