use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;

    fn more(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "headline:{}, by author:{} ({})",
            self.headline, self.author, self.location
        )
    }

    fn more(&self) -> String {
        format!("more headline:{} ({})", self.headline, self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Display for SocialPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "username:{} content:{} reply:{} repost:{}",
            self.username, self.content, self.reply, self.repost
        )
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary + Display>(item: &T) {
    println!("notify3 Breaking news! {}", item.summarize());
    println!("{}", item);
}

pub fn notify4<T>(item: &T)
where
    T: Display + Summary,
{
    println!("where run");
    println!("notify4 Breaking news! {}", item.summarize());
    println!("{}", item);
}

// fn returns_summarizable() -> Box<dyn Summary> {
//     let x = 2;
//     if x == 1 {
//         let news = NewsArticle {
//             headline: "abc".to_string(),
//             location: "xxx".to_string(),
//             author: "daheige".to_string(),
//             content: "hello world".to_string(),
//         };
//
//         Box::new(news)
//     } else {
//         let s = SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             repost: false,
//         };
//         Box::new(s)
//     }
// }

fn main() {
    let news = NewsArticle {
        headline: "abc".to_string(),
        location: "xxx".to_string(),
        author: "daheige".to_string(),
        content: "hello world".to_string(),
    };

    println!("{}", news.summarize());
    println!("{}", news.more());
    notify(&news);

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());
    println!("{}", post.more());
    notify(&post);
    notify2(&post);
    notify3(&post);
    notify4(&post);

    // let n = returns_summarizable();
    // println!("{}", n.summarize());
    // println!("{}", n.more());
}
