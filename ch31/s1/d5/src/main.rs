extern crate pond;
use pond as pd;

fn main() {
    // test_new();
    test_new_threads();
}


fn test_new() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut pool = pd::Pool::new();

    pool.scoped( |sco| {
        let local_sco = sco.with_state(|| 11);

        for i in &mut v {
            local_sco.execute( 
                move |state| {
                    *i += *state;
                    println!("*i: {}; *state: {}", *i, *state);
                }
            );
        }
    });
}


fn test_new_threads() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut pool = pd::Pool::new_threads(5, 100);
    pool.scoped(|s| {
            let local_scope = s.with_state(|| 111);
            for i in &mut v {
                local_scope.execute( 
                    move |state| {
                        *i += *state;
                        println!("i {}; state {}", i, state);
                    }
                );
            }
        }
    );
}




