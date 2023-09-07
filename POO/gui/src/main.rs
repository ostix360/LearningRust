use gui::{Screen, Button};

fn main() {
        let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("Yeah"),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.execute();
}