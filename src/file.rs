enum FileState {
    Open(File),
    Read(File),
    Eof(File),
    Close(File),
}

struct File;

impl File {
    fn read(&mut self) -> Vec<u8> {
        todo!()
    }

    fn eof(&self) -> bool {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }
}

impl FileState {
    fn open() -> FileState {
        FileState::Open(File)
    }

    fn read(self) -> FileState {
        match self {
            FileState::Open(file) => FileState::Read(file),
            FileState::Read(file) => {
                if file.eof() {
                    FileState::Eof(file)
                } else {
                    FileState::Read(file)
                }
            }
            _ => self,
        }
    }

    fn close(self) -> FileState {
        match self {
            FileState::Read(file) | FileState::Eof(file) => FileState::Close(file),
            _ => self,
        }
    }
}

#[test]
fn test_file() {
    let mut data = Vec::new();
    let mut file_state = FileState::open();

    let file_state = loop {
        file_state = match file_state.read() {
            FileState::Read(mut file) => {
                data.extend(file.read());

                FileState::Read(file)
            }
            file_state => break file_state,
        };
    };

    match file_state.close() {
        FileState::Close(mut file) => file.close(),
        _ => eprintln!("file state is not valid"),
    }
}
