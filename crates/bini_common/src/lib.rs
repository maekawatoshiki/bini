#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use elf::{endian::AnyEndian, ElfBytes};

    #[test]
    fn load_elf() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("hello");
        let file_data = fs::read(path).expect("Failed to read file");
        let slice = file_data.as_slice();
        let _file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Failed to parse elf");
    }
}
