
mod widget {
    pub trait Widget {
        /// Natural width of `self`.
        fn width(&self) -> usize;
    
        /// Draw the widget into a buffer.
        fn draw_into(&self, buffer: &mut dyn std::fmt::Write);
    
        /// Draw the widget on standard output.
        fn draw(&self) {
            let mut buffer = String::new();
            self.draw_into(&mut buffer);
            println!("{buffer}");
        }
    }
}

mod label {
    use super::widget::Widget; // there is a trait method used, so this Trait has to be brought into scope here
    
    pub struct Label {
        label: String,
    }
    
    impl Label {
        pub fn new(label: &str) -> Label {
            Label { label: label.to_owned() }
        }
    }
    
    impl Widget for Label {
        fn width(&self) -> usize {
            self.label.lines().map(|line| line.chars().count()).max().unwrap_or(0)
        }
    
        fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
            writeln!(buffer, "{}", &self.label).unwrap();
        }
    }
}


mod button {
    use super::label;
    use super::widget::Widget; // there is a trait method used, so this Trait has to be brought into scope here
    
    pub struct Button {
        label: label::Label,
    }
    
    impl Button {
        pub fn new(label: &str) -> Button {
            Button { label: label::Label::new(label) }
        }
    }
    
    
    impl Widget for Button {
        fn width(&self) -> usize {
            self.label.width() + 8 // add a bit of padding
        }
    
        fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
            let width = self.width();
            let mut label = String::new();
            self.label.draw_into(&mut label);
    
            writeln!(buffer, "+{:-<width$}+", "").unwrap();
            for line in label.lines() {
                writeln!(buffer, "|{:^width$}|", &line).unwrap();
            }
            writeln!(buffer, "+{:-<width$}+", "").unwrap();
        }
    }
}


mod window {
    use super::widget::Widget; // there is a trait method used, so this Trait has to be brought into scope here
    
    pub struct Window {
        title: String,
        widgets: Vec<Box<dyn Widget>>,
    }
    
    impl Window {
        pub fn new(title: &str) -> Window {
            Window { title: title.to_owned(), widgets: Vec::new() }
        }
    
        pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
            self.widgets.push(widget);
        }
    
        fn inner_width(&self) -> usize {
            std::cmp::max(
                self.title.chars().count(),
                self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
            )
        }
    }
    
    impl Widget for Window {
        fn width(&self) -> usize {
            // Add 4 paddings for borders
            self.inner_width() + 4
        }
    
        fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
            let mut inner = String::new();
            for widget in &self.widgets {
                widget.draw_into(&mut inner);
            }
    
            let inner_width = self.inner_width();
    
            // TODO: Change draw_into to return Result<(), std::fmt::Error>. Then use the
            // ?-operator here instead of .unwrap().
            writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
            writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();
            writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();
            for line in inner.lines() {
                writeln!(buffer, "| {:inner_width$} |", line).unwrap();
            }
            writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        }
    }
}


fn main() {
    use widget::Widget; // there is a trait method used, so this Trait has to be brought into scope here
    use button::Button;
    use window::Window;
    use label::Label;
    
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}