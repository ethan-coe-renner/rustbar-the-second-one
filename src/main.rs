use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(6000);

    loop {
        let bar = rbtso::Bar(vec![
            rbtso::music(),
            rbtso::updates(),
            rbtso::tasks(),
            rbtso::news(),
	    rbtso::network(),
	    rbtso::volume(),
            rbtso::battery(),
        ]);
        println!("{}", bar);
        thread::sleep(delay);
    }
}

