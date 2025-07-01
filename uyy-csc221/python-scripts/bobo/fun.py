name=print("What is your name?")
name=input()
print(f"Hello {name}")
print("Do you wanna see what I can do?")
print("yes or no")
answer=input()
if answer == "yes":
	print("I am a calculation program that can only add, divide, multiply and subtract two numbers.")
	print("Type in the first number")
	a = float(input())
	print("Type in the second number")
	b = float(input())
	print("Do you want to add, subtract, divide or multiply them?", end="")
	operation = input()
	if operation == "add":
		result = a + b
	elif operation == "subtract":
		result = a - b
	elif operation == "multiply":
		result = a * b
	elif operation == "divide":
		result = a / b
  
	else:
		result = "Invalid operation"
	print("Here's your result:")
	print(result)
elif answer == "no":
	print("okay, bye😞")