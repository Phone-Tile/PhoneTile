#[macro_export]
macro_rules! raylib_str {
    ($expression:expr) => {
        format!("{}\0", $expression).as_ptr() as *const c_char;
    };
}

#[macro_export]
macro_rules! draw {
    ($expr:expr) => {
        raylib::BeginDrawing();
        {
            $expr
        }
        raylib::EndDrawing();
    };
}
