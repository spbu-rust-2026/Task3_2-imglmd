pub fn add_command(history: &mut std::collections::VecDeque<String>, command: String, k: usize) {
    history.push_back(command);
    let mut len = history.len();

    while len > k {
        history.pop_front();
        len = len - 1;
    }
}
