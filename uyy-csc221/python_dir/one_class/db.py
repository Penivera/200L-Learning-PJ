import sqlite3
import tkinter
from tkinter import *

conn = sqlite3.connect("testDb.db")
cursor = conn.cursor()

cursor.execute("CREATE TABLE IF NOT EXISTS user_table(Id,Name,Password)")

cursor.execute("INSERT INTO user_table VALUES(?,?,?)", (1,"peniel","password"))
conn.commit()
conn.close()