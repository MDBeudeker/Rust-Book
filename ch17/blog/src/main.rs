use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(&1 , post.get_approvals());
    post.approve();
    //println!("{}", post.get_approvals()); // since the post now has a different state, the approvals do not count anymore
    assert_eq!("I ate a salad for lunch today", post.content());
}