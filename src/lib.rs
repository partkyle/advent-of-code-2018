pub mod lib {
    pub fn handle_frequency_line(line: Result<String, std::io::Error>) -> Result<i32, String> {
        let line = line.map_err(|e| e.to_string())?;
        let num = &line[1..].parse::<i32>().map_err(|e| e.to_string())?;
        match &line[0..1] {
            "+" => Ok(num.clone()),
            "-" => Ok(-num.clone()),
            _ => Err(format!("invalid frequency: {}", &line[0..1]))
        }
    }
}
