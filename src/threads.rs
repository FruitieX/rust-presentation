#[cfg(test)]
#[allow(unused_variables, unused_imports)]
mod tests {
    use anyhow::Result;
    use std::{thread, time::Duration};

    #[test]
    fn threads() -> Result<()> {
        let vec: Vec<u32> = Default::default();

        for i in 0..10 {
            thread::spawn(|| {
                // vec.push(i);
                // println!("{:?}", vec);
            });
        }

        Ok(())
    }
}
