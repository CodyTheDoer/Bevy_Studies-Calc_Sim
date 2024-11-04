use std::io;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon/calc_icon.ico");
        res.compile()?;
    }
    Ok(())
}