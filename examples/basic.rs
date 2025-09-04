use skiplist::SkipList;

fn main() {
    let mut skip_list = SkipList::new();

    skip_list.insert(3, 3);
    println!("{}", skip_list);
    skip_list.insert(1, 1);
    println!("{}", skip_list);
    skip_list.insert(2, 2);
    println!("{}", skip_list);

    skip_list.insert(10, 10);
    println!("{}", skip_list);
    skip_list.insert(-10, -10);
    println!("{}", skip_list);

    skip_list.remove(&10);
    println!("{}", skip_list);
    skip_list.remove(&2);
    println!("{}", skip_list);
    skip_list.remove(&-10);
    println!("{}", skip_list);
    skip_list.remove(&3);
    println!("{}", skip_list);
    skip_list.remove(&1);
    println!("{}", skip_list);
}
