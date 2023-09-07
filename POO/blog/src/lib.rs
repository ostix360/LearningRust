pub struct Billet {
    content: String,
}

impl Billet {
    pub fn new() -> Billet {
        Billet {
            content: String::new(),
        }
    }
}

struct Brouillon {
    content: String,
}

impl Brouillon {
    fn ask_reread(self) -> Rereading {
        Rereading {
            content: self.content
        }
    }
}

struct Rereading {
    content: String,
}

impl Rereading {

    fn approve(self) -> Billet {
        Billet {
            content: self.content
        }
    }
}


