pub trait Renderer {
    fn render(&self);
}

pub struct Screen {
    pub components: Vec<Box< dyn Renderer>>,
}

impl Screen {
    pub fn execute(&self) {
        for component in self.components.iter() {
            component.render();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Renderer for Button {
    fn render(&self) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
