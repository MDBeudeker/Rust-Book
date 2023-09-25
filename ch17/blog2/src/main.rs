use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();

    let post = post.approve();

    assert_eq!(&1, post.approvals()); // i can now change the amount of approvals but not get a published post back hmmm
    //assert_eq!("I ate a salad for lunch today", post.content());
}