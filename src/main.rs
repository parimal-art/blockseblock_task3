use std::fs::File;
use std::io::{self, BufWriter};
use printpdf::*;

fn calculate_average(total_marks: f64, subjects: f64) -> f64 {
    total_marks / subjects
}

fn assign_grade(avg: f64) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn generate_pdf(name: &str, total: f64, subjects: f64, avg: f64, grade: &str) {
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    current_layer.use_text("Student Report Card", 24.0, Mm(50.0), Mm(250.0), &font);
    current_layer.use_text(format!("Name: {}", name), 14.0, Mm(20.0), Mm(220.0), &font);
    current_layer.use_text(format!("Total Marks: {}", total), 14.0, Mm(20.0), Mm(200.0), &font);
    current_layer.use_text(format!("Subjects: {}", subjects), 14.0, Mm(20.0), Mm(180.0), &font);
    current_layer.use_text(format!("Average: {:.2}", avg), 14.0, Mm(20.0), Mm(160.0), &font);
    current_layer.use_text(format!("Grade: {}", grade), 14.0, Mm(20.0), Mm(140.0), &font);

    let output = File::create("report_card.pdf").unwrap();
    let mut buf = BufWriter::new(output);
    doc.save(&mut buf).unwrap();
}

fn main() {
    let mut name = String::new();
    let mut total = String::new();
    let mut subjects = String::new();

    println!("Enter Student Name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter Total Marks:");
    io::stdin().read_line(&mut total).unwrap();

    println!("Enter Number of Subjects:");
    io::stdin().read_line(&mut subjects).unwrap();

    let total: f64 = total.trim().parse().unwrap();
    let subjects: f64 = subjects.trim().parse().unwrap();

    let avg = calculate_average(total, subjects);
    let grade = assign_grade(avg);

    println!("\n--- Report Card ---");
    println!("Name: {}", name.trim());
    println!("Total Marks: {}", total);
    println!("Subjects: {}", subjects);
    println!("Average: {:.2}", avg);
    println!("Grade: {}", grade);

    generate_pdf(name.trim(), total, subjects, avg, grade);
}
