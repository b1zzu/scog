//type ControllerResult<T> = Result<T, String>;

//type BoxFn<A, E> = Box<Fn(A) -> Result<A, E>>;
//type Method<A, E> =  fn(A) -> Result<A, E>;

pub fn step<'a, A: 'a, E: 'a>(method: fn(A) -> Result<A, E>, next: Box<'a + Fn(A) -> Result<A, E>>) -> Box<'a + Fn(A) -> Result<A, E>> {
    Box::new(move |a: A| -> Result<A, E> {
        match method(a) {
            Ok(a) => next(a),
            Err(e) => Err(e),
        }
    })
}

pub fn end<'a, A: 'a, E: 'a>(method: fn(A) -> Result<A, E>) -> Box<'a + Fn(A) -> Result<A, E>> {
    Box::new(move |a: A| -> Result<A, E> {
        method(a)
    })
}

pub fn cluster<'a, A: 'a, E: 'a>(mut methods: Vec<fn(A) -> Result<A, E>>) -> Box<'a + Fn(A) -> Result<A, E>> {
    let mut next = end(methods.pop().unwrap());
    while let Some(method) = methods.pop() {
        next = step(method, next);
    }
    next
}