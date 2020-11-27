pub(crate) fn create_file_name(args: Vec<&str>) -> String {
    let mut file_name: String = args.iter().fold(String::new(), |s, a| s + a + "_");
    file_name.pop();
    file_name
}
