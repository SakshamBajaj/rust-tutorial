use blog::Post;

// different object depending on state
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("More text that should not be added");

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    let _a = vec![1,2,3,];
}