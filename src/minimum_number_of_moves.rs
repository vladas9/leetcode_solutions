pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;
    let mut steps = 0;
    while let Some(_) = seats.iter().max() {
        let max1 = seats
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(index, _)| index)
            .unwrap();
        let max2 = students
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(index, _)| index)
            .unwrap();
        if seats[max1] > seats[max2] {
            steps += seats[max1] - students[max2];
        } else {
            steps += students[max2] - seats[max1];
        }
        seats.remove(max1);
        students.remove(max2);
    }
    steps
}
