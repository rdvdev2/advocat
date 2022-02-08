use termion::color;
use crate::ux;

pub struct DiffDisplay {
    text: String,
    side_width: usize,
    left_color: &'static dyn color::Color,
    right_color: &'static dyn color::Color
}

impl DiffDisplay {
    pub fn new(left_title: &str, right_title: &str, left_color: &'static dyn color::Color, right_color: &'static dyn color::Color) -> Self {
        let side_width = ((ux::get_terminal_width() - 7) / 2) as usize;
        let mut dd = DiffDisplay { text: String::new(), side_width, left_color, right_color };
        dd.draw_horizontal_line('╭', '┬', '╮');
        dd.write_centered_row(left_title, right_title);
        dd.draw_horizontal_line('├', '┼', '┤');
        dd
    }

    fn draw_horizontal_line(&mut self, left: char, mid: char, right: char) {
        self.text.push_str(format!("{l}─{:─^w$}─{m}─{:─^w$}─{r}\n", "", "",
                                   l = left, m = mid, r = right, w = self.side_width).as_str());
    }

    pub fn end(&mut self) {
        self.draw_horizontal_line('╰', '┴', '╯');
    }

    fn write_centered_row(&mut self, left: &str, right: &str) {
        let left = self.trim_line(left);
        let right = self.trim_line(right);
        self.text.push_str(format!("│ {l:^w$} │ {r:^w$} │\n",
                                   l = left, r = right, w = self.side_width).as_str());
    }

    fn write_row(&mut self, left: &str, mid: char, right: &str, left_color: &dyn color::Color, right_color: &dyn color::Color) {
        let left = self.trim_line(left);
        let right = self.trim_line(right);
        self.text.push_str(format!("│ {lc}{l:w$}{nc} {m} {rc}{r:w$}{nc} │\n",
                                   l = left, m = mid, r = right,
                                   lc = color::Fg(left_color),
                                   rc = color::Fg(right_color),
                                   nc = color::Fg(color::Reset),
                                   w = self.side_width).as_str());
    }

    pub fn write_left(&mut self, left: &str) {
        self.write_row(left, '<', "", self.left_color, self.right_color);
    }

    pub fn write_right(&mut self, right: &str) {
        self.write_row("", '>', right, self.left_color, self.right_color);
    }

    pub fn write_both(&mut self, left: &str, right: &str) {
        self.write_row(left, '│', right, &color::Reset, &color::Reset);
    }

    fn trim_line(&self, line: &str) -> String {
        if line.len() <= self.side_width {
            line.to_owned()
        } else {
            let line = &line[0..self.side_width-3];
            line.to_owned() + "..."
        }
    }
    
    pub fn build(self) -> String {
        self.text
    }
}