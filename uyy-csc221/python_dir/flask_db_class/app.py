from flask import Flask,render_template,request
import sqlite3

app = Flask(__name__)
# Create table if not exists at startup
def init_db():
    with sqlite3.connect("classwork.db") as conn:
        cursor = conn.cursor()
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS Student(
                id INTEGER PRIMARY KEY,
                student_name TEXT,
                department TEXT,
                reg_no TEXT,
                password TEXT NOT NULL
            )
        ''')
        conn.commit()
init_db()

@app.route('/')
def home():
    return render_template('classwork.html')

@app.route('/submit',methods=['POST']) # type: ignore
def submit():
    student_name = request.form['student_name']
    reg_no = request.form['reg_no']
    department = request.form['dept']
    password = request.form['password']
    with sqlite3.connect("classwork.db") as conn:
        cursor = conn.cursor()
        cursor.execute(
            '''
            INSERT INTO Student(student_name,department,reg_no,password) VALUES(?,?,?,?)
            ''',(student_name,reg_no,department,password)
        )
        conn.commit()
    return {'message':'Data submitted successfully'}

if __name__ == '__main__':
    app.run(debug=True)