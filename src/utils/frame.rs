pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame(num_cols: usize, num_rows: usize) -> Frame {
    let mut cols = Vec::with_capacity(num_cols);
    for _ in 0..num_cols {
        let mut col = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
