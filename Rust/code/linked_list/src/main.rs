use linked_list::SingleLinklist;
use linked_list::DoubleLinklist;

fn main() {
    // let list = Node {
    //     element: 1,
    //     next: None
    // };
    //
    // let list = Node {
    //     element: 1,
    //     next: Some(Box::new(Node {
    //         element: 2,
    //         next: None,
    //     })),
    // };
    //
    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: None,
    //     })
    // };
    //
    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: Some(Box::new(Node {
    //             element: 2,
    //             next: Some(Box::new(Node {
    //                 element: 3,
    //                 next: None,
    //             }))
    //         }))
    //     })
    // };
    //
    // let list = Linklist { head: None };

    // let list = Linklist {
    //     head: Some(Box::new(Node {
    //         element: 100,
    //         next: Some(Box::new(Node {
    //             element: 200,
    //             next: None,
    //         })),
    //     })),
    // };
    //
    // println!("{:?}", &list.head.unwrap().element);

    let mut list = SingleLinklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    println!("{:?}", list);
    println!("{}", list.remove().unwrap());
    list.print();

    let mut list1 = DoubleLinklist::new();
    list1.add(30);
    list1.add(32);
    list1.add(34);
    list1.add(36);
    println!("{:?}", list1);
    list1.print();
    list1.remove();
    list1.print();
}
