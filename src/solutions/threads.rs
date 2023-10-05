#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::sync::{Arc, RwLock};
    use std::{thread, time::Duration};

    #[test]
    fn threads() -> Result<()> {
        let vec: Arc<RwLock<Vec<u32>>> = Default::default();

        (0..10)
            .map(|i| {
                let vec = vec.clone();

                thread::spawn(move || {
                    let mut lock = vec.write().unwrap();
                    lock.push(i);
                    println!("{:?}", lock);
                    thread::sleep(Duration::from_millis(1));
                })
            })
            .map(|handle| handle.join().unwrap())
            .count();

        Ok(())
    }
}
