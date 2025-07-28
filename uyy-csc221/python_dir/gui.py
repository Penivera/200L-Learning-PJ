import tkinter as tk



def start(root:tk.Tk):
    root.title("simple GUI Application")
    root.geometry('200x250')

def quit(root:tk.Tk):
    root.destroy()

root = tk.Tk()
root.title("simple GUI Application")
label = tk.Label(root,text="Hello Huys").place(x=20,y=30)
hellobtn = tk.Button(root,text="Click Me",command=lambda start(root)).place(x=50,y=3) # type: ignore
quitbtn = tk.Button(root,text="Quit",command=quit).place(x=30,y=40) # type: ignore
root.mainloop()