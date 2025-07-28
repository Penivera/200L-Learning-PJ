from flask import Flask, render_template, flash

app = Flask(__name__)
app.secret_key = 'dev'  # Required for flash messages

@app.route('/')
def index():
    flash("Welcome to UYY-CSC221!")
    return render_template('index.html')

@app.route('/about')
def about():
    return render_template('about.html')

@app.route('/help')
def help():
    return render_template('help.html')

if __name__ == '__main__':
    app.run(debug=True)
