pub struct Billet {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Billet {
    pub fn new() -> Billet {
        Billet {
            state: Some(Box::new(Brouillon {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn ask_reread(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.ask_reread());
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn ask_reread(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<self>) -> Box<dyn State>;

    fn content(&self, billet: &Billet) -> &str {
        ""
    }
}

struct Brouillon {}

impl State for Brouillon {
    fn ask_reread(self: Box<Self>) -> Box<dyn State> {
        Box::new(Rereading {})
    }

    fn approve(self: Box<self>) -> Box<dyn State> {
        self
    }
}

struct Rereading {}

impl State for Rereading {
    fn ask_reread(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<self>) -> Box<dyn State> {
        Box::new(Publish {})
    }
}

struct Publish {}

impl State for Publish {
    fn ask_reread(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<self>) -> Box<dyn State> {
        self
    }

    fn content(&self, billet: &Billet) -> &str {
        &billet.content
    }
}

