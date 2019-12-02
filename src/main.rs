#[derive(Debug)]
struct Student {
    name: String,
    class: u8,
    
}
impl Student {
    //Making instance by Associated function
    pub fn create_student(name: String, class: u8, ) -> Student {
        Student {
            name: name,
            class: class,
        
        }
    }
    fn view_student(&self) -> String {
        let info_student = format!(
            "Name:{}\nclass:{}\n",
            self.name,self.class
        );
        info_student
    }
}

fn main() {
    let student_01 =
        Student::create_student("Sakina Hussain".to_string(), 9);
    // making instance directly in the main function
    println!("{:?}", student_01);
    // making instance by associated function
    println!("{}", student_01.view_student());
    // making instance by associated function
    let student_02 = 
    Student::create_student("Nafisa Hussain".to_string(), 8);
    
    println!("{}", student_02.view_student());

}
