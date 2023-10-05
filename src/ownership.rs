#[test]
pub fn fs_ownership() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::open("hello.txt")?;

    // Note 1: The Drop trait is implemented for File and will close the file
    // descriptor when the File falls out of scope.
    //
    // Note 2: Take a look at the implementation of the drop() function.

    // Forcibly drop the file value
    // drop(file);

    file.write_all(b"Hello, world!")?;

    Ok(())
}
