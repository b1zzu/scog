//type ControllerResult<T> = Result<T, String>;

pub fn ver<'a, A: 'a, E: 'a>(method: fn(A) -> Result<A, E>, next: Box<Fn(A) -> Result<A, E>>) -> Box<Fn(A) -> Result<A, E>> {
    Box::new(move |a: A| -> Result<A, E> {
        match method(a) {
            Ok(a) => next(a),
            Err(e) => Err(e),
        }
    })
}

pub fn with<'a, A: 'a, E: 'static, T: 'static>(method: fn(&A) -> Result<T, E>, next: Box<Fn(A, T) -> Result<A, E>>) -> Box<Fn(A) -> Result<A, E>> {
    Box::new(move |a: A| -> Result<A, E> {
        match method(&a) {
            Ok(t) => next(a, t),
            Err(e) => Err(e),
        }
    })
}
