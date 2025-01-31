pub trait Get {
    fn get(&self, title: &String) {
        println!("{} is being fetched", title);
    }
}
