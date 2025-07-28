from doctest import debug
from flask import Flask

app = Flask(__name__)

@app.route('/')
def home() -> dict[str, str]:
    return {'message':'Welcome to your flask app'}


if __name__ == '__main__':
    app.run(debug=True)