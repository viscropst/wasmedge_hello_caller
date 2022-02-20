mod hello_caller;

fn main() {
    println!("---------host example caller start--------");
    host_example_caller::add_student("hello");
    host_example_caller::set_class_id(10);
    host_example_caller::print();
    host_example_caller::set_class_name("mine class");
    host_example_caller::add_student("我是谁");
    host_example_caller::print();
    host_example_caller::set_class_id(5);
    host_example_caller::set_class_name("other class");
    host_example_caller::add_student("小明");
    host_example_caller::print();
    println!("----------host example caller end-----------");
    println!("#########Starting Hello Caller########");
    println!("enter something to host");
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("type some text please");
    let back = hello_caller::host_prompt(text.as_str());
    println!("what's host returns back {}",back);
    println!("#########Starting Hello Caller########");
}
