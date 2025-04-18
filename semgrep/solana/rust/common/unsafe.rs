pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

unsafe trait Foo {
    // methods go here
    fn summarize(&self) -> String;
}

unsafe impl Foo for NewsArticle {
    // method implementations go here
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

unsafe fn foo() {

}

pub unsafe fn foo2() {

}

fn main() {

}