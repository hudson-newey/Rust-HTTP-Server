use std::fs;

// read file function
// input: filename [0]
// output: returns contents of file as std::string::String
pub fn read_file(file_name: &str) -> std::string::String {
  const ROOT_FILE_NAME: &str = "static/index.html";

  if file_name == "/" {
    return fs::read_to_string(ROOT_FILE_NAME)
      .expect("Something went wrong reading the file");
  }

  let new_file_name = without_first_char(file_name);

  return fs::read_to_string(new_file_name)
    .expect("Something went wrong reading the file");
}

fn without_first_char(string: &str) -> &str {
  string
      .char_indices()
      .next()
      .and_then(|(i, _)| string.get(i + 1..))
      .unwrap_or("")
}
