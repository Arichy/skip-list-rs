use skiplist::SkipList;

fn main() {
    let mut skip_list = SkipList::new();

    skip_list.insert(3, 3);
    skip_list.insert(1, 1);
    skip_list.insert(2, 2);
    skip_list.insert(10, 10);
    skip_list.insert(-10, -10);

    {
        let mut into_iter = skip_list.into_iter();
        let a = into_iter.next();
        println!("{a:?}");
    }
}
