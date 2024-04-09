// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        if self.student_name=="Gary Plotter"{
            let grade=match self.grade {
                x if x>=1.0&& x<1.25=>"A+",
                x if x>=1.25&&x<1.5=> "A",
                x if x>=1.5&& x<1.75=>"A-",
                x if x>=1.75&&x<2.0=> "B+",
                x if x>=2.0&& x<2.25=>"B",
                x if x>=2.25&&x<2.5=> "B-",
                x if x>=2.5&& x<2.75=>"C+",
                x if x>=2.75&&x<3.0=> "C",
                x if x>=3.0&& x<3.25=>"C-",
                x if x>=3.25&&x<3.5=> "D+",
                x if x>=3.5&& x<3.75=>"D",
                x if x>=3.75&&x<4.0=> "D-",
                x if x>=4.0&& x<4.25=>"E+",
                x if x>=4.25&&x<4.5=> "E",
                x if x>=4.5&& x<4.75=>"E-",
                x if x>=4.75&&x<5.0=> "F+",
                x if x>=5.0&& x<5.25=>"F",
                x if x>=5.25&&x<5.5=> "F-",
                _=>panic!()
            };
            format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade)
        }else{
            format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 1.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
