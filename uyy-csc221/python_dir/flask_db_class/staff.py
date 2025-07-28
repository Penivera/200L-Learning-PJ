from flask import Flask,request,render_template
import sqlite3


@app.route('/')
def index():
    return render_template('form.html')

@app.route('/submit',methods=['POST']) # type: ignore
def submit():
    staff_name = request.form['staff_name']
    password = request.form['password']
    with sqlite3.connect("staff.db") as conn:
        cursor = conn.cursor()
        cursor.execute(
            '''
            INSERT INTO USERSTAFF(staff_name,password) VALUES(?,?)
            ''',(staff_name,password)
        )
        conn.commit()
    return {'message':'Data submitted succesfully'}

@app.route('/signup')
def signup():
    return render_template('classwork.html')
if __name__ == '__main__':
    app.run(debug=True)