mod ua;


#[cfg(test)]
mod tests {
    use crate::ua::UserAgent;
    use std::time::Instant;
    #[test]
    fn test() {
        let now = Instant::now();
        let uas = UserAgent::new();
        for _ in 0..10 {
            let ua = uas.random();
            println!("{}", ua);
        }
        println!("耗时: {} ms", now.elapsed().as_millis());
    }
}
