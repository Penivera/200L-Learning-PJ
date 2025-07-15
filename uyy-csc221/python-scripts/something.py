print("Choose Arithmetic operstion you want to perform: ")
operation = input("1. Addition \n2. Subtraction\n3. Multiplication\n4. Division\n" )
operation = int(operation)
if operation == 1:
    first_num = input("Enter first Number: ")
    first_num = int(first_num)
    second_num = input("Enter first Number: ")
    second_num = int(second_num)
    print(first_num + second_num)
elif operation == 2:
     first_num = input("Enter first Number: ")
     first_num = int(first_num)
     second_num = input("Enter first Number: ")
     second_num = int(second_num)
     print(first_num - second_num)
elif operation == 3:
     first_num = input("Enter first Number: ")
     first_num = int(first_num)
     second_num = input("Enter first Number: ")
     second_num = int(second_num)
     print(first_num * second_num)
elif operation == 4:
     first_num = input("Enter first Number: ")
     first_num = int(first_num)
     second_num = input("Enter first Number: ")
     second_num = int(second_num)
     if second_num == 0 :
         print("Invalid input")
     else:
         print(first_num / second_num)
else:
    print("Invalid operation")
    
    
