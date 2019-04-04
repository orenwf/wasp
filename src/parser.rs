#[allow(dead_code)]
fn fail(msg: String) {
    panic!(
        "The following code did not match any syntax: {:?}",
        msg
        )
}

