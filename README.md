# 🦀 Rust Report Card Generator

This is a console-based Rust application built as part of **BlockseBlock Mini Task-3**. It allows users to enter student details, calculate average marks, assign grades, and generate a clean **PDF report card** using the `printpdf` crate.

---

## 📋 Features

✅ Takes input:  
- Student name  
- Total marks  
- Number of subjects  

✅ Computes:
- Average marks  
- Grade assignment (based on average):

| Grade | Criteria     |
|-------|--------------|
| A     | 90+          |
| B     | 75–89        |
| C     | 60–74        |
| D     | Below 60     |

✅ Generates:
- 📄 **PDF Report Card** (`report_card.pdf`)

✅ Prints a neat summary to the console

---

## ⚙️ How to Run

Make sure you have Rust installed. Then:

```bash
git clone https://github.com/parimal-art/blockseblock_task3.git
cd blockseblock_task3
cargo run
