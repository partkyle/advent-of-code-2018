pub mod lib {
    pub fn handle_frequency_line(line: Result<String, std::io::Error>) -> Result<i32, String> {
        let line = line.map_err(|e| e.to_string())?;
        i32::from_str_radix(&line[..], 10).map_err(|e| e.to_string())
    }
}
