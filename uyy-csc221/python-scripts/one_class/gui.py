import tkinter

root = tkinter.Tk()

root.geometry("800x500")
root.title("One class")

def say_hello(root:tkinter.Tk):
    label = tkinter.Label(root,text="Hey you,who clicked me?")
    label.place(x=250,y=250)
tkinter.Label(root,text="Hello,Welcome to problem solving").place(x=350,y=150)
button = tkinter.Button(root,text="Click me",command=lambda: say_hello(root)).place(x=20,y=420) # type: ignore
quit_btn = tkinter.Button(root,text="Quit",command= root.destroy,).place(x=550,y=450)
   

root.mainloop()