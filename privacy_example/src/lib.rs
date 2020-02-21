pub mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    // o
    outermost::middle_secret_function();
    // x
    outermost::inside::inner_function();
    // x inside 가 pub 이 아니다.
    outermost::inside::secret_function();
    // x inside, secret_function 둘다 pub 이 아니다.
}
