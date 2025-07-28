from flask import Flask, render_template, request
from flask_wtf import FlaskForm

app = Flask(__name__)

@app.route('/')
def student_score():
    return render_template('StudentScore.html')

@app.route('/result', methods=['POST', 'GET'])
def result():
    if request.method == 'POST':
        result_data = request.form  
        return render_template('result.html', result=result_data)
    return render_template('result.html', result={})  

if __name__ == '__main__':
    app.run(debug=True)
