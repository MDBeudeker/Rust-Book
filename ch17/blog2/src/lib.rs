pub struct Post{
    content: String,
    approvals: u32,
}

pub struct DraftPost{
    content: String,
    approvals: u32,
}

impl Post{
    pub fn new() -> DraftPost{
        DraftPost {
            content: String::new(),
            approvals: 0,
        }
    }
    
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn approvals(&self) -> &u32 {
        &self.approvals
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost{
            content: self.content,
            approvals: self.approvals,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u32,
}

impl PendingReviewPost {
    pub fn approve(mut self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: self.approvals + 1,
        }
    }
    pub fn approvals(&self) -> &u32 {
        &self.approvals
    }
}
