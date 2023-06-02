use traits::{PressArticle, Resumable};

fn main() {
    let article = PressArticle {
        title: "Sleepy Joe".to_string(),
        author: "Trump".to_string(),
        content: "Joe is a sleepy president! \n yes this article is very short!".to_string(),
        place: "USA".to_string(),
    };

    println!("Let's resum an article: {}", article.resum());
    println!("{}", notify(&article))
}

fn notify(info: &impl Resumable) -> String {
    format!("FLASH: {}", info.resum())
}
