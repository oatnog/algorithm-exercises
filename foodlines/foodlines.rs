// foodlines.rs -- solve Algorithmic Thinking intro problem

// of the lines in array 'lines', which is shortest?
fn shortest_line_index( lines: &Vec<i32>) -> usize {
	let mut shortest = 0;
	for j in 0..lines.len() {
		if lines[j] < lines[shortest] {
			shortest = j;
		}
	}
	//println!("shortest line index = {}", shortest);

	return shortest;
}


fn main() {

	// assign default values to test. TODO: read from stdin
	let mut lines = vec![3, 2, 5]; // lines with people in them
	let m = 4; // number of arriving new people


	let mut shortest: usize; // index of shortest line
	for _i in 0..m {
		shortest = shortest_line_index(&lines);
		println!("{}", lines[shortest]);
		lines[shortest] = lines[shortest] + 1;
	}
	
} 
