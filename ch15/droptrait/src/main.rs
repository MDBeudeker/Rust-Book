struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data {}!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    let e = CustomSmartPointer{
        data: String::from("kepok stuff"),
    };
    // e.drop(); // explicitly calling the destructor does not work
    drop(e); // you can manually call the drop function from the prelude
}
