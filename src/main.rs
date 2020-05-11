#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum RecordType{
	STRING,
	NUMBER,
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Record{
	column: String,
	record_type: RecordType,
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct SelectResult{
	headers: Vec<String>,
	records: Vec<Record>,
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct UpdateResult{
	message: String,
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum SQLResult{
	SRESULT(SelectResult),
	URESULT(UpdateResult),
}

fn sql_compile(text: &str) -> SQLResult {
	return SQLResult::URESULT(UpdateResult{message: "test".to_string()})
}


fn main() {

}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sql_compile_test(){
		assert_eq!(SQLResult::URESULT(UpdateResult{message: "test".to_string()}), sql_compile("hoge"));
	}
}
