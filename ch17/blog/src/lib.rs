pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
    approvals: u32,
}

impl Post{
    pub fn new() -> Post{
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approvals: 0,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn get_approvals(&self) -> &u32 {
        self.state.as_ref().unwrap().get_approvals(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take(){
            self.approvals = self.approvals + 1;
            if self.approvals >= 2 {
                println!("Post approved!");
                self.state = Some(s.approve())
            }
            else {
                println!("Post approved {} times!", self.approvals);
                self.state = Some(s.request_review())
            }
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.Draft())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box <dyn State>;
    
    fn approve(self: Box<Self>) -> Box <dyn State>;

    fn reject(self: Box<Self>) -> Box <dyn State>;
    
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }
    fn get_approvals<'a>(&self, post: &'a Post) -> &'a u32{
        &0
    }

}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box <dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box <dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box <dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box <dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box <dyn State> {
        Box::new(Rejected {})
    }

    fn get_approvals<'a>(&self, post: &'a Post) -> &'a u32{
        &post.approvals
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box <dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }
}

struct Rejected {}

impl State for Rejected {
    
    fn request_review(self: Box<Self>) -> Box <dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

