




use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};


pub const DEFAULT_PIPE_PATH: &str = "/tmp/ym2151_server.pipe";


pub struct NamedPipe {
    path: PathBuf,
}

impl NamedPipe {












    pub fn create() -> io::Result<Self> {
        Self::create_at(DEFAULT_PIPE_PATH)
    }
















    pub fn create_at<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();


        if path.exists() {
            fs::remove_file(&path)?;
        }


        mkfifo(
            &path,
            Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IROTH,
        )
        .map_err(io::Error::other)?;

        Ok(NamedPipe { path })
    }








    pub fn open_read(&self) -> io::Result<PipeReader> {
        let file = File::open(&self.path)?;
        Ok(PipeReader {
            reader: BufReader::new(file),
        })
    }








    pub fn open_write(&self) -> io::Result<PipeWriter> {
        let file = OpenOptions::new().write(true).open(&self.path)?;
        Ok(PipeWriter { file })
    }









    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<PipeWriter> {
        let file = OpenOptions::new().write(true).open(path)?;
        Ok(PipeWriter { file })
    }






    pub fn connect_default() -> io::Result<PipeWriter> {
        Self::connect(DEFAULT_PIPE_PATH)
    }


    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {

        let _ = fs::remove_file(&self.path);
    }
}


pub struct PipeReader {
    reader: BufReader<File>,
}

impl PipeReader {





    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
}


pub struct PipeWriter {
    file: File,
}

impl PipeWriter {








    pub fn write_str(&mut self, data: &str) -> io::Result<()> {
        self.file.write_all(data.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }






    pub fn read_response(&mut self) -> io::Result<String> {



        let mut buffer = String::new();
        let mut reader = BufReader::new(&self.file);
        reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_and_cleanup_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_create.pipe";


        let pipe = NamedPipe::create_at(test_path).unwrap();
        assert!(Path::new(test_path).exists());
        assert_eq!(pipe.path(), Path::new(test_path));


        drop(pipe);

        thread::sleep(Duration::from_millis(10));
        assert!(!Path::new(test_path).exists());
    }

    #[test]
    fn test_recreate_existing_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_recreate.pipe";


        let pipe1 = NamedPipe::create_at(test_path).unwrap();
        drop(pipe1);


        let pipe2 = NamedPipe::create_at(test_path).unwrap();
        assert!(Path::new(test_path).exists());

        drop(pipe2);
        thread::sleep(Duration::from_millis(10));
        assert!(!Path::new(test_path).exists());
    }

    #[test]
    fn test_write_read_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_rw.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();


        let write_path = test_path.to_string();
        let writer_thread = thread::spawn(move || {

            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&write_path).unwrap();
            writer.write_str("Hello, FIFO!\n").unwrap();
        });


        let mut reader = pipe.open_read().unwrap();
        let line = reader.read_line().unwrap();

        writer_thread.join().unwrap();

        assert_eq!(line, "Hello, FIFO!\n");

        drop(pipe);
        thread::sleep(Duration::from_millis(10));
    }

    #[test]
    fn test_multiple_messages() {
        let test_path = "/tmp/test_ym2151_pipe_multi.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();

        let write_path = test_path.to_string();
        let writer_thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&write_path).unwrap();
            writer.write_str("Message 1\n").unwrap();
            writer.write_str("Message 2\n").unwrap();
            writer.write_str("Message 3\n").unwrap();
        });

        let mut reader = pipe.open_read().unwrap();
        let line1 = reader.read_line().unwrap();
        let line2 = reader.read_line().unwrap();
        let line3 = reader.read_line().unwrap();

        writer_thread.join().unwrap();

        assert_eq!(line1, "Message 1\n");
        assert_eq!(line2, "Message 2\n");
        assert_eq!(line3, "Message 3\n");

        drop(pipe);
        thread::sleep(Duration::from_millis(10));
    }
}
